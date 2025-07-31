mod planet;
mod player;
mod the_hoop;

use bevy::app::{App, Startup};
use planet::planets_setup;
use player::player_setup;

pub fn game_objects_systems(app: &mut App) {
    app.add_systems(Startup, (player_setup, planets_setup));
}
