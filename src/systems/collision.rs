use std::f32::EPSILON;

use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::RectangleOrigin, shapes::Rectangle};

use crate::{
    components::{
        ball::{Ball, BallNocking, GoalinBall},
        block::{
            Block, BlockOriginalPos, BlockSlidePath, BlockTransform, BlockType, SlideStrategy,
        },
        block_attach::switch::SwitchTile,
        goal::GoalHole,
        physics::{
            force::Force,
            material::{PhysicMaterial, Volume},
            position::Position,
            velocity::Velocity,
        },
    },
    AppState,
};

/// 直交座標系に水平な矩形が, ある点を含んでいるか？
/// center: 矩形の中心
/// extents: 矩形の大きさ（width, height）
/// p: 判定したい点
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
    let block_pos =
        block_info.1 .0 + block_slide_path.calc_orbit(block_info.2.pos_param) + block_info.2.offset;
    let block_angle = block_info.2.angle;
    let ball_pos = ball_info.1.translation.truncate();
    let ball_radius = ball_info.0.ball_type.radius();

    // 原点に限定して判定をする簡単なものをつくっておく
    let rect_contains_origin =
        |center: Vec2, extents: Vec2| rect_contains_point(center, extents, Vec2::ZERO);

    // ボールを原点として, 矩形の角度を水平に補正した局所座標を定義する
    // lcをつけたら局所座標での値とする
    // block_centerは対角線の交点とする
    let lc_block_center = Vec2::from_angle(-block_angle)
        .rotate(block_pos + Vec2::from_angle(block_angle).rotate(block_origin) - ball_pos);
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
    mut ball_query: Query<
        (
            &Transform,
            &Ball,
            &PhysicMaterial,
            &mut Position,
            &Velocity,
            &mut Force,
            &Volume,
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
    for (ball_trans, ball, ball_material, mut ball_pos, ball_vel, mut force, volume) in
        ball_query.iter_mut()
    {
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
                let collide_normal = Vec2::from_angle(block_trans.angle).rotate(lc_collide_normal);
                ball_pos.0 += collide_normal * penetrate_depth;
                let restitution = block_material.restitution * ball_material.restitution;
                // let friction = block_material.friction;
                let ball_weight = ball_material.density * volume.0;

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
                force.0 += impulsive_force;
            }
        }
    }
}

/// 衝突応答としてball1にかかるべき力を返す（ball2は向きを反転させた力を使う）
fn collision_of_balls(ball1: (&Ball, &Transform), ball2: (&Ball, &Transform)) -> Option<Vec2> {
    let ball1_radius = ball1.0.ball_type.radius();
    let ball1_pos = ball1.1.translation.truncate();
    let ball2_radius = ball2.0.ball_type.radius();
    let ball2_pos = ball2.1.translation.truncate();
    let diff = ball1_pos - ball2_pos;
    if diff.length_squared() < (ball1_radius + ball2_radius) * (ball1_radius + ball2_radius) {
        const FORCE_PARAM: f32 = 0.1;
        let diff_length = diff.length();
        let overlap_vec = if diff_length < EPSILON {
            // 球同士が完全に重なっている場合lengthが0なので, X方向に返すとして計算する.
            ball1_radius * Vec2::X
        } else {
            diff * ((ball1_radius + ball2_radius) / diff_length - 1.0)
        };
        // kx^2を返す.
        Some(overlap_vec * overlap_vec.length() * FORCE_PARAM)
    } else {
        None
    }
}

#[allow(clippy::type_complexity)]
fn balls_collision(
    mut ball_query: Query<
        (
            &Transform,
            &Ball,
            &PhysicMaterial,
            &Velocity,
            &mut Force,
            &Volume,
            Option<&BallNocking>,
        ),
        Without<GoalinBall>,
    >,
) {
    let mut ball_combination_iter = ball_query.iter_combinations_mut();
    while let Some([ball1_info, ball2_info]) = ball_combination_iter.fetch_next() {
        // nocking状態のボールは常にball2であるように入れ替える.
        let [ball1_info, ball2_info] = if ball1_info.6.is_some() && ball2_info.6.is_none() {
            [ball2_info, ball1_info]
        } else {
            [ball1_info, ball2_info]
        };
        let (ball1_trans, ball1, ball1_material, ball1_vel, mut ball1_force, ball1_vol, _) =
            ball1_info;
        let (
            ball2_trans,
            ball2,
            ball2_material,
            ball2_vel,
            mut ball2_force,
            ball2_vol,
            ball2_nocking,
        ) = ball2_info;
        if let Some(repulsive_force) =
            collision_of_balls((ball1, ball1_trans), (ball2, ball2_trans))
        {
            let restitution = ball1_material.restitution * ball2_material.restitution;
            // 素材と体積から質量を計算する
            let ball1_weight = ball1_material.density * ball1_vol.0;
            let ball2_weight = ball2_material.density * ball2_vol.0;
            // 換算質量
            let reduced_mass = ball1_weight * ball2_weight / (ball1_weight + ball2_weight);
            let vel_diff = ball2_vel.0 - ball1_vel.0;
            let [ball1_add_force, ball2_add_force] = if ball2_nocking.is_some() {
                let impulsive_force = (1.0 + restitution)
                    * ball1_weight
                    * vel_diff.project_onto(repulsive_force.normalize());
                [repulsive_force + impulsive_force, Vec2::ZERO]
            } else {
                let impulsive_force = (1.0 + restitution)
                    * reduced_mass
                    * vel_diff.project_onto(repulsive_force.normalize());
                [
                    repulsive_force + impulsive_force,
                    -repulsive_force - impulsive_force,
                ]
            };
            ball1_force.0 += ball1_add_force;
            ball2_force.0 += ball2_add_force;
        }
    }
}

fn collision_between_goal_and_ball(ball: (&Ball, &Transform), goal: &GoalHole) -> Option<Vec2> {
    let ball_radius = ball.0.ball_type.radius();
    let ball_pos = ball.1.translation.truncate();
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
                    commands.entity(ball_ent).insert(GoalinBall(goal.score));
                }
            }
        }
    }
}

/// スイッチとボールの当たり判定
#[allow(clippy::type_complexity)]
fn switch_and_ball_collision(
    mut ball_query: Query<
        (&Transform, &PhysicMaterial, &Volume),
        (With<Ball>, Without<GoalinBall>),
    >,
    mut switch_query: Query<(&Transform, &mut SwitchTile)>,
) {
    for (ball_trans, ball_material, ball_vol) in ball_query.iter_mut() {
        for (switch_trans, mut switch) in switch_query.iter_mut() {
            if switch.active {
                continue;
            }
            if let Some(ball_weight) = {
                // スイッチの上に乗っていたら重さを返す
                let ball_pos = ball_trans.translation.truncate();
                let switch_pos = switch_trans.translation.truncate();
                let switch_extents = switch.extents;
                // 球同士が完全に重なっている場合lengthが0でおかしくなるが, とりあえず保留
                if rect_contains_point(switch_pos, switch_extents, ball_pos) {
                    Some(ball_material.density * ball_vol.0)
                } else {
                    None
                }
            } {
                // スイッチがアクティブでなく閾値を超えた重さが加わったとき
                if switch.threshold < ball_weight {
                    switch.active = true;
                    switch.just_active = true;
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
        app.add_system_set(
            SystemSet::on_update(AppState::Game).with_system(
                switch_and_ball_collision
                    .before("execute_force")
                    .label("collision:switch_and_ball"),
            ),
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
