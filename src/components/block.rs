use std::f32::consts::{FRAC_2_PI, FRAC_PI_2};

use bevy::prelude::*;
use bevy_prototype_lyon::shapes::Rectangle;

/// ブロックであることを示す. これを使って衝突判定を行う
#[derive(Component)]
pub struct Block;
// Rectangleをあとで使うためのコンポーネント
#[derive(Component)]
pub struct RectangleBlock {
    pub original_pos: Vec2, // 軌道パラメータが0のときの座標
    pub rect: Rectangle,
    pub angle: f32,
    pub pos_param: f32, // 位置を計算するためのパラメータ. Manualの場合[-1, 1]をとるとする.
    /// 直前フレームの位置データを保持して差分を取れるようにする.
    pub prev_angle: f32,
    pub prev_param: f32,
}
impl RectangleBlock {
    /// そのフレームでの重心周りの角速度
    pub fn angle_diff(&self) -> f32 {
        self.angle - self.prev_angle
    }
    /// そのフレームでの重心の並進速度
    pub fn pos_diff(&self, path: &BlockSlidePath) -> Vec2 {
        path.calc_orbit(self.pos_param) - path.calc_orbit(self.prev_param)
    }
}

/// 回転の方法
#[derive(Component, Clone)]
pub enum RotateStrategy {
    NoRotate,
    Manual(f32),
    Auto(f32),
}

/// 移動の方法
#[derive(Component, Clone)]
pub enum SlideStrategy {
    NoSlide,
    Manual { speed: f32, path: BlockSlidePath }, // キー入力で移動
    AutoWrap { speed: f32, path: BlockSlidePath }, // キー入力で移動, 自動で折り返し
    Auto { speed: f32, path: BlockSlidePath },   // 自動で移動
}

#[derive(Clone)]
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
pub enum BlockType {
    Wall {
        pos: Vec2,
        extents: Vec2,
        weight: f32,      // 質量
        friction: f32,    // 摩擦係数
        restitution: f32, // 反発係数
    },
    Rect {
        pos: Vec2,         // 位置
        extents: Vec2,     // xyの大きさ
        rect_origin: Vec2, // 矩形内の位置
        rotate_strategy: RotateStrategy,
        slide_strategy: SlideStrategy,
        weight: f32,      // 質量
        friction: f32,    // 摩擦係数
        restitution: f32, // 反発係数
    },
}
// タイプのデフォルトカラーを決めておく
impl From<&BlockType> for Color {
    fn from(t: &BlockType) -> Self {
        match *t {
            BlockType::Wall {
                pos: _,
                extents: _,
                weight: _,
                friction: _,
                restitution: _,
            } => Color::BLACK,
            BlockType::Rect {
                pos: _,
                extents: _,
                rect_origin: _,
                rotate_strategy: _,
                slide_strategy: _,
                weight: _,
                friction: _,
                restitution: _,
            } => Color::CYAN,
        }
    }
}

/// タイプと色を指定
pub struct SpawnBlockEvent {
    pub block_type: BlockType,
    pub color: Color,
    pub default_angle: f32,
    pub default_pos_param: f32,
}

impl SpawnBlockEvent {
    pub fn from_type(block_type: BlockType, default_angle: f32, default_pos_param: f32) -> Self {
        let color = Color::from(&block_type);
        SpawnBlockEvent {
            block_type,
            color,
            default_angle,
            default_pos_param,
        }
    }
}
