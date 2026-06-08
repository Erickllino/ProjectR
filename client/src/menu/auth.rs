use bevy::{color::palettes::css::CRIMSON, prelude::*};
use super::{AuthState, AuthButtonAction};
use crate::{TEXT_COLOR, NORMAL_BUTTON};


pub fn plugin(app: &mut App) {
      app.init_state::<AuthState>()
          .add_systems(OnEnter(AuthState::Login), login_setup)
          .add_systems(OnEnter(AuthState::Register), register_setup);
      // depois: campos de texto, validação, chamada pro server/
}

#[derive(Component)]
struct OnLoginScreen;

fn login_setup(mut commands: Commands) {
    
    let user_name_field = Node {
        width: px(300),
        height: px(65),
        margin: UiRect::all(px(20)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let password_field = Node {
        width: px(300),
        height: px(65),
        margin: UiRect::all(px(20)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    
    let button_node = Node {
        width: px(200),
        height: px(65),
        margin: UiRect::all(px(20)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = (
        TextFont {
            font_size: 33.0,
            ..default()
        },
        TextColor(TEXT_COLOR),
    );

    commands.spawn((
        DespawnOnExit(AuthState::Login),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        OnLoginScreen,
        children![(
            Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(CRIMSON.into()),
            Children::spawn(SpawnIter(



                [
                    (AuthButtonAction::SubmitLogin, "Login"),
                    (AuthButtonAction::BackToMainMenu, "Back"),
                ]
                .into_iter()
                .map(move |(action, text)| {
                    (
                        Button,
                        button_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        action,
                        children![(Text::new(text), button_text_style.clone())],
                    )
                })
            ))
        )],
    ));
}

fn register_setup(mut commands: Commands) {
    // spawn UI elements for register screen    
}
