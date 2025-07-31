mod components;
mod systems;

use bevy::prelude::*;
use systems::game_objects::game_objects_systems;

pub struct MainAppPlugin;

impl Plugin for MainAppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins(game_objects_systems);
    }
}
