use bevy::prelude::*;



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