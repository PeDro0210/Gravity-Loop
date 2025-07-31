// ! Debugging, just a plane for refericing if the 3d Camera is working well

use bevy::{
    asset::Assets,
    color::Color,
    core_pipeline::core_3d::Camera3d,
    ecs::system::{Commands, ResMut},
    math::{Vec3, VectorSpace, primitives::Plane3d},
    pbr::{DirectionalLight, MeshMaterial3d, StandardMaterial},
    render::mesh::{Mesh, Mesh3d, Meshable},
    transform::components::Transform,
};

pub fn planets_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Basic plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(30.0, 30.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.4, 0.3))),
    ));

    // Light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(15.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
