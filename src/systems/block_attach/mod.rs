pub mod fan;
pub mod switch;

use bevy::prelude::*;

use self::switch::SwitchPlugin;

pub struct BlockAttachmentPlugin;
impl Plugin for BlockAttachmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SwitchPlugin);
    }
}
