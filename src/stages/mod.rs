// use heron::PhysicMaterial;

use crate::components::{
    ball::{BallType, SetBallEvent},
    block::SpawnBlockEvent,
    goal::SpawnGoalEvent,
};

pub mod debug;
mod field_blocks;
pub mod sample;

pub struct StageInfo {
    pub blocks: Vec<SpawnBlockEvent>,
    pub balls: Vec<SetBallEvent>,
    pub goal_pos: Vec<SpawnGoalEvent>,
}

// struct StageInfo {
//     blocks: Vec<Block>, // 様々なブロックの配置
//     floors: Vec<Floor>, // 様々な床の配置
//     hole_pos: Vec2,     // ゴールの位置
//     ball_pos: Vec2,     // ボール初期位置
// }

// struct Block {
//     operatable: bool,                 // プレイヤーが操作可能かどうか
//     color: Color,                     // ブロックの色
//     rot_axis: Vec2,                   // 回転軸位置（中心を原点とする）
//     position: Vec2,                   // 位置（回転軸からの相対位置とする）
//     material: Option<PhysicMaterial>, // 材質
//     ini_angle: f32,                   // 初期角度
//     rotate_speed: f32, // 回転速度（operatableなら反応の鋭敏さ, notなら自動の回転速度）
// }

// struct Floor {}
