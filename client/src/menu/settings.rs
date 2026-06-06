  use bevy::prelude::*;
  use super::MenuState;
  use crate::{DisplayQuality, Volume};

  pub fn plugin(app: &mut App) {
      app.add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
          .add_systems(OnEnter(MenuState::SettingsDisplay),
  display_settings_menu_setup)
          .add_systems(OnEnter(MenuState::SettingsSound),
  sound_settings_menu_setup)
          .add_systems(Update,
  setting_button::<DisplayQuality>.run_if(in_state(MenuState::SettingsDisplay)))
          .add_systems(Update,
  setting_button::<Volume>.run_if(in_state(MenuState::SettingsSound)));
  }
