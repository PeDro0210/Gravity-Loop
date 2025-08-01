use std::{
    f32::consts::{FRAC_1_PI, PI},
    ops::MulAssign,
};

use avian3d::{
    math::FRAC_PI_2,
    prelude::{AngularVelocity, Collider, LinearVelocity, RigidBody},
};
use bevy::{
    asset::Assets,
    color::Color,
    core_pipeline::core_3d::Camera3d,
    ecs::{
        query::With,
        system::{Commands, Query, Res, ResMut, Single},
    },
    input::{ButtonInput, keyboard::KeyCode, mouse::AccumulatedMouseMotion},
    math::{
        DVec3, EulerRot, Quat, Vec2, Vec3,
        primitives::{Capsule3d, Cuboid, Plane3d},
    },
    pbr::{MeshMaterial3d, StandardMaterial},
    render::{
        camera::{Camera, PerspectiveProjection, Projection},
        mesh::{Mesh, Mesh3d, Meshable},
        view::Visibility,
    },
    scene::ron::de,
    time::Time,
    transform::components::Transform,
};

use crate::components::{
    base_components::player::{CameraSensitivity, Player, PlayerCamera},
    resources::PlayerCarriedAcceleration,
};

// creating player "instance"
pub fn player_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("Player Setup");

    // for the acceleration and having a single reference
    commands.init_resource::<PlayerCarriedAcceleration>();

    commands
        // the main is the physics related componenets
        .spawn((
            Player, //Base Player
            RigidBody::Dynamic,
            CameraSensitivity::default(),
            Collider::cuboid(5., 10., 10.),
            // Big mesh for for face
            Transform::from_xyz(0.0, 5.0, 0.0), //World Position
            Mesh3d(meshes.add(Capsule3d::default().mesh().longitudes(10))),
            MeshMaterial3d(materials.add(Color::srgb(0.3, 0.4, 0.3))),
        ))
        .with_children(|parent| {
            parent.spawn((
                // Camera children
                PlayerCamera,
                CameraSensitivity::default(),
                Camera3d::default(),
                Camera {
                    order: 1,
                    ..Default::default()
                },
                Projection::from(PerspectiveProjection {
                    // 70 is a pretty vanilla FOV
                    fov: 70_f32.to_radians(), // I was a big idiot, didn't saw that this wasn't on
                    // radians
                    ..Default::default()
                }),
            ));
        });
}

// First I'll implement it without the micro-gravity in mine, then I'll do it

// Thx sburris
// Okay this has to be pretty refine
pub fn move_player(
    mut commands: Commands,
    time: Res<Time>,
    mut acceleration_res: ResMut<PlayerCarriedAcceleration>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Single<(&mut Transform, &mut LinearVelocity), With<Player>>,
) {
    let (mut transform, mut velocity) = player.into_inner();

    let mut velocity = velocity.as_vec3();

    // for handleling orientaiton
    let local_z = transform.local_z();

    // cool math that I won't in to detail
    let forward = -Vec3::new(local_z.x, 0., local_z.z);
    let right = Vec3::new(local_z.z, 0., -local_z.x);

    // Needs a more abstract implementation

    let mut acceleration = acceleration_res.into_inner();

    println!("acceleraation {}", acceleration.0);

    // TODO: refactor this for having multi key board input

    // just for checking out the physics
    match keyboard_input {
        ref val if keyboard_input.pressed(KeyCode::KeyW) => velocity += forward * acceleration.0,
        ref val if keyboard_input.pressed(KeyCode::KeyS) => velocity -= forward * acceleration.0,
        ref val if keyboard_input.pressed(KeyCode::KeyA) => velocity -= right * acceleration.0,
        ref val if keyboard_input.pressed(KeyCode::KeyD) => velocity += right * acceleration.0,
        _ => {
            // in the case not an input is being pressed, the body starts to decelrate
            if (acceleration.0 < 1.0) {
                acceleration.0 = 1.;
            } else {
                acceleration.0 -= 0.5;
            }
        }
    }

    if keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyD, KeyCode::KeyS]) {
        acceleration.0 += 0.5;
        // some acceleration factor, for having cool effect
        velocity = velocity.normalize_or_zero() * acceleration.0;

        transform.translation += velocity * time.delta_secs() * 2.0;
    }
}

// just controlls the orientation, might change for the micro-gravity implementation
pub fn move_player_orientation(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player_camera: Single<(&mut Transform, &CameraSensitivity), With<Player>>,
) {
    let (mut transform, camera_sensitivity) = player_camera.into_inner();

    let delta = accumulated_mouse_motion.delta;

    // this part just controlls how the camera moves, but not the player
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

// controls all perspecive camera
// this implementation was taken from the bevy example page
pub fn move_player_camera(
    //for accelerating the camera rotation depending how you do it with the mouse
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player_camera: Single<(&Transform, &CameraSensitivity), With<PlayerCamera>>,
) {
    let (mut transform, camera_sensitivity) = player_camera.into_inner();

    let delta = accumulated_mouse_motion.delta;

    // this part just controlls how the camera moves, but not the player
    if delta != Vec2::ZERO {
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        // To not run into these issues, we clamp the pitch to a safe range.
        const PITCH_LIMIT: f32 = FRAC_PI_2 as f32 - 0.01;

        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        // not messing with transformation, cause it will affect the player movement
    }
}
