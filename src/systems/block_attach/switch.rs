use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::*, shapes::Rectangle};

use crate::{
    components::{
        block::{Block, BlockPosOffset},
        block_attach::{
            switch::{SwitchReceiver, SwitchTile, SwitchType},
            updater::{BlockAngleUpdater, BlockPosUpdater, OffsetByUpdater, Updater},
        },
        timer::{CountDownTimer, FrameCounter},
    },
    events::switch::SpawnSwitchEvent,
    AppState,
};

/// スイッチボタンを出現させる
fn spawn_switch(mut commands: Commands, mut event_reader: EventReader<SpawnSwitchEvent>) {
    for ev in event_reader.iter() {
        let color = if ev.component.active {
            // 押されているなら濃い色にする
            Color::DARK_GREEN
        } else {
            Color::YELLOW_GREEN
        };
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &Rectangle {
                    extents: ev.component.extents,
                    origin: RectangleOrigin::CustomCenter(Vec2::ZERO),
                },
                DrawMode::Outlined {
                    fill_mode: FillMode::color(color),
                    outline_mode: StrokeMode::new(Color::DARK_GRAY, 2.0),
                },
                Transform {
                    translation: ev.pos.extend(10.5),
                    ..Default::default()
                },
            ))
            .insert(ev.component.clone());
    }
}

/// ブロック出現時にレシーバーを付与するときに使う関数（システムではなくただの関数）
pub fn attach_switch_receiver(
    commands: &mut Commands,
    block_ent: Entity,
    receiver: &SwitchReceiver,
) {
    // デバッグ用に見た目をつくる
    let shape_bundle = GeometryBuilder::build_as(
        &Rectangle {
            extents: Vec2::splat(5.0),
            origin: RectangleOrigin::Center,
        },
        DrawMode::Fill(FillMode::color(Color::RED)),
        Transform {
            translation: Vec2::ZERO.extend(60.0),
            ..Default::default()
        },
    );
    // 子コンポーネントとして生成・追加
    let child_ent = commands
        .spawn()
        .insert_bundle(shape_bundle)
        .insert(receiver.clone())
        .id();
    commands.entity(block_ent).push_children(&[child_ent]);
}

/// タイマーが切れたらスイッチのactiveを切る（タイマーは自動で除去される）
fn deactivate_switch(mut query: Query<(&mut SwitchTile, &CountDownTimer)>) {
    for (mut switch, timer) in query.iter_mut() {
        if timer.is_finished() {
            switch.active = false;
        }
    }
}

/// スイッチの状態が変化したときの処理
fn switch_state_changed(
    mut commands: Commands,
    mut query: Query<(&mut SwitchTile, &mut DrawMode, Entity), Changed<SwitchTile>>,
) {
    for (switch, mut draw_mode, ent) in query.iter_mut() {
        if switch.just_active {
            // info!("switch {:?} is just active", ent);
            if let Some(count) = switch.auto_reverse {
                commands
                    .entity(ent)
                    .insert(CountDownTimer::new_will_not_be_removed(count));
            }

            if let DrawMode::Outlined {
                ref mut fill_mode,
                outline_mode: _,
            } = *draw_mode
            {
                fill_mode.color = Color::DARK_GREEN;
            }
        } else if !switch.active {
            // info!("switch {:?} is not active", ent);
            if let DrawMode::Outlined {
                ref mut fill_mode,
                outline_mode: _,
            } = *draw_mode
            {
                fill_mode.color = Color::YELLOW_GREEN;
            }
        }
    }
}

/// just_activeとjust_reverseがtrueのときにそのフレームの最後にfalseに戻しておくためのシステム
fn off_one_frame_flag(mut query: Query<&mut SwitchTile>) {
    for mut switch in query.iter_mut() {
        if switch.just_active {
            switch.just_active = false;
        }
    }
}

/// スイッチによる変化を処理する.
/// アップデーターを付与するなどを行う.
fn execute_change_by_switch(
    mut commands: Commands,
    switch_query: Query<&SwitchTile, Changed<SwitchTile>>,
    receiver_query: Query<(&SwitchReceiver, &Parent)>,
    block_q: Query<Entity, With<Block>>,
) {
    for switch in switch_query.iter() {
        if switch.just_active {
            for (receiver, receiver_parents) in receiver_query.iter() {
                // 動作したスイッチとレシーバーのidが一致すれば動作させる
                if switch.target_id == receiver.target_id {
                    if let Ok(block_ent) = block_q.get(receiver_parents.get()) {
                        let mut entity_commands = commands.entity(block_ent);
                        match &receiver.switch_type {
                            SwitchType::ChangeRotateStrategy { before: _, after } => {
                                // info!("{:?}", after);
                                // entity_commands.insert(after.clone());
                            }
                            SwitchType::ChangeSlideStrategy { before: _, after } => {
                                // entity_commands.insert(after.clone());
                            }
                            SwitchType::ToggleFanActive => {}
                            SwitchType::MoveBlock { range, func } => {
                                // ブロックの子コンポーネントとしてアップデーターを追加
                                // info!("move block attachment : limit {}", limit);
                                let updater_ent = commands
                                    .spawn()
                                    .insert(Updater::new(
                                        switch.target_id,
                                        range.clone(),
                                        if let Some(auto_reverse) = switch.auto_reverse {
                                            auto_reverse
                                        } else {
                                            u32::MAX
                                        },
                                    ))
                                    .insert(BlockPosOffset::default())
                                    .insert(OffsetByUpdater)
                                    .insert(BlockPosUpdater { func: *func })
                                    .insert(FrameCounter::new())
                                    .id();
                                commands.entity(block_ent).push_children(&[updater_ent]);
                            }
                            SwitchType::RotateBlock { range, func } => {}
                        }
                    }
                }
            }
        } else if !switch.active {
            for (receiver, receiver_parents) in receiver_query.iter() {
                if switch.target_id == receiver.target_id {
                    if let Ok(block_ent) = block_q.get(receiver_parents.get()) {
                        let mut entity_commands = commands.entity(block_ent);
                        match &receiver.switch_type {
                            SwitchType::ChangeRotateStrategy { before, after: _ } => {
                                // commands.entity(ent).insert(before.clone());
                            }
                            SwitchType::ChangeSlideStrategy { before, after: _ } => {
                                // commands.entity(ent).insert(before.clone());
                            }
                            SwitchType::ToggleFanActive => {}
                            SwitchType::MoveBlock { range, func } => {
                                let mut reversed_range = range.clone();
                                reversed_range.reverse();

                                let updater_ent = commands
                                    .spawn()
                                    .insert(Updater::new(
                                        switch.target_id,
                                        reversed_range,
                                        if let Some(auto_reverse) = switch.auto_reverse {
                                            auto_reverse
                                        } else {
                                            u32::MAX
                                        },
                                    ))
                                    .insert(BlockPosOffset::default())
                                    .insert(OffsetByUpdater)
                                    .insert(BlockPosUpdater { func: *func })
                                    .insert(FrameCounter::new())
                                    .id();
                                commands.entity(block_ent).push_children(&[updater_ent]);
                            }
                            SwitchType::RotateBlock { range, func } => {}
                        }
                    }
                }
            }
        }
    }
}

pub(super) struct SwitchPlugin;
impl Plugin for SwitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Game)
                .with_system(spawn_switch)
                .after("spawn_stage_entities"),
        );
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(switch_state_changed));
        app.add_system_set(
            SystemSet::on_update(AppState::Game).with_system(
                deactivate_switch
                    .after("count_down_update")
                    .label("switch:deactivate_switch"),
            ),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(execute_change_by_switch.after("collision:switch_and_ball")),
        );
        app.add_system_to_stage(CoreStage::Last, off_one_frame_flag);
    }
}
