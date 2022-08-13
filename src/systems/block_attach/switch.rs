use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::*, shapes::Rectangle};

use crate::{
    components::block_attach::switch::SwitchTile, events::switch::SpawnSwitchEvent, AppState,
};

fn spawn_switch(mut commands: Commands, mut event_reader: EventReader<SpawnSwitchEvent>) {
    for ev in event_reader.iter() {
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &Rectangle {
                    extents: Vec2::splat(40.0),
                    origin: RectangleOrigin::CustomCenter(Vec2::ZERO),
                },
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::LIME_GREEN),
                    outline_mode: StrokeMode::new(Color::DARK_GRAY, 2.0),
                },
                Transform {
                    translation: ev.pos.extend(12.0),
                    ..Default::default()
                },
            ))
            .insert(ev.component.clone());
    }
}

/// スイッチを押す処理
fn push_switch(mut commands: Commands, query: Query<(&SwitchTile)>) {}

pub(super) struct SwitchPlugin;
impl Plugin for SwitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Game)
                .with_system(spawn_switch)
                .after("spawn_stage_entities"),
        );
    }
}
