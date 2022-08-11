use std::f32::EPSILON;

use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::RectangleOrigin, shapes::Rectangle};

use crate::{
    components::{
        ball::{Ball, BallNocking, GoalinBall},
        block::{
            Block, BlockOriginalPos, BlockSlidePath, BlockTransform, BlockType, SlideStrategy,
        },
        goal::GoalHole,
        physics::{force::Force, material::PhysicMaterial, position::Position, velocity::Velocity},
    },
    AppState,
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
    block_info: (&Rectangle, &BlockOriginalPos, &BlockTransform),
    block_slide_path: &BlockSlidePath,
    ball_info: (&Ball, &Transform),
) -> Option<(Vec2, f32)> {
    // 矩形の回転軸からの相対位置ベクトル
    let block_origin = if let RectangleOrigin::CustomCenter(center) = block_info.0.origin {
        center
    } else {
        panic!("not customed origin put")
    };
    // 横縦幅
    let block_extents = block_info.0.extents;
    let block_pos = block_info.1 .0 + block_slide_path.calc_orbit(block_info.2.pos_param);
    let block_angle = block_info.2.angle;
    let ball_pos = ball_info.1.translation.truncate();
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

#[allow(clippy::type_complexity)]
fn block_ball_collision(
    mut commands: Commands,
    mut ball_query: Query<
        (
            &Transform,
            &Ball,
            &PhysicMaterial,
            &mut Position,
            &Velocity,
            Entity,
        ),
        Without<GoalinBall>,
    >,
    block_query: Query<
        (
            &BlockTransform,
            &BlockType,
            &BlockOriginalPos,
            &PhysicMaterial,
            Option<&SlideStrategy>,
        ),
        With<Block>,
    >,
) {
    for (ball_trans, ball, ball_material, mut ball_pos, ball_vel, ent) in ball_query.iter_mut() {
        for (block_trans, block_type, block_original_pos, block_material, slide_strategy) in
            block_query.iter()
        {
            // 移動軌道を取得
            let path = if let Some(slide_strategy) = slide_strategy {
                slide_strategy.get_path()
            } else {
                BlockSlidePath::NoPath
            };
            if let Some((lc_collide_normal, penetrate_depth)) = match *block_type {
                BlockType::Wall { shape } => collision_between_block_and_ball(
                    (&shape, block_original_pos, block_trans),
                    &path,
                    (ball, ball_trans),
                ),
                BlockType::Rect { shape } => collision_between_block_and_ball(
                    (&shape, block_original_pos, block_trans),
                    &path,
                    (ball, ball_trans),
                ),
                // TODO: 楕円形ブロックとの衝突を実装
                BlockType::Ellipse { shape } => {
                    continue;
                }
            } {
                // 局所座標を画面座標に修正
                let collide_normal = rotate_vec2(lc_collide_normal, block_trans.angle);
                ball_pos.0 += collide_normal * penetrate_depth;
                let restitution = block_material.restitution * ball_material.restitution;
                // let friction = block_material.friction;
                // TODO: ボールタイプを保持するかどうか
                let ball_weight = ball.ball_type.weight();

                // let [block_x, block_y] = block_rect.rect.extents.to_array();
                // let block_mass = block_material.density * block_x * block_y;
                // 慣性モーメント（[0, 0, z]のようなベクトルなのでスカラーで保持する）
                // let inertia_moment = block_mass * (block_x * block_x + block_y * block_y) / 12.0;
                // (I^{-1}(r\times n))\times r：方向はrを90度回転させた向き.
                // |n|=1なので大きさは |r|^2\sin(rからみたnの角度)となる.
                // 衝突点までの位置ベクトルは, 球形なので, 拘束方向ベクトルと半径の積.
                // 球が衝突する場合rとnの向きが平行なのでこの項は0になる.

                // 撃力は速度差の単位法線へ射影となり, 衝突後の速度はそれを単に足したものになる.
                // ブロックが止まっているときを考えたいので相対速度補正
                let delta = match *block_type {
                    BlockType::Wall { shape } => {
                        if let RectangleOrigin::CustomCenter(center) = shape.origin {
                            center
                        } else {
                            panic!("custom center error");
                        }
                    }
                    BlockType::Rect { shape } => {
                        if let RectangleOrigin::CustomCenter(center) = shape.origin {
                            center
                        } else {
                            panic!("custom center error");
                        }
                    }
                    BlockType::Ellipse { shape } => {
                        continue;
                    }
                };
                let prev_vel = ball_vel.0 - block_trans.pos_diff(&path, delta);
                // let prev_vel = ball_vel.0;
                let impulsive_force =
                    (1.0 + restitution) * ball_weight * (-prev_vel).project_onto(collide_normal);
                commands.entity(ent).insert(Force(impulsive_force));
            }
        }
    }
}

/// 衝突応答としてball1にかかるべき力を返す（ball2は向きを反転させた力を使う）
/// ...力を扱うシステムを実装していないので, とりあえず貫通深度を返す
/// TODO: -> 実装したのでそのように変更しても良さそう
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

#[allow(clippy::type_complexity)]
fn balls_collision(
    mut commands: Commands,
    mut ball_query: Query<
        (
            &Transform,
            &Ball,
            &PhysicMaterial,
            &mut Position,
            &mut Velocity,
            Option<&BallNocking>,
            Entity,
        ),
        Without<GoalinBall>,
    >,
) {
    let mut ball_combination_iter = ball_query.iter_combinations_mut();
    while let Some([ball1_info, ball2_info]) = ball_combination_iter.fetch_next() {
        // nocking状態のボールは常にball2であるようにする.
        let [ball1_info, ball2_info] = if ball1_info.5.is_some() && ball2_info.5.is_none() {
            [ball2_info, ball1_info]
        } else {
            [ball1_info, ball2_info]
        };
        let (ball1_trans, ball1, ball1_material, mut ball1_pos, ball1_vel, _, ball1_ent) =
            ball1_info;
        let (
            ball2_trans,
            ball2,
            ball2_material,
            mut ball2_pos,
            ball2_vel,
            ball2_nocking,
            ball2_ent,
        ) = ball2_info;
        if let Some(collide_normal) = collision_of_balls((ball1, ball1_trans), (ball2, ball2_trans))
        {
            if ball2_nocking.is_some() {
                ball1_pos.0 += collide_normal;
            } else {
                ball1_pos.0 += collide_normal / 2.0;
                ball2_pos.0 -= collide_normal / 2.0;
            }
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
            // info!("ent: {:?}, force: {}", ball1_ent, impulsive_force);
            commands.entity(ball1_ent).insert(Force(impulsive_force));
            commands.entity(ball2_ent).insert(Force(-impulsive_force));
            // ball1_vel.0 += impulsive_force;
            // ball2_vel.0 -= impulsive_force;
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
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(block_ball_collision.before("execute_force")),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(balls_collision.before("execute_force")),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(goal_and_ball_collision.before("execute_force")),
        );
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
