use bevy::prelude::*;

use crate::components::block_attach::switch::SwitchTile;

pub struct SpawnSwitchEvent {
    pub component: SwitchTile,
    pub pos: Vec2,
}
