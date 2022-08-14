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
        physics::position::Position,
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

#[derive(Component)]
struct TempPin;
#[derive(Component)]
struct TempText;
fn temp(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &Circle {
                radius: 10.0,
                center: Vec2::new(0.0, 0.0),
            },
            DrawMode::Fill(FillMode::color(Color::PURPLE)),
            Transform::default(),
        ))
        .insert(TempPin);
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(30.0),
                    right: Val::Px(40.0),
                    ..default()
                },
                ..default()
            },
            text: Text::with_section(
                "angle",
                TextStyle {
                    font: asset_server.load("fonts/ume-tgs5.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
                TextAlignment::default(),
            ),
            ..default()
        })
        .insert(TempText);
}

/// 動いている送風機とボールの間に障害物がなければ力を加える
fn generate_wind(
    mut commands: Commands,
    fan_query: Query<(&Fan, &BlockTransform, &GlobalTransform, &BlockType)>,
    ball_query: Query<(&Ball, &Position)>,
    mut temp: Query<(&mut Transform, &TempPin)>,
    mut temp_text: Query<(&mut Text, &TempText)>,
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

            for (mut t, _) in temp.iter_mut() {
                t.translation = p2.extend(80.0);
            }
            for (mut t, _) in temp_text.iter_mut() {
                t.sections[0].value = angle.to_string();
            }
            for (ball, ball_pos) in ball_query.iter() {}
        }
    }
}

pub(super) struct FanPlugin;
impl Plugin for FanPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(temp));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(generate_wind));
    }
}
