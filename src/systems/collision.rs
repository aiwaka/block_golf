use bevy::prelude::*;
use bevy_prototype_lyon::prelude::RectangleOrigin;
use heron::rapier_plugin::nalgebra::ComplexField;

use crate::components::{
    ball::{Ball, BallNocking},
    block::{Block, RectangleBlock},
};

use super::{
    ball::BALL_RADIUS,
    field::{FIELD_HEIGHT, FIELD_WIDTH},
};

/// 矩形と円の当たり判定を行う.
/// rect_posは位置指定, rect_originは軸の矩形中央からの相対指定
fn rect_circle_collision(block: &RectangleBlock, rect_trans: &Transform, circ_pos: Vec2) -> bool {
    let rect_pos = Vec2::new(rect_trans.translation.x, rect_trans.translation.y);
    let rect_angle = block.angle;
    let rect_origin = if let RectangleOrigin::CustomCenter(center) = block.rect.origin {
        center
    } else {
        panic!("not customed origin put")
    };
    let rotate_vec2 = |v: Vec2, angle: f32| -> Vec2 {
        Vec2::new(
            v.x * angle.cos() - v.y * angle.sin(),
            v.x * angle.sin() + v.y * angle.cos(),
        )
    };
    // まず円を正方形と考えてかんたんな当たり判定を行う.
    // 適当な点を, 矩形の中心から出る局所座標の成分に直す. 角度0のときの直交座標と向きを合わせて回転させる.
    // rect_posを引き補正. その後はoriginを中心とした逆回転操作を行えばよい.
    let pos_diff = circ_pos - rect_pos - rect_origin;
    let before_rotate_vec = rotate_vec2(pos_diff, -rect_angle);
    let rect_local_coord = before_rotate_vec + rect_origin;
    // 矩形同士が重なっていたら判定を行う
    if rect_local_coord.x.abs() < block.rect.extents.x / 2.0 + BALL_RADIUS
        || rect_local_coord.y.abs() < block.rect.extents.y / 2.0 + BALL_RADIUS
    {
        // 矩形で重なっている場合, 円と重なっていないのは頂点付近が怪しい場合のみ.
        // 矩形と円が重なっていてかつ頂点を円が含まない場合は必ず矩形としても重なっているので,
        // 各頂点が円に含まれないかどうかを判定すればよい.
        let dir_temp: Vec<(f32, f32)> = vec![(1.0, 1.0), (-1.0, 1.0), (-1.0, -1.0), (1.0, -1.0)];
        // 原点からの相対頂点座標リスト（回転基準点の補正付き）
        let relative_vertex_list = dir_temp
            .iter()
            .map(|p| {
                Vec2::new(
                    p.0 * block.rect.extents.x / 2.0,
                    p.1 * block.rect.extents.y / 2.0,
                ) - rect_origin
            })
            .collect::<Vec<Vec2>>();
        let vertex_list = relative_vertex_list
            .iter()
            .map(|rel_v| rotate_vec2(*rel_v, rect_angle) + rect_pos + rect_origin)
            .collect::<Vec<Vec2>>();
        vertex_list
            .iter()
            .any(|v| v.distance(circ_pos) < BALL_RADIUS)
    } else {
        false
    }
}

fn block_ball_collision(
    mut commands: Commands,
    ball_query: Query<(&Transform, &Ball, Entity)>,
    block_query: Query<(&Transform, &Block, &RectangleBlock)>,
) {
    for (ball_trans, ball, ent) in ball_query.iter() {
        for (trans, block, rect) in block_query.iter() {
            let collide = rect_circle_collision(
                rect,
                trans,
                Vec2::new(ball_trans.translation.x, ball_trans.translation.y),
            );
            if collide {
                // とりあえず一旦動きを止めさせる
                commands.entity(ent).insert(BallNocking);
            }
        }
    }
}

fn field_ball_collision(mut ball_query: Query<(&Transform, &mut Ball)>) {
    for (trans, mut ball) in ball_query.iter_mut() {
        let pos_x = trans.translation.x;
        let pos_y = trans.translation.y;
        if pos_x.abs() + BALL_RADIUS > FIELD_WIDTH / 2.0 {
            ball.direction.x *= -1.0;
        }
        if pos_y.abs() + BALL_RADIUS > FIELD_HEIGHT / 2.0 {
            ball.direction.y *= -1.0;
        }
    }
}

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(block_ball_collision);
        app.add_system(field_ball_collision);
    }
}

#[test]
fn test() {
    use std::f32::consts::PI;
    let circ_pos = Vec2::new(1.0, 1.0);
    let rect_pos = Vec2::new(0.0, 0.0);
    let rect_angle = PI / 2.0;
    let rect_origin = Vec2::new(1.0, 1.0);

    let pos_diff = circ_pos - rect_pos - rect_origin;
    let before_rotate_vec = Vec2::new(
        pos_diff.x * rect_angle.cos() + pos_diff.y * rect_angle.sin(),
        -pos_diff.x * rect_angle.sin() + pos_diff.y * rect_angle.cos(),
    );
    let rect_local_coord = before_rotate_vec + rect_origin;
    // let rect_local_coord = before_rotate_vec - rect_origin;
    println!("{}", rect_local_coord);
}
