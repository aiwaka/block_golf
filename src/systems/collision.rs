use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::RectangleOrigin;

use crate::components::{
    ball::{Ball, BallNocking},
    block::{Block, RectangleBlock},
};

use super::{
    ball::BALL_RADIUS,
    field::{FIELD_HEIGHT, FIELD_WIDTH},
};

fn rotate_vec2(v: Vec2, angle: f32) -> Vec2 {
    Vec2::new(
        v.x * angle.cos() - v.y * angle.sin(),
        v.x * angle.sin() + v.y * angle.cos(),
    )
}

/// 矩形と円の当たり判定を行う.
/// rect_posは位置指定, rect_originは軸の矩形中央からの相対指定
fn rect_circle_collision(
    block: &RectangleBlock,
    rect_trans: &Transform,
    circ_pos: Vec2,
) -> Option<Vec2> {
    let rect_pos = Vec2::new(rect_trans.translation.x, rect_trans.translation.y);
    let rect_angle = block.angle;
    let rect_origin = if let RectangleOrigin::CustomCenter(center) = block.rect.origin {
        center
    } else {
        panic!("not customed origin put")
    };
    // まず円を正方形と考えてかんたんな当たり判定を行う.
    // 適当な点を, 矩形の中心から出る局所座標の成分に直す. 角度0のときの直交座標と向きを合わせて回転させる.
    // 回転軸はrect_originとは関係なくrect_posなので, 回転軸からボールまでの相対ベクトルを求める
    let pos_diff = circ_pos - rect_pos;
    // 回転を補正
    let before_rotate_vec = rotate_vec2(pos_diff, -rect_angle);
    //
    let rect_local_coord = before_rotate_vec - rect_origin;
    // 矩形同士が重なっていたら判定を行う
    if rect_local_coord.x.abs() < block.rect.extents.x / 2.0 + BALL_RADIUS
        || rect_local_coord.y.abs() < block.rect.extents.y / 2.0 + BALL_RADIUS
    {
        // 辺が重なっているか判定
        if (rect_local_coord.x.abs() < block.rect.extents.x / 2.0
            && rect_local_coord.y.abs() < block.rect.extents.y / 2.0 + BALL_RADIUS)
            || (rect_local_coord.x.abs() < block.rect.extents.x / 2.0 + BALL_RADIUS
                && rect_local_coord.y.abs() < block.rect.extents.y / 2.0)
        {
            Some(rect_local_coord)
        } else {
            // 矩形として考えた円と矩形で重なっているが一辺では重なっていない場合, 目的の円との重なりは頂点付近で決まる.
            // 各頂点が円に含まれるかどうかを判定すればよい.
            // 原点からの相対頂点座標リスト（回転基準点の補正付き）
            let relative_vertex_list = {
                let _x = block.rect.extents.x / 2.0;
                let _y = block.rect.extents.y / 2.0;
                vec![
                    Vec2::new(_x, _y) + rect_origin,
                    Vec2::new(-_x, _y) + rect_origin,
                    Vec2::new(-_x, -_y) + rect_origin,
                    Vec2::new(_x, -_y) + rect_origin,
                ]
            };
            let vertex_list = relative_vertex_list
                .iter()
                .map(|rel_v| rotate_vec2(*rel_v, rect_angle) + rect_pos)
                .collect::<Vec<Vec2>>();
            if vertex_list
                .iter()
                .any(|v| v.distance(circ_pos) < BALL_RADIUS)
            {
                Some(rect_local_coord)
            } else {
                None
            }
        }
    } else {
        None
    }
}

fn block_ball_collision(
    mut commands: Commands,
    mut ball_query: Query<(&Transform, &mut Ball, Entity)>,
    block_query: Query<(&Transform, &Block, &RectangleBlock)>,
) {
    for (ball_trans, mut ball, ent) in ball_query.iter_mut() {
        for (trans, block, rect) in block_query.iter() {
            if let Some(coord) = rect_circle_collision(
                rect,
                trans,
                Vec2::new(ball_trans.translation.x, ball_trans.translation.y),
            ) {
                // とりあえず一旦動きを止めさせる
                // commands.entity(ent).insert(BallNocking);
                // 矩形の対角線の傾きと局所座標点の傾きの関係から反射位置を割り出し, 角度を使って反射軸を作成する.
                // 傾きをa>0として|y|>|ax|の領域なら横辺, そうでなければ縦辺で反射している.
                let tilt_coef = rect.rect.extents.y / rect.rect.extents.x;
                // 反射面に対する法線方向の角度
                let reflect_normal_angle = if coord.y.abs() > tilt_coef * coord.x.abs() {
                    // 横辺で反射
                    rect.angle + coord.y.signum() * FRAC_PI_2
                } else {
                    rect.angle + (if coord.x > 0.0 { 0.0 } else { PI })
                };
                // 入射角（水平面に対して左側から入る場合を正の入射とする）
                let direction_angle = Vec2::X.angle_between(-ball.direction);
                let incident_angle = direction_angle - reflect_normal_angle;
                ball.direction = rotate_vec2(-ball.direction, -incident_angle * 2.0);
            }
        }
    }
}

#[test]
fn angle() {
    let direction_angle = (-Vec2::new(1.0, -1.0)).angle_between(Vec2::X) - FRAC_PI_2;
    println!(
        "{}",
        (-Vec2::new(1.0, -1.0)).angle_between(Vec2::X) * 180.0 / PI
    );
    println!("{}", direction_angle * 180.0 / PI);
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
