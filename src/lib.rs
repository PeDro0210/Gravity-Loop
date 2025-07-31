mod components;
mod systems;

use avian3d::PhysicsPlugins;
use bevy::prelude::*;
use systems::{basics::basics_systems, game_objects::game_objects_systems};

pub struct MainAppPlugin;

impl Plugin for MainAppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Gravity-Loop".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(game_objects_systems)
        .add_plugins(basics_systems);
    }
}
