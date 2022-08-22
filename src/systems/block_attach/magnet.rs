use bevy::prelude::*;
use bevy_prototype_lyon::{
    prelude::{DrawMode, FillMode, GeometryBuilder},
    shapes::{Rectangle, RectangleOrigin},
};

use crate::{
    components::{
        ball::{Ball, MetalBall},
        block::{BlockTransform, BlockType},
        block_attach::{magnet::Magnet, utils::EdgeDirection},
        physics::{force::Force, position::Position},
    },
    systems::utils::calc_edge_points_of_rectangle,
    AppState,
};

/// ブロック出現時に磁石のポリゴンを描画するときに使う
pub fn spawn_magnet(commands: &mut Commands, block_ent: Entity, rect: &Rectangle, magnet: &Magnet) {
    let (magnet_extents, magnet_pos) = match magnet.direction {
        EdgeDirection::Up => (
            rect.extents.project_onto(Vec2::X) + Vec2::Y * 10.0,
            rect.extents.project_onto(Vec2::Y) / 2.0,
        ),
        EdgeDirection::Down => (
            rect.extents.project_onto(Vec2::X) + Vec2::Y * 10.0,
            -rect.extents.project_onto(Vec2::Y) / 2.0,
        ),
        EdgeDirection::Left => (
            rect.extents.project_onto(Vec2::Y) + Vec2::X * 10.0,
            -rect.extents.project_onto(Vec2::X) / 2.0,
        ),
        EdgeDirection::Right => (
            rect.extents.project_onto(Vec2::Y) + Vec2::X * 10.0,
            rect.extents.project_onto(Vec2::X) / 2.0,
        ),
    };
    let magnet_shape_bundle = GeometryBuilder::build_as(
        &Rectangle {
            extents: magnet_extents,
            origin: RectangleOrigin::CustomCenter(Vec2::ZERO),
        },
        DrawMode::Fill(FillMode::color(Color::GRAY)),
        Transform {
            translation: magnet_pos.extend(16.0),
            ..Default::default()
        },
    );
    let child_ent = commands
        .spawn_bundle(magnet_shape_bundle)
        .insert(magnet.clone())
        .id();
    commands.entity(block_ent).push_children(&[child_ent]);
}

/// 磁石とボールの間に力を加える
fn magnet_force(
    block_query: Query<(&BlockTransform, &GlobalTransform, &BlockType, &Children)>,
    magnet_query: Query<&Magnet>,
    mut ball_query: Query<(&Ball, &Position, &mut Force), With<MetalBall>>,
) {
    for (block_trans, block_glb_trans, block_type, block_children) in block_query.iter() {
        for &child in block_children.iter() {
            if let Ok(magnet) = magnet_query.get(child) {
                if magnet.active {
                    if let BlockType::Rect { shape } = block_type {
                        let angle = block_trans.angle;
                        let (_, _, block_glb_translation) =
                            block_glb_trans.to_scale_rotation_translation();
                        let [p1, p2] = calc_edge_points_of_rectangle(
                            &magnet.direction,
                            block_glb_translation.truncate(),
                            angle,
                            shape.extents,
                        );

                        for (_, ball_pos, mut force) in ball_query.iter_mut() {
                            let ball_pos = ball_pos.0;
                            if (p2 - p1).dot(ball_pos - p1) > 0.0
                                && (p1 - p2).dot(ball_pos - p2) > 0.0
                                && (ball_pos - p1).perp_dot(p2 - p1) > 0.0
                            {
                                let dir_unit = (p2 - p1).perp().normalize();
                                force.0 += dir_unit * magnet.flux_density * magnet.flux_density;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub(super) struct MagnetPlugin;
impl Plugin for MagnetPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(magnet_force));
    }
}
