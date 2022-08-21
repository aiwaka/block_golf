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
