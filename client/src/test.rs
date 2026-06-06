use bevy::prelude::*;
mod menu;
use menu::main_menu::*;



#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
enum AppState {
  #[default]
  MainMenu,
  LogIn,
  InGame,
  Paused,
}

impl AppState {
  fn next(&self) -> Self {
    match *self {
      AppState::MainMenu => AppState::LogIn,
      AppState::LogIn => AppState::InGame,
      AppState::InGame => AppState::Paused,
      AppState::Paused => AppState::InGame,
    }
  }
}

// fn setup(mut commands: Commands) {
//     commands.spawn(Camera2d);

//     commands
//         .spawn((
//             Button,
//             Node {
//                 width: px(200.0),
//                 height: px(65.0),
//                 justify_content: JustifyContent::Center,
//                 align_items: AlignItems::Center,
//                 ..default()
//             },
//             BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
//             MyButton,
//         ))
//         .with_children(|parent| {
//             parent.spawn(Text::new("Click Me"));
//         });
// }


// #[derive(Component)]
// struct MyButton;

fn button_system(
    query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
) {
    for interaction in &query {
        match interaction {
            Interaction::Pressed => {
                println!("Button clicked!");
            }
            Interaction::Hovered => {
                println!("Hovering");
            }
            Interaction::None => {}
        }
    }
}


use bevy::color::palettes::basic::GREEN;

fn hover_button(
  hover: On<Pointer<Over>>,
  mut commands: Commands,
) {
  commands
    .entity(hover.entity)
    .insert(BackgroundColor(GREEN.into()));

}

fn dont_hover_button(
  not_hover: On<Pointer<Out>>,
  mut commands: Commands,
) {
  commands
    .entity(not_hover.entity)
    .insert(BackgroundColor(Color::BLACK.into()));


}

fn spawn_button(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(button()).observe(hover_button)
                 .observe(dont_hover_button)    ;
        });
}

fn button() -> impl Bundle {
  (
    Button,
    Node {
      width: px(150),
      height: px(65),
      border: UiRect::all(px(5)),
      justify_content: JustifyContent::Center,
      align_items: AlignItems::Center,
      ..default()
    },
    BorderColor::all(Color::WHITE),
    BackgroundColor(Color::BLACK),
    children![(
      Text::new("Button"),
      TextColor(Color::srgb(0.9, 0.9, 0.9)),
      TextShadow::default(),
    )],
  )
}




// fn on_menu_button_pressed(
//   event: On<Pointer<Click>>,
//   ui: Res<Ui>,
//   mut next_state: ResMut<NextState<AppState>>,
// ) {
//   let Some(menu_entity) = ui.menu else {
//     return;
//   };

//   if event.event().entity == menu_entity {
//     info!("Start Game button pressed");
//     next_state.set(AppState::InGame);
//   }
// }


fn toggle_game_pause(
  mut next_state: ResMut<NextState<AppState>>,
  current_state: Res<State<AppState>>,
  input: Res<ButtonInput<KeyCode>>,
) {
  if input.just_pressed(KeyCode::Escape) {
    next_state.set(current_state.next());
  }
}



fn spawn_menu() {
  // Spawn a menu
}

fn despawn_menu() {
  // Despawn the menu
}

fn play_game() {
  // Play the game
}

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    // Add our state to our app definition
    .init_state::<AppState>()
    // We can add systems to trigger during transitions
    .add_systems(Startup, spawn_button)
    .add_systems(Update, button_system)
    .add_systems(OnEnter(AppState::MainMenu), spawn_menu)
    .add_systems(OnExit(AppState::MainMenu), despawn_menu)
    // Or we can use run conditions
    .add_systems(Update, play_game.run_if(in_state(AppState::InGame)))
    .run();
}




