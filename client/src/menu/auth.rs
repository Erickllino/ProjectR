

use bevy::{color::palettes::css::CRIMSON, input_focus::{InputDispatchPlugin, InputFocus}, prelude::*, text};

use bevy_simple_text_input::{
    TextInput,TextInputValue, TextInputPlugin, TextInputInactive, TextInputPlaceholder, TextInputTextColor,
    TextInputTextFont,TextInputSettings, TextInputSubmitMessage,
};

use shared::{ LoginRequest, AuthResponse};

use super::{AuthState, MenuState,GameState, AuthButtonAction};
use crate::{TEXT_COLOR, NORMAL_BUTTON};

const BORDER_COLOR_ACTIVE: Color = Color::srgb(0.75, 0.52, 0.99);
const BORDER_COLOR_INACTIVE: Color = Color::srgb(0.25, 0.25, 0.25);
const BACKGROUND_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);

pub fn plugin(app: &mut App) {
      app.init_state::<AuthState>()
          .add_plugins((InputDispatchPlugin, TextInputPlugin))
          .add_systems(OnEnter(AuthState::Login), login_setup)
          .add_systems(OnEnter(AuthState::Register), register_setup)
          .add_systems(
            Update,
            (auth_action, auth_enter, focus).run_if(in_state(GameState::Menu)),
        )
        ;
      // depois: campos de texto, validação, chamada pro server/
}

#[derive(Component)]
struct OnLoginScreen;

#[derive(Component)]
pub struct UsernameField;
fn username_input(placeholder_hide_on_focus: bool) -> impl Bundle {
    (
        Node {
            width: Val::Px(400.0),
            height: px(65),
            border: UiRect::all(Val::Px(5.0)),
            padding: UiRect::all(Val::Px(5.0)),
            margin: UiRect::all(px(20)),
            ..default()
        },
        BorderColor::all(BORDER_COLOR_INACTIVE),
        BackgroundColor(BACKGROUND_COLOR),
        TextInput,
        TextInputTextFont(TextFont {
            font_size: 34.,
            ..default()
        }),
        TextInputTextColor(TextColor(TEXT_COLOR)),
        TextInputPlaceholder {
            value: "Username".to_string(),
            hide_on_focus: placeholder_hide_on_focus,
            ..default()
        },
        TextInputInactive(true),
        UsernameField
    )
}

#[derive(Component)]
pub struct PasswordField;

fn password_input(placeholder_hide_on_focus: bool) -> impl Bundle {
    (
        Node {
            width: Val::Px(350.0),
            height: px(65),
            border: UiRect::all(Val::Px(5.0)),
            padding: UiRect::all(Val::Px(5.0)),
            margin: UiRect::all(px(10)),
            ..default()
        },
        BorderColor::all(BORDER_COLOR_INACTIVE),
        BackgroundColor(BACKGROUND_COLOR),
        TextInput,
        TextInputTextFont(TextFont {
            font_size: 34.,
            ..default()
        }),
        TextInputTextColor(TextColor(TEXT_COLOR)),
        TextInputPlaceholder {
            value: "Password".to_string(),
            hide_on_focus: placeholder_hide_on_focus,
            ..default()
        },
        TextInputSettings {
                    mask_character: Some('*'),
                    retain_on_submit: true,
                    // We're configuring this value in the example to demonstrate the
                    // functionality, but you probably don't want to limit the length
                    // of passwords.
                    max_length: Some(12),
        },
        TextInputInactive(true),
        PasswordField
    )
}

fn submit_button(text: &str, aba: AuthButtonAction) -> impl Bundle {
    let button_node = Node {
        width: px(200),
        height: px(65),
        margin: UiRect::all(px(20)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    (
        Button,
        button_node,
        BackgroundColor(NORMAL_BUTTON),
        aba,
        children![(
            Text::new(text),
            TextFont {
                font_size: 33.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
        )],
    )
}

fn login_setup(mut commands: Commands) {
    
   
    commands
        .spawn((
            DespawnOnExit(AuthState::Login),
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                ..default()
            },
            OnLoginScreen,
        ))
        .observe(background_node_click)
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: percent(100),
                        height: percent(100),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(CRIMSON.into()),
                ))
                .with_children(|parent| {
                    
                    parent.spawn(username_input(true)).observe(text_input_click);

                    parent.spawn(password_input(true)).observe(text_input_click);

                    // Botão de Login
                    parent
                        .spawn(submit_button("Login", AuthButtonAction::SubmitLogin));

                    // Botão de Back
                    parent
                        .spawn(submit_button("Back", AuthButtonAction::BackToMainMenu));
                });
        });
}

#[derive(Component)]
struct OnRegisterScreen;


fn register_setup(mut commands: Commands) {
    
   
    commands
        .spawn((
            DespawnOnExit(AuthState::Register),
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnRegisterScreen,
        ))
        .observe(background_node_click)
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: percent(100),
                        height: percent(100),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(CRIMSON.into()),
                ))
                .with_children(|parent| {
                    
                    parent.spawn(username_input(true)).observe(text_input_click);

                    parent.spawn(password_input(true)).observe(text_input_click);

                    // Botão de Register
                    parent
                        .spawn(submit_button("Login", AuthButtonAction::SubmitRegister));

                    // Botão de Back
                    parent
                        .spawn(submit_button("Back", AuthButtonAction::BackToMainMenu));
                });
        });
}


pub fn auth_action(
    interaction_query: Query<
        (&Interaction, &AuthButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    username_q: Query<&TextInputValue, With<UsernameField>>,
    password_q: Query<&TextInputValue, With<PasswordField>>,

    // mut app_exit_writer: MessageWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut auth_state: ResMut<NextState<AuthState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                AuthButtonAction::BackToMainMenu => {
                    menu_state.set(MenuState::Main);
                    auth_state.set(AuthState::Disabled);
                }
                action => {
                    let (Ok(username), Ok(password)) = (username_q.single(), password_q.single()) else {
                        warn!("login fields not found");
                        return;
                    };

                    do_submit(
                        action,
                        &username.0,
                        &password.0,
                        &mut game_state,
                        &mut auth_state,
                        &mut menu_state,
                    );
                }
            }
        }
    }
}

/// Shared submit logic for both the buttons (`auth_action`) and the Enter key
/// (`auth_enter`). Decides what to do based on the action, then transitions
/// the game state only if the server says the login succeeded.
fn do_submit(
    action: &AuthButtonAction,
    username: &str,
    password: &str,
    game_state: &mut ResMut<NextState<GameState>>,
    auth_state: &mut ResMut<NextState<AuthState>>,
    menu_state: &mut ResMut<NextState<MenuState>>,
) {
    match action {
        // Register still goes through check_login for now — swap to check_register
        // once the server has a /register route.
        AuthButtonAction::SubmitLogin | AuthButtonAction::SubmitRegister => {
            match check_login(username, password) {
                AuthResponse::Success => {
                    game_state.set(GameState::Game);
                    auth_state.set(AuthState::Disabled);
                    menu_state.set(MenuState::Disabled);
                }
                other => warn!("login failed: {other:?}"),
            }
        }
        AuthButtonAction::BackToMainMenu => {}
    }
}

/// Submits when the player presses Enter inside a focused text box.
/// The submit message tells us *that* Enter was pressed; we read both fields
/// ourselves and pick login vs register from the current `AuthState`.
fn auth_enter(
    mut submits: MessageReader<TextInputSubmitMessage>,
    username_q: Query<&TextInputValue, With<UsernameField>>,
    password_q: Query<&TextInputValue, With<PasswordField>>,
    auth_state_now: Res<State<AuthState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut auth_state: ResMut<NextState<AuthState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    // Nothing typed-and-entered this frame.
    if submits.is_empty() {
        return;
    }
    // Consume every pending submit so one Enter triggers exactly one submit.
    submits.clear();

    let (Ok(username), Ok(password)) = (username_q.single(), password_q.single()) else {
        return;
    };

    let action = match auth_state_now.get() {
        AuthState::Login => AuthButtonAction::SubmitLogin,
        AuthState::Register => AuthButtonAction::SubmitRegister,
        _ => return,
    };

    do_submit(
        &action,
        &username.0,
        &password.0,
        &mut game_state,
        &mut auth_state,
        &mut menu_state,
    );
}


fn check_login(username: &str, password: &str) -> AuthResponse {
    let msg = LoginRequest {
        username: username.to_string(),
        password: password.to_string(),
    };

    match ureq::post("http://127.0.0.1:3000/login").send_json(&msg) {
        Ok(mut response) => response
            .body_mut()
            .read_json::<AuthResponse>()
            .unwrap_or(AuthResponse::InvalidCredentials),
        Err(e) => {
            warn!("could not reach server: {e}");
            AuthResponse::InvalidCredentials
        }
    }
}




fn focus(
    focus: Res<InputFocus>,
    mut text_inputs: Query<(Entity, &mut TextInputInactive, &mut BorderColor)>,
) {
    if !focus.is_changed() {
        return;
    }

    for (entity, mut inactive, mut border_color) in text_inputs.iter_mut() {
        if focus.0 == Some(entity) {
            inactive.0 = false;
            *border_color = BORDER_COLOR_ACTIVE.into();
        } else {
            inactive.0 = true;
            *border_color = BORDER_COLOR_INACTIVE.into();
        }
    }
}

fn background_node_click(mut trigger: On<Pointer<Click>>, mut focus: ResMut<InputFocus>) {
    focus.0 = None;
    trigger.propagate(false);
}

fn text_input_click(mut trigger: On<Pointer<Click>>, mut focus: ResMut<InputFocus>) {
    focus.0 = Some(trigger.event_target());
    trigger.propagate(false);
}