use bevy::prelude::*;

mod game;

use game::GamePlugin;

fn main() {
    App::new().add_plugins((DefaultPlugins, GamePlugin)).run();
}
