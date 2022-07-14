pub mod motion_dynamics;

use self::motion_dynamics::MotionDynamicsPlugin;
use bevy::prelude::Plugin;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(MotionDynamicsPlugin);
    }
}
