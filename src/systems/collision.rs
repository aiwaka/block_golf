use std::f32::EPSILON;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::RectangleOrigin;

use crate::components::{
    ball::{Ball, GoalinBall},
    block::{Block, RectangleBlock},
    goal::GoalHole,
    physics::{material::PhysicMaterial, position::Position, velocity::Velocity},
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

/// 当たり判定をして拘束解消に必要な情報（拘束方向と貫通深度のタプル）を返す
fn collision_between_block_and_ball(
    block_info: (&RectangleBlock, &Transform),
    ball_info: (&Ball, &Transform),
) -> Option<(Vec2, f32)> {
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
            Some((
                -Vec2::X * lc_block_center.x.signum(),
                (lc_block_center.x
                    - lc_block_center.x.signum() * (block_extents.x / 2.0 + ball_radius))
                    .abs(),
            ))
        } else if rect_contains_origin(
            lc_block_center,
            block_extents + Vec2::new(0.0, ball_radius * 2.0),
        ) {
            Some((
                -Vec2::Y * lc_block_center.y.signum(),
                (lc_block_center.y
                    - lc_block_center.y.signum() * (block_extents.y / 2.0 + ball_radius))
                    .abs(),
            ))
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
                // squareのほうがradiusの二乗より重いと考えられるのでこちらを使う
                vertex_list
                    .into_iter()
                    .min_by(|v1, v2| {
                        v1.length_squared()
                            .partial_cmp(&v2.length_squared())
                            .unwrap()
                    })
                    .unwrap()
            };
            let nearest_vertex_length = nearest_vertex.length();
            if nearest_vertex_length < ball_radius {
                let lc_collide_normal = if nearest_vertex_length < EPSILON {
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
                    nearest_vertex * (1.0 - ball_radius / nearest_vertex_length)
                };
                Some((lc_collide_normal.normalize(), lc_collide_normal.length()))
            } else {
                None
            }
        }
    } else {
        None
    }
}

fn block_ball_collision(
    mut ball_query: Query<
        (
            &Transform,
            &Ball,
            &PhysicMaterial,
            &mut Position,
            &mut Velocity,
        ),
        Without<GoalinBall>,
    >,
    block_query: Query<(&Transform, &RectangleBlock, &PhysicMaterial), With<Block>>,
) {
    for (ball_trans, ball, ball_material, mut ball_pos, mut ball_vel) in ball_query.iter_mut() {
        for (block_trans, block_rect, block_material) in block_query.iter() {
            if let Some((lc_collide_normal, penetrate_depth)) =
                collision_between_block_and_ball((block_rect, block_trans), (ball, ball_trans))
            {
                // 局所座標を画面座標に修正
                let collide_normal = rotate_vec2(lc_collide_normal, block_rect.angle);
                ball_pos.0 += collide_normal * penetrate_depth;
                let restitution = block_material.restitution * ball_material.restitution;
                let friction = block_material.friction;
                let block_weight =
                    block_material.density * block_rect.rect.extents.x * block_rect.rect.extents.y;
                // TODO: ボールタイプを保持するかどうか
                let ball_weight = ball.ball_type.weight();
                // 換算質量
                let reduced_mass = block_weight * ball_weight / (block_weight + ball_weight);
                // 質量も反発係数もすべて1とする
                // 撃力は速度差の単位法線へ射影となり, 衝突後の速度はそれを単に足したものになる.
                let prev_vel = ball_vel.0;
                let impulsive_force = (1.0 + restitution)
                    * reduced_mass
                    * (-prev_vel).project_onto(collide_normal)
                    * 2.0;
                ball_vel.0 += impulsive_force;
            }
        }
    }
}

/// 衝突応答としてball1にかかるべき力を返す（ball2は向きを反転させた力を使う）
/// ...力を扱うシステムを実装していないので, とりあえず貫通深度を返す
fn collision_of_balls(ball1: (&Ball, &Transform), ball2: (&Ball, &Transform)) -> Option<Vec2> {
    let ball1_radius = ball1.0.ball_type.radius();
    let ball1_pos = vec3_to_vec2(ball1.1.translation);
    let ball2_radius = ball2.0.ball_type.radius();
    let ball2_pos = vec3_to_vec2(ball2.1.translation);
    let diff = ball1_pos - ball2_pos;
    // 球同士が完全に重なっている場合lengthが0でおかしくなるが, とりあえず保留
    if diff.length_squared() < (ball1_radius + ball2_radius) * (ball1_radius + ball2_radius) {
        Some(diff * ((ball1_radius + ball2_radius) / diff.length() - 1.0))
    } else {
        None
    }
}

fn balls_collision(
    mut ball_query: Query<
        (
            &Transform,
            &Ball,
            &PhysicMaterial,
            &mut Position,
            &mut Velocity,
        ),
        Without<GoalinBall>,
    >,
) {
    let mut ball_combination_iter = ball_query.iter_combinations_mut();
    while let Some([ball1_info, ball2_info]) = ball_combination_iter.fetch_next() {
        let (ball1_trans, ball1, ball1_material, mut ball1_pos, mut ball1_vel) = ball1_info;
        let (ball2_trans, ball2, ball2_material, mut ball2_pos, mut ball2_vel) = ball2_info;
        if let Some(collide_normal) = collision_of_balls((ball1, ball1_trans), (ball2, ball2_trans))
        {
            ball1_pos.0 += collide_normal / 2.0;
            ball2_pos.0 -= collide_normal / 2.0;
            let restitution = ball1_material.restitution * ball2_material.restitution;
            // TODO: 今後ボールタイプを保持するかどうかは考える
            let ball1_weight = ball1.ball_type.weight();
            let ball2_weight = ball2.ball_type.weight();
            // 換算質量
            let reduced_mass = ball1_weight * ball2_weight / (ball1_weight + ball2_weight);
            let vel_diff = ball2_vel.0 - ball1_vel.0;
            let impulsive_force = (1.0 + restitution)
                * reduced_mass
                * vel_diff.project_onto(collide_normal.normalize());
            ball1_vel.0 += impulsive_force;
            ball2_vel.0 -= impulsive_force;
        }
    }
}

fn collision_between_goal_and_ball(ball: (&Ball, &Transform), goal: &GoalHole) -> Option<Vec2> {
    let ball_radius = ball.0.ball_type.radius();
    let ball_pos = vec3_to_vec2(ball.1.translation);
    let goal_radius = goal.radius;
    let goal_pos = goal.pos;
    let diff = ball_pos - goal_pos;
    // 球同士が完全に重なっている場合lengthが0でおかしくなるが, とりあえず保留
    if diff.length_squared() < (ball_radius + goal_radius) * (ball_radius + goal_radius) {
        Some(diff)
    } else {
        None
    }
}

fn goal_and_ball_collision(
    mut commands: Commands,
    mut ball_query: Query<(&Transform, &Ball, &mut Velocity, Entity), Without<GoalinBall>>,
    goal_query: Query<(&Transform, &GoalHole)>,
) {
    for (ball_trans, ball, mut velocity, ball_ent) in ball_query.iter_mut() {
        for (_, goal) in goal_query.iter() {
            if let Some(diff_vec) = collision_between_goal_and_ball((ball, ball_trans), goal) {
                if diff_vec.length() < goal.radius * 0.9 {
                    velocity.0 = Vec2::ZERO;
                    commands.entity(ball_ent).insert(GoalinBall);
                }
            }
        }
    }
}

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(block_ball_collision);
        app.add_system(balls_collision);
        app.add_system(goal_and_ball_collision);
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
