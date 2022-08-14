use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::{
    prelude::{DrawMode, FillMode, GeometryBuilder},
    shapes::{Circle, Rectangle, RectangleOrigin},
};

use crate::{
    components::{
        ball::Ball,
        block::{BlockTransform, BlockType},
        block_attach::fan::{Fan, FanDirection},
        physics::{force::Force, position::Position, velocity::Velocity},
    },
    AppState,
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
            -rect.extents.project_onto(Vec2::Y) / 2.0,
        ),
        FanDirection::Left => (
            rect.extents.project_onto(Vec2::Y) + Vec2::X * 10.0,
            -rect.extents.project_onto(Vec2::X) / 2.0,
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

// TODO: これはcollisionにある関数のコピーなのでまとめたほうがよい
fn rotate_vec2(v: Vec2, angle: f32) -> Vec2 {
    Vec2::new(
        v.x * angle.cos() - v.y * angle.sin(),
        v.x * angle.sin() + v.y * angle.cos(),
    )
}
/// ファンの両端点を計算する
fn calc_edge_points_of_fan(
    fan_direction: &FanDirection,
    block_orig_pos: Vec2,
    angle: f32,
    extents: Vec2,
) -> [Vec2; 2] {
    let half_ext = rotate_vec2(extents / 2.0, angle);
    // xだけ反転させたベクトル
    let refl_half_ext = rotate_vec2(Vec2::new(-extents.x, extents.y) / 2.0, angle);
    match fan_direction {
        FanDirection::Up => [block_orig_pos + half_ext, block_orig_pos + refl_half_ext],
        FanDirection::Down => [block_orig_pos - half_ext, block_orig_pos - refl_half_ext],
        FanDirection::Left => [block_orig_pos + refl_half_ext, block_orig_pos - half_ext],
        FanDirection::Right => [block_orig_pos - refl_half_ext, block_orig_pos + half_ext],
    }
}

/// 動いている送風機とボールの間に障害物がなければ力を加える
fn generate_wind(
    mut commands: Commands,
    fan_query: Query<(&Fan, &BlockTransform, &GlobalTransform, &BlockType)>,
    mut ball_query: Query<(&Ball, &Position, &mut Velocity, Entity)>,
) {
    for (fan, block_trans, block_glb_trans, block_type) in fan_query.iter() {
        if let BlockType::Rect { shape } = block_type {
            let angle = block_trans.angle;
            // まずファンの両端点を計算する
            let [p1, p2] = calc_edge_points_of_fan(
                &fan.direction,
                block_glb_trans.translation.truncate(),
                angle,
                shape.extents,
            );

            for (ball, ball_pos, mut ball_vel, ent) in ball_query.iter_mut() {
                let ball_pos = ball_pos.0;
                if (p2 - p1).dot(ball_pos - p1) > 0.0
                    && (p1 - p2).dot(ball_pos - p2) > 0.0
                    && (ball_pos - p1).perp_dot(p2 - p1) > 0.0
                {
                    let area = 4.0 * ball.ball_type.radius() * PI;
                    let dir_unit = (p1 - p2).perp();
                    // FIXME: 力を複数受けることができないことになっているのを改善すべき
                    // TODO: また, 障害物を挟んだ場合風が届かないようにしたい.
                    // commands
                    //     .entity(ent)
                    //     .insert(Force(dir_unit * fan.pressure * area));
                    ball_vel.0 += dir_unit * fan.pressure * area;
                    ball_vel.0 = ball_vel.0.clamp_length_max(15.0);
                }
            }
        }
    }
}

pub(super) struct FanPlugin;
impl Plugin for FanPlugin {
    fn build(&self, app: &mut App) {
        // app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(temp));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(generate_wind));
    }
}
