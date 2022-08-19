//! ステージを定義する構造体を定義するモジュール.
use crate::systems::field::FIELD_WIDTH;
use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use crate::components::physics::force::Gravity;
use crate::events::ball::SetBallEvent;
use crate::events::goal::SpawnGoalEvent;
use crate::events::launcher::SpawnLauncherEvent;
use crate::events::switch::SpawnSwitchEvent;
use crate::{
    components::{
        ball::BallType,
        block::{RotateStrategy, SlideStrategy},
        block_attach::{switch::SwitchTile, BlockAttachment},
        physics::material::PhysicMaterial,
    },
    events::ToSpawnEvent,
};

#[derive(Clone)]
pub struct StageInfo {
    pub stage_title: &'static str,
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
    /// スイッチの送信機の情報
    pub switches: Vec<SwitchInfo>,
    /// 重力場
    pub gravity: Option<Gravity>,
}

#[derive(Clone, Debug)]
pub struct LauncherInfo {
    pub pos: Vec2,
    pub default_angle: f32,
    pub rotate_speed: f32,
    pub min_angle: f32,
    pub max_angle: f32,
}
impl Default for LauncherInfo {
    fn default() -> Self {
        LauncherInfo {
            pos: Vec2::new(-FIELD_WIDTH / 2.0 + 30.0, 0.0),
            default_angle: 0.0,
            rotate_speed: 0.02,
            min_angle: -FRAC_PI_2,
            max_angle: FRAC_PI_2,
        }
    }
}
impl ToSpawnEvent for LauncherInfo {
    type E = SpawnLauncherEvent;
    fn to_spawn_event(&self) -> Self::E {
        SpawnLauncherEvent {
            pos: self.pos,
            default_angle: self.default_angle,
            rotate_speed: self.rotate_speed,
            min_angle: self.min_angle,
            max_angle: self.max_angle,
        }
    }
}

/// ブロック情報からブロックタイプコンポーネントを作成し, さらに出現イベントを作成する
#[derive(Clone, Debug)]
pub enum BlockShapeInfo {
    Wall {
        extents: Vec2,
    },
    Rect {
        extents: Vec2,     // xyの大きさ
        rect_origin: Vec2, // 矩形内の位置
        rotate_strategy: RotateStrategy,
        slide_strategy: SlideStrategy,
    },
    Ellipse {
        radii: Vec2, // x半径とy半径
        center: Vec2,
        rotate_strategy: RotateStrategy,
        slide_strategy: SlideStrategy,
    },
}
impl Default for BlockShapeInfo {
    fn default() -> Self {
        BlockShapeInfo::Rect {
            extents: Vec2::ONE * 50.0,
            rect_origin: Vec2::ZERO,
            rotate_strategy: RotateStrategy::NoRotate,
            slide_strategy: SlideStrategy::NoSlide,
        }
    }
}

/// ブロック一つの情報
#[derive(Clone, Default, Debug)]
pub struct BlockInfo {
    pub pos: Vec2,
    pub block_shape_info: BlockShapeInfo,
    pub material: PhysicMaterial,
    pub default_angle: f32,                     // 初期角度
    pub default_pos_param: f32,                 // 初期位置パラメータ
    pub block_attachment: Vec<BlockAttachment>, // ブロックにくっつけるもの
}

/// ボールひとつの情報
#[derive(Clone)]
pub struct BallInfo {
    pub ball_type: BallType,
}
impl BallInfo {
    pub fn from_type(ball_type: BallType) -> Self {
        BallInfo { ball_type }
    }
}
impl ToSpawnEvent for BallInfo {
    type E = SetBallEvent;
    fn to_spawn_event(&self) -> Self::E {
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

#[derive(Clone)]
pub struct GoalInfo {
    pub pos: Vec2,
    pub radius: f32,
    pub score: u32,
}
impl ToSpawnEvent for GoalInfo {
    type E = SpawnGoalEvent;
    fn to_spawn_event(&self) -> Self::E {
        SpawnGoalEvent {
            pos: self.pos,
            radius: self.radius,
            score: self.score,
        }
    }
}
impl Default for GoalInfo {
    fn default() -> Self {
        GoalInfo {
            pos: Vec2::ZERO,
            radius: 30.0,
            score: 1,
        }
    }
}

#[derive(Clone)]
pub struct SwitchInfo {
    pub default_active: bool,
    /// 発動する重さ制限（必ず発動するなら0にすればよい）
    pub threshold: f32,
    /// 押された後自動で解除されるかどうか（フレーム数のOptionで指定）
    pub auto_reverse: Option<u32>,
    /// SwitchTargetに付けた整数を指定して効果対象を限定する.
    pub target_id: u32,
    pub extents: Vec2,
    pub pos: Vec2,
}
impl Default for SwitchInfo {
    fn default() -> Self {
        Self {
            default_active: false,
            threshold: 0.0,
            auto_reverse: None,
            target_id: 0,
            extents: Vec2::splat(40.0),
            pos: Vec2::ZERO,
        }
    }
}
impl ToSpawnEvent for SwitchInfo {
    type E = SpawnSwitchEvent;
    fn to_spawn_event(&self) -> Self::E {
        SpawnSwitchEvent {
            component: SwitchTile {
                just_active: self.default_active,
                active: self.default_active,
                threshold: self.threshold,
                auto_reverse: self.auto_reverse,
                target_id: self.target_id,
                extents: self.extents,
            },
            pos: self.pos,
        }
    }
}
