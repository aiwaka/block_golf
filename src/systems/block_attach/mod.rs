pub mod fan;
pub mod switch;

use bevy::prelude::*;

use crate::components::block_attach::switch::{SwitchReceiver, SwitchTile, SwitchType};

use self::switch::SwitchPlugin;

fn execute_change_by_switch(
    mut commands: Commands,
    switch_query: Query<&SwitchTile>,
    receiver_query: Query<&SwitchReceiver>,
) {
    for switch in switch_query.iter() {
        let target_id = switch.target_id;
        for attachment in receiver_query.iter() {
            if target_id == attachment.target_id {
                match &attachment.switch_type {
                    SwitchType::ChangeRotateStrategy { before, after } => {}
                    SwitchType::ChangeSlideStrategy { before, after } => {}
                    SwitchType::ToggleFanActive => {}
                }
            }
        }
    }
}

pub struct BlockAttachmentPlugin;
impl Plugin for BlockAttachmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SwitchPlugin);
    }
}
