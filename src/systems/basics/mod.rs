mod setup;

use bevy::app::{App, Startup};
use setup::mouse_constraints;

pub fn basics_systems(app: &mut App) {
    app.add_systems(Startup, mouse_constraints);
}
