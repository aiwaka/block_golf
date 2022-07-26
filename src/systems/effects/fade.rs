use bevy::prelude::*;

use crate::components::effects::fade::FadeEffect;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

/// これで登録すれば勝手に動く
pub fn register_fade(commands: &mut Commands, delta: f32, color: Color) {
    if delta == 0.0 {
        panic!("cannot regist no-move fade");
    }
    let mut init_color = color;
    let init_alpha: f32 = if delta > 0.0 { 0.0 } else { 1.0 };
    init_color.set_a(init_alpha);
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: init_color,
                custom_size: Some(Vec2::new(SCREEN_WIDTH, SCREEN_HEIGHT)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 99.9)),
            ..Default::default()
        })
        .insert(FadeEffect {
            alpha: init_alpha,
            target_alpha: color.a(),
            delta,
            finished: false,
        });
}

fn update(mut fade_query: Query<(&mut FadeEffect, &mut Sprite)>) {
    if let Ok((mut fade, mut spr)) = fade_query.get_single_mut() {
        if !fade.finished {
            fade.alpha += fade.delta;
            if (fade.delta < 0.0 && fade.alpha <= fade.target_alpha)
                || (fade.delta > 0.0 && fade.alpha >= fade.target_alpha)
            {
                fade.alpha = fade.target_alpha;
                fade.finished = true;
            }
            spr.color.set_a(fade.alpha);
        }
    }
}

pub(super) struct FadeEffectPlugin;
impl Plugin for FadeEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update);
    }
}
