use std::f32::EPSILON;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::RectangleOrigin;

use crate::components::{
    ball::{Ball, BallNocking},
    block::{Block, RectangleBlock},
    physics::{position::Position, velocity::Velocity},
};

fn rotate_vec2(v: Vec2, angle: f32) -> Vec2 {
    Vec2::new(
        v.x * angle.cos() - v.y * angle.sin(),
        v.x * angle.sin() + v.y * angle.cos(),
    )
}

/// vec3のxyだけを抜き出したものをつくる
fn vec3_to_vec2(v: Vec3) -> Vec2 {
    Vec2::new(v.x, v.y)
}

// 直交座標系に水平な矩形が, ある点を含んでいるか？
fn rect_contains_point(center: Vec2, extents: Vec2, p: Vec2) -> bool {
    let leftbottom = center - extents / 2.0;
    ((leftbottom.x)..(leftbottom.x + extents.x)).contains(&p.x)
        && ((leftbottom.y)..(leftbottom.y + extents.y)).contains(&p.y)
}

/// 当たり判定をして拘束解消に必要な情報を返す
fn collision_between_block_and_ball(
    block_info: (&RectangleBlock, &Transform),
    ball_info: (&Ball, &Transform),
) -> Option<Vec2> {
    // 矩形の回転軸からの相対位置ベクトル
    let block_origin = if let RectangleOrigin::CustomCenter(center) = block_info.0.rect.origin {
        center
    } else {
        panic!("not customed origin put")
    };
    // 横縦幅
    let block_extents = block_info.0.rect.extents;
    let block_pos = vec3_to_vec2(block_info.1.translation);
    let block_angle = block_info.0.angle;
    let ball_pos = vec3_to_vec2(ball_info.1.translation);
    let ball_radius = ball_info.0.ball_type.radius();

    // 原点に限定して判定をする簡単なものをつくっておく
    let rect_contains_origin =
        |center: Vec2, extents: Vec2| rect_contains_point(center, extents, Vec2::ZERO);

    // ボールを原点として, 矩形の角度を水平に補正した局所座標を定義する
    // lcをつけたら局所座標での値とする
    // block_centerは対角線の交点とする
    let lc_block_center = rotate_vec2(
        block_pos + rotate_vec2(block_origin, block_angle) - ball_pos,
        -block_angle,
    );
    if rect_contains_origin(
        lc_block_center,
        block_extents + Vec2::splat(ball_radius * 2.0),
    ) {
        // 辺が重なっているか判定
        if rect_contains_origin(
            lc_block_center,
            block_extents + Vec2::new(ball_radius * 2.0, 0.0),
        ) {
            // 縦の辺で重なっているとき
            // 貫通深度（めり込み度合いのスカラー）（計算はメモ30ページ）
            // 衝突法線（衝突が解消されるのに一番距離が短い方向）
            // 衝突法線の長さが貫通深度になるようにする
            let lc_collide_normal = Vec2::new(
                lc_block_center.x
                    - lc_block_center.x.signum() * (block_extents.x / 2.0 + ball_radius),
                0.0,
            );
            Some(lc_collide_normal)
        } else if rect_contains_origin(
            lc_block_center,
            block_extents + Vec2::new(0.0, ball_radius * 2.0),
        ) {
            let lc_collide_normal = Vec2::new(
                0.0,
                lc_block_center.y
                    - lc_block_center.y.signum() * (block_extents.y / 2.0 + ball_radius),
            );
            Some(lc_collide_normal)
        } else {
            // 各頂点が円に含まれるかどうかを判定すればよい.
            // 局所座標での頂点座標リスト. これらを中心とする半径が球と同じ領域のいずれかに原点が入っていればよい
            // 原点からの距離が最も近い頂点を用いるのが妥当なので, それを探す
            let nearest_vertex = {
                let _x = block_extents.x / 2.0;
                let _y = block_extents.y / 2.0;
                let vertex_list = vec![
                    Vec2::new(_x, _y) + lc_block_center,
                    Vec2::new(-_x, _y) + lc_block_center,
                    Vec2::new(-_x, -_y) + lc_block_center,
                    Vec2::new(_x, -_y) + lc_block_center,
                ];
                vertex_list
                    .into_iter()
                    .min_by(|v1, v2| v1.length().partial_cmp(&v2.length()).unwrap())
                    .unwrap()
            };
            if nearest_vertex.length() < ball_radius {
                // 局所座標で原点と頂点が重なっている場合lengthが0でおかしくなるが, とりあえず保留
                let lc_collide_normal = if nearest_vertex.length() < EPSILON {
                    // めり込みがちょうど半径と一致する場合方向を決められないので, 45度っぽい角度にしておく
                    // lc_block_centerがどの象限にあるかで方向を判別する
                    let dir_vec = if lc_block_center.x > 0.0 {
                        if lc_block_center.y > 0.0 {
                            Vec2::new(-1.0, -1.0)
                        } else {
                            Vec2::new(-1.0, 1.0)
                        }
                    } else if lc_block_center.y > 0.0 {
                        Vec2::new(1.0, -1.0)
                    } else {
                        Vec2::new(1.0, 1.0)
                    };
                    dir_vec / 2.0f32.sqrt() * ball_radius
                } else {
                    nearest_vertex * (1.0 - ball_radius / nearest_vertex.length())
                };
                Some(lc_collide_normal)
            } else {
                None
            }
        }
    } else {
        None
    }
}

fn block_ball_collision(
    mut ball_query: Query<(&Transform, &Ball, &mut Position, &mut Velocity)>,
    block_query: Query<(&Transform, &RectangleBlock), With<Block>>,
) {
    for (ball_trans, ball, mut ball_pos, mut ball_vel) in ball_query.iter_mut() {
        for (block_trans, block_rect) in block_query.iter() {
            if let Some(lc_collide_normal) =
                collision_between_block_and_ball((block_rect, block_trans), (ball, ball_trans))
            {
                // 局所座標を画面座標に修正
                let collide_normal = rotate_vec2(lc_collide_normal, block_rect.angle);

                ball_pos.0 += collide_normal;
                // 質量も反発係数もすべて1とする
                // 撃力は速度差の単位法線へ射影となり, 衝突後の速度はそれを単に足したものになる.
                let prev_vel = ball_vel.0;
                let impulsive_force = (-prev_vel).project_onto(collide_normal.normalize()) * 2.0;
                ball_vel.0 += impulsive_force;
            }
        }
    }
}

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(block_ball_collision);
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
