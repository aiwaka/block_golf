use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::*, shapes::Rectangle};

use crate::{
    components::{
        block_attach::{
            switch::{SwitchReceiver, SwitchTile, SwitchType},
            updater::{Updater, UpdaterType, UpdaterVec},
        },
        timer::CountDownTimer,
    },
    events::switch::SpawnSwitchEvent,
    AppState,
};

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

fn execute_change_by_switch(
    mut commands: Commands,
    switch_query: Query<&SwitchTile, Changed<SwitchTile>>,
    mut receiver_query: Query<(&SwitchReceiver, Option<&mut UpdaterVec>, Entity)>,
) {
    for switch in switch_query.iter() {
        if switch.just_active {
            for (attachment, updater_vec, ent) in receiver_query.iter_mut() {
                if switch.target_id == attachment.target_id {
                    let mut entity_commands = commands.entity(ent);
                    match &attachment.switch_type {
                        SwitchType::ChangeRotateStrategy { before: _, after } => {
                            info!("{:?}", after);
                            entity_commands.insert(after.clone());
                        }
                        SwitchType::ChangeSlideStrategy { before: _, after } => {
                            entity_commands.insert(after.clone());
                        }
                        SwitchType::ToggleFanActive => {}
                        SwitchType::MoveBlock { range, func } => {
                            // info!("move block attachment : limit {}", limit);
                            let updater =
                                Updater::new(range.clone(), UpdaterType::BlockPos { func: *func });
                            if let Some(mut updater_vec) = updater_vec {
                                updater_vec.0.push(updater);
                            } else {
                                entity_commands.insert(UpdaterVec::new_from_a_updater(updater));
                            }
                        }
                    }
                }
            }
        } else if !switch.active {
            for (attachment, updater_vec, ent) in receiver_query.iter_mut() {
                if switch.target_id == attachment.target_id {
                    let mut entity_commands = commands.entity(ent);
                    match &attachment.switch_type {
                        SwitchType::ChangeRotateStrategy { before, after: _ } => {
                            commands.entity(ent).insert(before.clone());
                        }
                        SwitchType::ChangeSlideStrategy { before, after: _ } => {
                            commands.entity(ent).insert(before.clone());
                        }
                        SwitchType::ToggleFanActive => {}
                        SwitchType::MoveBlock { range, func } => {
                            let mut reversed_range = range.clone();
                            reversed_range.reverse();
                            let updater =
                                Updater::new(reversed_range, UpdaterType::BlockPos { func: *func });
                            if let Some(mut updater_vec) = updater_vec {
                                updater_vec.0.push(updater);
                            } else {
                                entity_commands.insert(UpdaterVec::new_from_a_updater(updater));
                            }
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
