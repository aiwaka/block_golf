use std::f32::consts::{FRAC_2_PI, FRAC_PI_2};

use bevy::prelude::*;
use bevy_prototype_lyon::shapes;

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
    pub offset: Vec2,   // 位置補正
    pub pos_param: f32, // 位置を計算するためのパラメータ. Manualの場合[-1, 1]をとるとする.
    /// 直前フレームの位置データを保持して差分を取れるようにする.
    pub prev_angle: f32,
    pub prev_offset: Vec2,
    pub prev_param: f32,
}
impl BlockTransform {
    /// デフォルト角度と位置パラメータから新規生成
    pub fn new(angle: f32, pos_param: f32) -> Self {
        Self {
            angle,
            offset: Vec2::ZERO,
            pos_param,
            prev_angle: angle,
            prev_offset: Vec2::ZERO,
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
impl Default for RotateStrategy {
    fn default() -> Self {
        RotateStrategy::NoRotate
    }
}

/// 移動の方法
#[derive(Component, Clone, Debug)]
pub enum SlideStrategy {
    NoSlide,
    Manual { speed: f32, path: BlockSlidePath }, // キー入力で移動
    AutoWrap { speed: f32, path: BlockSlidePath }, // キー入力で移動, 自動で折り返し
    Auto { speed: f32, path: BlockSlidePath },   // 自動で移動
}
impl Default for SlideStrategy {
    fn default() -> Self {
        SlideStrategy::NoSlide
    }
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
    Wall { shape: shapes::Rectangle },
    Rect { shape: shapes::Rectangle },
    Ellipse { shape: shapes::Ellipse },
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
