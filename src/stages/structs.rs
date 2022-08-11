use bevy::prelude::*;

use crate::components::{
    ball::{BallType, SetBallEvent},
    block::{BlockType, SpawnBlockEvent},
    goal::SpawnGoalEvent,
    launcher::SpawnLauncherEvent,
};

pub struct StageInfo {
    /// 制限時間（フレーム数）
    pub time: u32,
    /// ランチャー情報（回転可能角度の上限と下限）
    pub launcher: LauncherInfo,
    /// 配置ブロックリスト
    pub blocks: Vec<BlockInfo>,
    /// 使用可能なボールリスト
    pub balls: Vec<BallInfo>,
    /// ゴール情報
    pub goal_pos: Vec<GoalInfo>,
}

pub struct LauncherInfo {
    pub pos: Vec2,
    pub rotate_speed: f32,
    pub min_angle: f32,
    pub max_angle: f32,
}
impl LauncherInfo {
    pub fn to_spawn_event(&self) -> SpawnLauncherEvent {
        SpawnLauncherEvent {
            pos: self.pos,
            rotate_speed: self.rotate_speed,
            min_angle: self.min_angle,
            max_angle: self.max_angle,
        }
    }
}

/// ブロック一つのの情報
pub struct BlockInfo {
    pub block_type: BlockType,
    pub default_angle: f32,     // 初期角度
    pub default_pos_param: f32, // 初期位置パラメータ
}
impl BlockInfo {
    pub fn to_spawn_event(&self) -> SpawnBlockEvent {
        SpawnBlockEvent::from_type(
            self.block_type.clone(),
            self.default_angle,
            self.default_pos_param,
        )
    }
}

/// ボールひとつの情報
pub struct BallInfo {
    pub ball_type: BallType,
}
impl BallInfo {
    pub fn from_type(ball_type: BallType) -> Self {
        BallInfo { ball_type }
    }
    pub fn to_spawn_event(&self) -> SetBallEvent {
        SetBallEvent {
            ball_type: self.ball_type,
        }
    }
}
pub trait ArrangeBallInfo {
    fn set_balls(&mut self, ball_type: BallType, num: u32) -> &mut Self;
}
impl ArrangeBallInfo for Vec<BallInfo> {
    fn set_balls(&mut self, ball_type: BallType, num: u32) -> &mut Self {
        for _ in 0..num {
            self.push(BallInfo::from_type(ball_type))
        }
        self
    }
}

pub struct GoalInfo {
    pub pos: Vec2,
    pub radius: f32,
    pub score: u32,
}
impl GoalInfo {
    pub fn to_spawn_event(&self) -> SpawnGoalEvent {
        SpawnGoalEvent {
            pos: self.pos,
            radius: self.radius,
            score: self.score,
        }
    }
}
