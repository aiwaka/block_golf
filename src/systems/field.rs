use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

fn set_field(mut commands: Commands, server: Res<AssetServer>) {
    let field_shape = shapes::Rectangle {
        extents: Vec2::new(960.0, 540.0),
        origin: RectangleOrigin::Center,
    };
    commands.spawn_bundle(GeometryBuilder::build_as(
        &field_shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::LIME_GREEN),
            outline_mode: StrokeMode::new(Color::BLACK, 10.0),
        },
        Transform::default(),
    ));
}

pub struct FieldPlugin;
impl Plugin for FieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(set_field);
    }
}
