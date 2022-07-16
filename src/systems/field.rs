use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub const FIELD_WIDTH: f32 = 960.0;
pub const FIELD_HEIGHT: f32 = 540.0;

fn set_field(mut commands: Commands, server: Res<AssetServer>) {
    let field_shape = shapes::Rectangle {
        extents: Vec2::new(FIELD_WIDTH, FIELD_HEIGHT),
        origin: RectangleOrigin::Center,
    };
    commands.spawn_bundle(GeometryBuilder::build_as(
        &field_shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::LIME_GREEN),
            outline_mode: StrokeMode::new(Color::BLACK, 10.0),
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 10.0),
            ..Default::default()
        },
    ));
}

pub struct FieldPlugin;
impl Plugin for FieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(set_field);
    }
}
