use std::f32::consts::{FRAC_2_PI, FRAC_PI_2};

use bevy::prelude::*;
use bevy_prototype_lyon::{
    prelude::RectangleOrigin,
    shapes::{Ellipse, Rectangle},
};

use crate::stages::structs::{BlockInfo, BlockShapeInfo};

use super::{block_attach::BlockAttachment, physics::material::PhysicMaterial};

/// ブロックであることを示す. これを使って衝突判定を行う
#[derive(Component)]
pub struct Block;

/// ブロックの位置パラメータが0のときの位置
#[derive(Component)]
pub struct BlockOriginalPos(pub Vec2);

/// ブロックの位置や角度の情報を持っておくコンポーネント
#[derive(Component, Default)]
pub struct BlockTransform {
    pub angle: f32,     // 現在の角度
    pub pos_param: f32, // 位置を計算するためのパラメータ. Manualの場合[-1, 1]をとるとする.
    /// 直前フレームの位置データを保持して差分を取れるようにする.
    pub prev_angle: f32,
    pub prev_param: f32,
}
impl BlockTransform {
    /// デフォルト角度と位置パラメータから新規生成
    pub fn new(angle: f32, pos_param: f32) -> Self {
        Self {
            angle,
            pos_param,
            prev_angle: angle,
            prev_param: pos_param,
        }
    }
    /// そのフレームでの重心周りの角速度
    pub fn angle_diff(&self) -> f32 {
        self.angle - self.prev_angle
    }
    /// そのフレームでの重心の並進速度
    /// delta: 重心 - 回転軸 のベクトル（Rectならoriginでよい）
    pub fn pos_diff(&self, path: &BlockSlidePath, delta: Vec2) -> Vec2 {
        let current_pos =
            path.calc_orbit(self.pos_param) + delta * Vec2::new(self.angle.cos(), self.angle.sin());
        let prev_pos = path.calc_orbit(self.prev_param)
            + delta * Vec2::new(self.prev_angle.cos(), self.prev_angle.sin());
        current_pos - prev_pos
    }
}

/// 回転の方法
#[derive(Component, Clone, Debug)]
pub enum RotateStrategy {
    NoRotate,
    Manual(f32),
    Auto(f32),
}

/// 移動の方法
#[derive(Component, Clone, Debug)]
pub enum SlideStrategy {
    NoSlide,
    Manual { speed: f32, path: BlockSlidePath }, // キー入力で移動
    AutoWrap { speed: f32, path: BlockSlidePath }, // キー入力で移動, 自動で折り返し
    Auto { speed: f32, path: BlockSlidePath },   // 自動で移動
}

impl SlideStrategy {
    pub fn get_path(&self) -> BlockSlidePath {
        match self {
            SlideStrategy::NoSlide => BlockSlidePath::NoPath,
            SlideStrategy::Manual { speed: _, path } => path.clone(),
            SlideStrategy::AutoWrap { speed: _, path } => path.clone(),
            SlideStrategy::Auto { speed: _, path } => path.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum BlockSlidePath {
    NoPath,
    StandardLine { theta: f32, width: f32 }, // X軸からの角度を引数に取る
}
/// [-1,1]の三角波の周期関数
fn periodic_param(param: f32) -> f32 {
    FRAC_2_PI * (param * FRAC_PI_2).sin().asin()
}
impl BlockSlidePath {
    // 定義された軌道を実際に計算する.
    // paramからVec2を返す. ブロックの中心を原点とする相対的なものにする.
    // autowrapに対応して[-1, 1]を定義域とする関数の周期関数であると定める.
    // manualでしか使わないのであればそうでなくてもよいがコンパイルの時点では制限されない.
    pub fn calc_orbit(&self, param: f32) -> Vec2 {
        match *self {
            BlockSlidePath::NoPath => Vec2::ZERO,
            BlockSlidePath::StandardLine { theta, width } => {
                Vec2::new(theta.cos(), theta.sin()) * width * periodic_param(param)
            }
        }
    }
}

/// ブロックのタイプ. 矩形, 円形, 中空等
/// shapeを保持する
#[derive(Component, Clone)]
pub enum BlockType {
    Wall { shape: Rectangle },
    Rect { shape: Rectangle },
    Ellipse { shape: Ellipse },
}
// タイプのデフォルトカラーを決めておく
impl From<&BlockType> for Color {
    fn from(t: &BlockType) -> Self {
        match *t {
            BlockType::Wall { shape: _ } => Color::BLACK,
            BlockType::Rect { shape: _ } => Color::CYAN,
            BlockType::Ellipse { shape: _ } => Color::PINK,
        }
    }
}

/// タイプと色を指定
pub struct SpawnBlockEvent {
    pub pos: Vec2,
    pub block_type: BlockType,
    pub material: PhysicMaterial,
    pub default_angle: f32,
    pub default_pos_param: f32,
    pub rotate_strategy: RotateStrategy,
    pub slide_strategy: SlideStrategy,
    pub block_attachment: Vec<BlockAttachment>,
}

impl From<&BlockInfo> for SpawnBlockEvent {
    fn from(block_info: &BlockInfo) -> Self {
        match &block_info.block_shape_info {
            BlockShapeInfo::Wall { extents } => {
                let block_type = BlockType::Wall {
                    shape: Rectangle {
                        extents: *extents,
                        origin: RectangleOrigin::CustomCenter(Vec2::ZERO),
                    },
                };
                SpawnBlockEvent {
                    pos: block_info.pos,
                    block_type,
                    material: block_info.material,
                    default_angle: block_info.default_angle,
                    default_pos_param: block_info.default_pos_param,
                    rotate_strategy: RotateStrategy::NoRotate,
                    slide_strategy: SlideStrategy::NoSlide,
                    block_attachment: block_info.block_attachment.clone(),
                }
            }
            BlockShapeInfo::Rect {
                extents,
                rect_origin,
                rotate_strategy,
                slide_strategy,
            } => {
                let block_type = BlockType::Rect {
                    shape: Rectangle {
                        extents: *extents,
                        origin: RectangleOrigin::CustomCenter(*rect_origin),
                    },
                };
                SpawnBlockEvent {
                    pos: block_info.pos,
                    block_type,
                    material: block_info.material,
                    default_angle: block_info.default_angle,
                    default_pos_param: block_info.default_pos_param,
                    rotate_strategy: rotate_strategy.clone(),
                    slide_strategy: slide_strategy.clone(),
                    block_attachment: block_info.block_attachment.clone(),
                }
            }
            BlockShapeInfo::Ellipse {
                radii,
                center,
                rotate_strategy,
                slide_strategy,
            } => {
                let block_type = BlockType::Ellipse {
                    shape: Ellipse {
                        radii: *radii,
                        center: *center,
                    },
                };
                SpawnBlockEvent {
                    pos: block_info.pos,
                    block_type,
                    material: block_info.material,
                    default_angle: block_info.default_angle,
                    default_pos_param: block_info.default_pos_param,
                    rotate_strategy: rotate_strategy.clone(),
                    slide_strategy: slide_strategy.clone(),
                    block_attachment: block_info.block_attachment.clone(),
                }
            }
        }
    }
}
