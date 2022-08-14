use bevy::prelude::*;
use bevy_prototype_lyon::{
    prelude::{DrawMode, FillMode, GeometryBuilder},
    shapes::{Rectangle, RectangleOrigin},
};

use crate::components::{
    ball::Ball,
    block::{BlockOriginalPos, BlockTransform, BlockType},
    block_attach::fan::{Fan, FanDirection},
    physics::position::Position,
};

/// ブロック出現時に送風機のポリゴンを描画するときに使う関数
pub fn spawn_fan(commands: &mut Commands, block_ent: Entity, rect: &Rectangle, fan: &Fan) {
    let (fan_extents, fan_pos) = match fan.direction {
        FanDirection::Up => (
            rect.extents.project_onto(Vec2::X) + Vec2::Y * 10.0,
            rect.extents.project_onto(Vec2::Y) / 2.0,
        ),
        FanDirection::Down => (
            rect.extents.project_onto(Vec2::X) + Vec2::Y * 10.0,
            rect.extents.project_onto(-Vec2::Y) / 2.0,
        ),
        FanDirection::Left => (
            rect.extents.project_onto(Vec2::Y) + Vec2::X * 10.0,
            rect.extents.project_onto(-Vec2::X) / 2.0,
        ),
        FanDirection::Right => (
            rect.extents.project_onto(Vec2::Y) + Vec2::X * 10.0,
            rect.extents.project_onto(Vec2::X) / 2.0,
        ),
    };
    let fan_shape_bundle = GeometryBuilder::build_as(
        &Rectangle {
            extents: fan_extents,
            origin: RectangleOrigin::CustomCenter(Vec2::ZERO),
        },
        DrawMode::Fill(FillMode::color(Color::BLUE)),
        Transform {
            translation: fan_pos.extend(16.0),
            ..Default::default()
        },
    );
    commands.entity(block_ent).with_children(|parent| {
        parent.spawn_bundle(fan_shape_bundle);
    });
}

///
/// 動いている送風機とボールの間に障害物がなければ力を加える
fn generate_wind(
    fan_query: Query<(&Fan, &BlockOriginalPos, &BlockTransform, &BlockType)>,
    ball_query: Query<(&Ball, &Position)>,
) {
    for (fan, block_orig_pos, block_trans, block_type) in fan_query.iter() {
        for (ball, ball_pos) in ball_query.iter() {}
    }
}

pub(super) struct FanPlugin;
impl Plugin for FanPlugin {
    fn build(&self, app: &mut App) {}
}
