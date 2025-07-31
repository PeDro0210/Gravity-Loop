use bevy::{
    core_pipeline::core_3d::Camera3d,
    ecs::{
        query::With,
        system::{Commands, Query},
    },
    window::{CursorGrabMode, PrimaryWindow, Window},
};

pub fn mouse_constraints(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = window.single_mut().unwrap();

    // for not letting the mouse escape
    primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;

    // self explanatory
    primary_window.cursor_options.visible = false;
}
