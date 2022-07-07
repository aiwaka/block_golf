mod components;
mod stages;
mod systems;

use bevy::{input::keyboard::keyboard_input_system, prelude::*};
use bevy_prototype_lyon::prelude::*;
use heron::prelude::*;

use systems::{ball::BallPlugin, field::FieldPlugin, setup::global_setup};

const SCREEN_WIDTH: f32 = 1280.0;
const SCREEN_HEIGHT: f32 = 720.0;

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        width: SCREEN_WIDTH,
        height: SCREEN_HEIGHT,
        title: "Block Golf".to_string(),
        ..Default::default()
    });
    app.add_plugins(DefaultPlugins);
    app.add_system(keyboard_input_system);
    app.add_plugin(ShapePlugin);
    // app.add_plugin(PhysicsPlugin::default());
    app.add_system(bevy::input::system::exit_on_esc_system);
    app.add_startup_system(global_setup);
    app.add_plugin(FieldPlugin);
    app.add_plugin(BallPlugin);
    app.run();
}

// fn spawn(mut commands: Commands) {
//     let shape = shapes::RegularPolygon {
//         sides: 6,
//         feature: shapes::RegularPolygonFeature::Radius(50.0),
//         ..shapes::RegularPolygon::default()
//     };
//     commands.spawn_bundle(OrthographicCameraBundle::new_2d());
//     commands
//         .spawn_bundle(GeometryBuilder::build_as(
//             &shape,
//             DrawMode::Outlined {
//                 fill_mode: FillMode::color(Color::CYAN),
//                 outline_mode: StrokeMode::new(Color::BLACK, 7.0),
//             },
//             Transform::default(),
//         ))
//         .insert(RigidBody::Dynamic)
//         .insert(CollisionShape::Sphere { radius: 50.0 })
//         .insert(Velocity::from_linear(Vec3::X * 2.0 + Vec3::Y * 4.0))
//         .insert(Acceleration::from_linear(Vec3::X * 1.0))
//         .insert(PhysicMaterial {
//             friction: 1.0,
//             density: 10.0,
//             ..Default::default()
//         })
//         .insert(RotationConstraints::lock())
//         .insert(
//             CollisionLayers::none()
//                 .with_group(Layer::Player)
//                 .with_mask(Layer::World),
//         );
// }

// Define your physics layers
#[derive(PhysicsLayer)]
enum Layer {
    World,
    Player,
    Enemies,
}
