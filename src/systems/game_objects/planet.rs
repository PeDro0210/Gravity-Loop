// ! Debugging, just a plane for refericing if the 3d Camera is working well

use avian3d::prelude::{Collider, RigidBody};
use bevy::{
    asset::Assets,
    color::Color,
    core_pipeline::core_3d::Camera3d,
    ecs::system::{Commands, ResMut},
    math::{
        Vec3, VectorSpace,
        primitives::{Cuboid, Plane3d},
    },
    pbr::{DirectionalLight, MeshMaterial3d, StandardMaterial},
    render::mesh::{Mesh, Mesh3d, Meshable},
    transform::components::Transform,
};

use crate::components::base_components::planet::Planet;

pub fn planets_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // May Planets are a bit out of scoope, but loop planets sound cool (like a donut plataform)

    // Mostly of this stuff is just for debugging and trying out the character movement
    // Basic plane
    commands.spawn((
        Planet,
        Collider::cuboid(30., 0., 30.),
        RigidBody::Static,
        Mesh3d(meshes.add(Cuboid::new(30., 5., 30.))),
        MeshMaterial3d(materials.add(Color::srgb(0.9, 0.4, 0.9))),
        Transform::from_xyz(10., 0., 10.),
    ));

    commands.spawn((
        Planet,
        Collider::cuboid(30., 0., 30.),
        RigidBody::Static,
        Mesh3d(meshes.add(Cuboid::new(30., 5., 30.))),
        MeshMaterial3d(materials.add(Color::srgb(0.9, 0.4, 0.9))),
        Transform::from_xyz(30., 0., 40.),
    ));

    commands.spawn((
        Planet,
        Collider::cuboid(30., 0., 30.),
        RigidBody::Static,
        Mesh3d(meshes.add(Cuboid::new(30., 5., 30.))),
        MeshMaterial3d(materials.add(Color::srgb(0.9, 0.4, 0.9))),
        Transform::from_xyz(50., 0., 10.),
    ));

    //Debug Camera
    //commands.spawn((
    //    Camera3d::default(),
    //    Transform::from_xyz(15.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    //));
}
