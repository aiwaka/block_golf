pub mod ball;
pub mod block;
pub mod game;
pub mod goal;
pub mod launcher;
pub mod main_menu;
pub mod switch;

use bevy::prelude::*;

// これは定義をその場でしたほうが良さげなので再公開という形にする
use crate::components::main_menu::menu::ChangeMenuLayerEvent;

use self::{
    ball::{LaunchBallEvent, SetBallEvent, SpawnBallEvent},
    block::SpawnBlockEvent,
    game::GameOverEvent,
    goal::SpawnGoalEvent,
    launcher::SpawnLauncherEvent,
    switch::SpawnSwitchEvent,
};

pub trait ToSpawnEvent {
    type E;
    fn to_spawn_event(&self) -> Self::E;
}

pub fn add_events(app: &mut App) {
    app.add_event::<SpawnBallEvent>();
    app.add_event::<LaunchBallEvent>();
    app.add_event::<SpawnLauncherEvent>();
    app.add_event::<SpawnBlockEvent>();
    app.add_event::<SpawnGoalEvent>();
    app.add_event::<SetBallEvent>();
    app.add_event::<SpawnSwitchEvent>();
    app.add_event::<GameOverEvent>();
    app.add_event::<ChangeMenuLayerEvent>();
}
