use bevy::prelude::*; 

pub mod main_menu;
pub mod splash;

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    Main,
    Settings,
    SettingsDisplay,
    SettingsSound,
    #[default]
    Disabled,
}