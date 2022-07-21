//! プレイヤーに見えるゲーム情報を表示するシステム等
use bevy::prelude::*;

use crate::components::{
    ball::SpawnBallEvent,
    info::{ConsumingBall, RemainingBall},
    launcher::BallMagazine,
};

/// ボール出現時に箱の先頭のボールに更新中マーカーを付与し, 箱から取り出す
fn pop_ball_from_magazine(
    mut commands: Commands,
    mut magazine_query: Query<&mut BallMagazine>,
    mut spawn_ball_event_reader: EventReader<SpawnBallEvent>,
) {
    for _ in spawn_ball_event_reader.iter() {
        let mut mag = magazine_query.single_mut();
        // TODO: VecDequeを使うと計算量を減らせる
        let (_, top_ball_ent) = mag.balls.remove(0);
        // 1つ目に消費中マーカーをつける
        commands.entity(top_ball_ent).insert(ConsumingBall);
    }
}

fn update_remaining_balls_info(
    mut commands: Commands,
    mut remaining_ball_query: Query<
        (&mut Transform, Option<&ConsumingBall>, Entity),
        With<RemainingBall>,
    >,
) {
    if let Some((mut trans, _, ent)) = remaining_ball_query.iter_mut().find(|q| q.1.is_some()) {
        // consumingballが存在していたらその処理だけ行う
        trans.scale *= 0.8;
        if trans.scale.x < 0.01 {
            commands.entity(ent).despawn();
        }
    }
}

pub struct InfoBoardPlugin;
impl Plugin for InfoBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(pop_ball_from_magazine);
        app.add_system(update_remaining_balls_info);
    }
}
