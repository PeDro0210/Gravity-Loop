use avian3d::prelude::{Collider, RigidBody};
use bevy::{
    core_pipeline::core_3d::Camera3d, ecs::system::Commands, transform::components::Transform,
};

// creating player "instance"
pub fn player_setup(mut commands: Commands) {
    commands.spawn((
        RigidBody::Dynamic,
        Collider::capsule(0.5, 1.5),
        Transform::from_xyz(0.0, 3.0, 0.0),
    ));
}
