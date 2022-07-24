use bevy::prelude::*;

use crate::components::effects::fade::FadeEffect;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

/// これで登録すれば勝手に動く
fn register(commands: &mut Commands, delta: f32) {
    if delta == 0.0 {
        panic!("cannot regist no-move fade");
    }
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(SCREEN_WIDTH, SCREEN_HEIGHT)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 99.9)),
            ..Default::default()
        })
        .insert(FadeEffect {
            alpha: if delta > 0.0 { 0.0 } else { 1.0 },
            delta,
            finished: false,
        });
}

fn update(mut fade_query: Query<(&mut FadeEffect, &mut Sprite)>) {
    if let Ok((mut fade, mut spr)) = fade_query.get_single_mut() {
        fade.alpha += fade.delta;
        if fade.delta < 0.0 && fade.alpha <= 0.0 {
            fade.alpha = 0.0;
            fade.delta = 0.0;
        } else if fade.delta > 0.0 && fade.alpha >= 1.0 {
            fade.alpha = 1.0;
            fade.delta = 0.0;
        }

        spr.color.set_a(fade.alpha);
    }
}

fn delete(mut commands: Commands, fade_query: Query<(&FadeEffect, Entity)>) {
    if let Ok((fade, ent)) = fade_query.get_single() {
        if (fade.delta < 0.0 && fade.alpha <= 0.0) || (fade.delta > 0.0 && fade.alpha >= 1.0) {
            commands.entity(ent).despawn();
        }
    }
}

pub(super) struct FadeEffectPlugin;
impl Plugin for FadeEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update);
        app.add_system(delete);
    }
}
