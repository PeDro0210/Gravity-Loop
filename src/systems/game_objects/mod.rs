mod planet;
mod player;
mod the_hoop;

use bevy::app::{App, Startup, Update};
use planet::planets_setup;
use player::{move_player, move_player_camera, move_player_orientation, player_setup};

pub fn game_objects_systems(app: &mut App) {
    app.add_systems(Startup, (player_setup, planets_setup))
        .add_systems(
            Update,
            (move_player_camera, move_player, move_player_orientation),
        );
}
