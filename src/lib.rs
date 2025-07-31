mod components;
mod systems;

use bevy::prelude::*;

pub struct MainAppPlugin;

impl Plugin for MainAppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins(general_plugin)
            .add_plugins(specific_plugin);
    }
}
