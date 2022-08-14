pub mod fan;
pub mod switch;
pub mod updater;

use bevy::prelude::*;

use self::{fan::FanPlugin, switch::SwitchPlugin, updater::UpdaterPlugin};

pub struct BlockAttachmentPlugin;
impl Plugin for BlockAttachmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SwitchPlugin);
        app.add_plugin(UpdaterPlugin);
        app.add_plugin(FanPlugin);
    }
}
