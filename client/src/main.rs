//! This example will display a simple menu using Bevy UI where you can start a new game,
//! change some settings or quit. There is no actual game, it will just display the current
//! settings for 5 seconds before going back to the menu.


use bevy::prelude::*;
use bevy::audio::Volume;

use client::{DisplayQuality, GameState, VolumeSetting};
use client::menu::menu_plugin;
use client::menu::splash::*;

mod game;
use game::test_game::game_plugin;  
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Insert as resource the initial value for the settings resources
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(VolumeSetting(7))
        .insert_resource(GlobalVolume::new(Volume::Linear(VolumeSetting::default().0 as f32 / 9.0)))
        // Declare the game state, whose starting value is determined by the `Default` trait
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        // Adds the plugins for each state
        .add_plugins((splash_plugin, menu_plugin, game_plugin))
        .run(); 
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}





