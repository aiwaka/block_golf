use bevy::prelude::*;

#[derive(Component)]
pub struct Block;

/// 回転の方法
#[derive(Component, Clone)]
pub enum RotateStrategy {
    CannotRotate,
    Rotatable(f32),
    Always(f32),
}

/// ブロックのタイプ. 矩形, 円形, 中空等
pub enum BlockType {
    Rect {
        pos: Vec2,         // 位置
        extents: Vec2,     // xyの大きさ
        rect_origin: Vec2, // 矩形内の位置
        strategy: RotateStrategy,
    },
}
// タイプのデフォルトカラーを決めておく
impl From<&BlockType> for Color {
    fn from(t: &BlockType) -> Self {
        match *t {
            BlockType::Rect {
                pos: _,
                extents: _,
                rect_origin: _,
                strategy: _,
            } => Color::CYAN,
        }
    }
}

/// タイプと色を指定
pub struct SpawnBlockEvent {
    pub block_type: BlockType,
    pub color: Color,
    pub default_angle: f32,
}

impl SpawnBlockEvent {
    pub fn from_type(block_type: BlockType, default_angle: f32) -> Self {
        let color = Color::from(&block_type);
        SpawnBlockEvent {
            block_type,
            color,
            default_angle,
        }
    }
}
