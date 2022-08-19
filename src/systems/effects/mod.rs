use bevy::prelude::Plugin;

use self::fade::FadeEffectPlugin;

pub mod fade;

pub struct EffectPlugin;
impl Plugin for EffectPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(FadeEffectPlugin);
    }
}
