use bevy::prelude::Component;

#[derive(Component)]
pub struct PhysicMaterial {
    pub restitution: f32, // 反発係数
    pub density: f32,     // 密度
    pub friction: f32,    // 摩擦係数
}

impl PhysicMaterial {
    pub fn new(restitution: f32, density: f32, friction: f32) -> Self {
        Self {
            restitution,
            density,
            friction,
        }
    }
}
