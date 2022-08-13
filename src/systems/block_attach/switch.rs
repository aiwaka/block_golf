use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::*, shapes::Rectangle};

use crate::{
    components::{
        block::{RotateStrategy, SlideStrategy},
        block_attach::switch::{SwitchReceiver, SwitchTile, SwitchType},
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

/// just_activeがtrueのときにそのフレームの最後にfalseに戻しておくためのシステム
fn off_just_active(mut query: Query<(&mut SwitchTile, Entity)>) {
    for (mut switch, ent) in query.iter_mut() {
        if switch.just_active {
            switch.just_active = false;
            // info!("off just active {:?}", ent);
        }
    }
}

fn execute_change_by_switch(
    mut commands: Commands,
    switch_query: Query<&SwitchTile, Changed<SwitchTile>>,
    receiver_query: Query<(&SwitchReceiver, Entity)>,
) {
    for switch in switch_query.iter() {
        let target_id = switch.target_id;
        if switch.just_active {
            for (attachment, ent) in receiver_query.iter() {
                if target_id == attachment.target_id {
                    match &attachment.switch_type {
                        SwitchType::ChangeRotateStrategy { before: _, after } => {
                            info!("{:?}", after);
                            commands.entity(ent).insert(after.clone());
                        }
                        SwitchType::ChangeSlideStrategy { before: _, after } => {
                            commands.entity(ent).insert(after.clone());
                        }
                        SwitchType::ToggleFanActive => {}
                    }
                }
            }
        } else if !switch.active {
            for (attachment, ent) in receiver_query.iter() {
                if target_id == attachment.target_id {
                    match &attachment.switch_type {
                        SwitchType::ChangeRotateStrategy { before, after: _ } => {
                            commands.entity(ent).insert(before.clone());
                        }
                        SwitchType::ChangeSlideStrategy { before, after: _ } => {
                            commands.entity(ent).insert(before.clone());
                        }
                        SwitchType::ToggleFanActive => {}
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
            SystemSet::on_update(AppState::Game)
                .with_system(deactivate_switch.after("count_down_update")),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Game).with_system(execute_change_by_switch),
        );
        app.add_system_to_stage(CoreStage::Last, off_just_active);
    }
}
