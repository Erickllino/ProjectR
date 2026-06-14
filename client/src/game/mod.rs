pub mod test_game;

use crate::GameState;


use bevy::prelude::*;


fn plugin(app: &mut App) {
    app.add_plugins(test_game::game_plugin);
}