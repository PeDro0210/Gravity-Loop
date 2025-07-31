use bevy::{
    ecs::component::Component,
    math::Vec2,
    prelude::{Deref, DerefMut},
};

// Base componenet just for referencing ECS
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Debug, Component, DerefMut, Deref)]
pub struct CameraSensitivity(Vec2);
impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(Vec2::new(0.003, 0.002)) // a pretty arbitrary mouse sensitive, I can change it, but I
        // won't do it a settings feature, cause it will take to long
    }
}
