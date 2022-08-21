pub mod fan;
pub mod magnet;
pub mod switch;
pub mod updater;

use bevy::prelude::*;

use self::{fan::FanPlugin, magnet::MagnetPlugin, switch::SwitchPlugin, updater::UpdaterPlugin};

pub struct BlockAttachmentPlugin;
impl Plugin for BlockAttachmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SwitchPlugin);
        app.add_plugin(UpdaterPlugin);
        app.add_plugin(FanPlugin);
        app.add_plugin(MagnetPlugin);
    }
}
