use bevy::prelude::Vec2;

/// 辺の方向として扱えるもの
pub trait ToEdgeDirection {
    fn to_edge_direction(&self) -> EdgeDirection;
}
/// 四角形ブロックに取り付けるときの辺の方向
#[derive(Debug, Clone, Copy)]
pub enum EdgeDirection {
    Up,
    Down,
    Left,
    Right,
}

impl ToEdgeDirection for EdgeDirection {
    fn to_edge_direction(&self) -> EdgeDirection {
        *self
    }
}
impl ToEdgeDirection for u32 {
    fn to_edge_direction(&self) -> EdgeDirection {
        match self {
            0 => EdgeDirection::Up,
            1 => EdgeDirection::Down,
            2 => EdgeDirection::Left,
            _ => EdgeDirection::Right,
        }
    }
}

// 矩形中心から辺に向かう単位ベクトルに変換できるようにする
impl From<EdgeDirection> for Vec2 {
    fn from(dir: EdgeDirection) -> Self {
        match dir {
            EdgeDirection::Up => Vec2::Y,
            EdgeDirection::Down => -Vec2::Y,
            EdgeDirection::Left => Vec2::X,
            EdgeDirection::Right => -Vec2::X,
        }
    }
}
