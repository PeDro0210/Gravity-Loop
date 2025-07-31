use std::f32::consts::{FRAC_1_PI, PI};

use avian3d::{
    math::FRAC_PI_2,
    prelude::{Collider, RigidBody},
};
use bevy::{
    asset::Assets,
    color::Color,
    core_pipeline::core_3d::Camera3d,
    ecs::{
        query::With,
        system::{Commands, Res, ResMut, Single},
    },
    input::mouse::AccumulatedMouseMotion,
    math::{
        EulerRot, Quat, Vec2, Vec3,
        primitives::{Capsule3d, Cuboid, Plane3d},
    },
    pbr::{MeshMaterial3d, StandardMaterial},
    render::{
        camera::{Camera, PerspectiveProjection, Projection},
        mesh::{Mesh, Mesh3d, Meshable},
        view::Visibility,
    },
    scene::ron::de,
    transform::components::Transform,
};

use crate::components::base_components::player::{CameraSensitivity, Player, PlayerCamera};

// creating player "instance"
pub fn player_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("Player Setup");
    commands
        .spawn((
            Player, //Base Player
            RigidBody::Dynamic,
            Collider::capsule(3., 1.5),
            Transform::from_xyz(0.0, 5.0, 0.0), //World Position
            Mesh3d(meshes.add(Capsule3d::default().mesh().longitudes(10))),
            MeshMaterial3d(materials.add(Color::srgb(0.3, 0.4, 0.3))),
        ))
        .with_children(|parent| {
            parent.spawn((
                // Camera children
                PlayerCamera,
                Camera3d::default(),
                Camera {
                    order: 1,
                    ..Default::default()
                },
                CameraSensitivity::default(),
                Projection::from(PerspectiveProjection {
                    // 70 is a pretty vanilla FOV
                    fov: 70_f32.to_radians(), // I was a big idiot, didn't saw that this wasn't on
                    // radians
                    ..Default::default()
                }),
            ));
        });
}

// this implementation was taken from the bevy example page
pub fn move_player_camera(
    //for accelerating the camera rotation depending how you do it with the mouse
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player: Single<(&mut Transform, &CameraSensitivity), With<PlayerCamera>>,
) {
    let (mut transform, camera_sensitivity) = player.into_inner();

    let delta = accumulated_mouse_motion.delta;

    //TODO: if see neccesary, comment the things
    if delta != Vec2::ZERO {
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        // To not run into these issues, we clamp the pitch to a safe range.
        const PITCH_LIMIT: f32 = FRAC_PI_2 as f32 - 0.01;

        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}
