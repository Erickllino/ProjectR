use bevy::prelude::*; 

pub mod main_menu;
pub mod auth;
pub mod settings;
pub mod splash;

use super::{GameState, NORMAL_BUTTON, HOVERED_PRESSED_BUTTON, HOVERED_BUTTON, PRESSED_BUTTON};


// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    Main,
    Settings,
    Auth,
    #[default]
    Disabled,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum SettingsState {
    Menu,
    Display,
    Volume,
    #[default]
    Disabled,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AuthState {
    Login,
    Register,
    #[default]
    Disabled,
}


#[derive(Component)]
pub enum MenuButtonAction {
    Login,
    Register,
    Settings,
    Quit,
}

#[derive(Component)]
pub enum SettingsButtonAction {  
    SettingsDisplay,
    SettingsSound,
    BackToMainMenu,
    BackToSettings,
}

#[derive(Component)]
pub enum AuthButtonAction {
    BackToMainMenu,
    SubmitLogin,
    SubmitRegister,
}


pub fn menu_plugin(app: &mut App) {
    app.init_state::<MenuState>()
        .add_systems(OnEnter(GameState::Menu), menu_setup)
        .add_plugins((main_menu::plugin, auth::plugin, settings::plugin))
        .add_systems(
            Update,
            (menu_action, button_system).run_if(in_state(GameState::Menu)),
        );
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}


#[derive(Component)]
pub struct SelectedOption;

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color, selected) in &mut interaction_query {
        *background_color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}


pub fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_writer: MessageWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut settings_state: ResMut<NextState<SettingsState>>,
    mut auth_state: ResMut<NextState<AuthState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_writer.write(AppExit::Success);
                }
                MenuButtonAction::Login => {
                    menu_state.set(MenuState::Auth);
                    auth_state.set(AuthState::Login);
                }
                MenuButtonAction::Register => {
                    menu_state.set(MenuState::Auth);
                    auth_state.set(AuthState::Register);
                }
                MenuButtonAction::Settings => {
                    menu_state.set(MenuState::Settings);
                    settings_state.set(SettingsState::Menu);
                }
            }
        }
    }
}

