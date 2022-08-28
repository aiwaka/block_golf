use std::f32::consts::{FRAC_2_PI, FRAC_PI_2};

use bevy::prelude::*;
use bevy_prototype_lyon::shapes;

#[derive(Component, Clone, Copy, Default, Debug)]
pub struct Block;

/// ブロックのすべての位置補正がかかっていない状態のブロック中心の位置
#[derive(Component, Clone, Copy, Default, Debug)]
pub struct BlockOriginalPos(pub Vec2);

/// ブロック回転軸からブロック中心位置の位置を相対座標で指定する
#[derive(Component, Clone, Copy, Default, Debug)]
pub struct BlockAxisPos(pub Vec2);

/// 位置を計算するためのパラメータ. Manualの場合[-1, 1]をとるとする.
#[derive(Component, Default)]
pub struct BlockSlideParam(pub f32);

// TODO: これをBlockPosOffsetとBlockAngleというコンポーネントに分解し,
//       様々なオフセット（軌道やスイッチの影響等）ごとに個別に登録するように変更する.
//       個別のコンポーネントをイテレートし, それらの合算を描画に反映する.
/// ブロックの位置や角度の情報を持っておくコンポーネント
#[derive(Component, Default)]
pub struct BlockTransformInfo {
    pub angle: f32,   // 現在の角度
    pub offset: Vec2, // 位置補正
}

impl BlockTransformInfo {
    pub fn new(angle: f32, offset: Vec2) -> Self {
        Self { angle, offset }
    }
    // /// デフォルト角度と位置パラメータから新規生成
    // pub fn new(angle: f32, pos_param: f32) -> Self {
    //     Self {
    //         angle,
    //         offset: Vec2::ZERO,
    //         pos_param,
    //     }
    // }
    // /// そのフレームでの重心の並進速度
    // /// delta: 重心 - 回転軸 のベクトル（Rectならoriginでよい）
    // pub fn pos_diff(&self, path: &BlockSlidePath, delta: Vec2) -> Vec2 {
    //     let current_pos = path.calc_orbit(self.pos_param)
    //         + delta * Vec2::new(self.angle.cos(), self.angle.sin())
    //         + self.offset;
    //     let prev_pos = path.calc_orbit(self.prev_param)
    //         + delta * Vec2::new(self.prev_angle.cos(), self.prev_angle.sin())
    //         + self.prev_offset;
    //     current_pos - prev_pos
    // }
}

/// 回転の方法
#[derive(Component, Clone, Debug)]
pub enum RotateStrategy {
    NoRotate,
    /// 回転量, 下限, 上限
    Manual(f32, f32, f32),
    Auto(f32),
}
impl Default for RotateStrategy {
    fn default() -> Self {
        RotateStrategy::NoRotate
    }
}
impl RotateStrategy {
    pub fn manual(amount: f32, min: f32, max: f32) -> Self {
        Self::Manual(amount, min, max)
    }
    pub fn infinite_manual(amount: f32) -> Self {
        Self::Manual(amount, f32::MIN, f32::MAX)
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
    // StandardLineを軌道とするManualタイプを作成
    pub fn simple_manual_slider(speed: f32, theta: f32, width: f32) -> Self {
        SlideStrategy::Manual {
            speed,
            path: BlockSlidePath::StandardLine { theta, width },
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
