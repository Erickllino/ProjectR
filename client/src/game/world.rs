use bevy::{
    prelude::*,
};

const MAP_SIZE : Vec2 = Vec2::new(120.0, 20.0); //get resolution and set accordingly
const CAMERA_SIZE: Vec2 = Vec2::new(120.0, 80.0);


fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), setup);
}

fn setup(mut commands: Commands) {
    // Here we would spawn the entities for the game, but for now we will just spawn a text to show that we are in the game state
    commands.spawn(Text::from_section(
        "This is the game state",
        TextFont {
            font_size: 50.0,
            ..default()
        },
    ));
}