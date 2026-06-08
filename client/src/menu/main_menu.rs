use bevy::{color::palettes::css::CRIMSON, prelude::*};

// client/src/menu/main_menu.rs
use super::{MenuButtonAction, MenuState};

// `crate` = the library root (lib.rs) here
use crate::{NORMAL_BUTTON, TEXT_COLOR};

// This plugin manages the menu, with 5 different screens:
// - a main menu with "New Game", "Settings", "Quit"
// - a settings menu with two submenus and a back button

// // - two settings screen with a setting that can be set and a back button
pub fn plugin(app: &mut App) {
    // `MenuState` is registered by the umbrella `menu_plugin` in `mod.rs`.
    app.add_systems(OnEnter(MenuState::Main), main_menu_setup);
}

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
struct OnMainMenuScreen;

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Common style for all buttons on the screen

    let button_node = Node {
        width: px(300),
        height: px(65),
        margin: UiRect::all(px(20)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_node = Node {
        width: px(30),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: px(10),
        ..default()
    };
    let button_text_font = TextFont {
        font_size: 33.0,
        ..default()
    };

    let login_icon = asset_server.load("textures/Game Icons/right.png");
    let register_icon = asset_server.load("textures/Game Icons/right.png");
    let wrench_icon = asset_server.load("textures/Game Icons/wrench.png");
    let exit_icon = asset_server.load("textures/Game Icons/exitRight.png");

    commands.spawn((
        DespawnOnExit(MenuState::Main),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        OnMainMenuScreen,
        children![(
            Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(CRIMSON.into()),
            children![
                // Display the game name
                (
                    Text::new("ProjectR"),
                    TextFont {
                        font_size: 67.0,

                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                    Node {
                        margin: UiRect::all(px(50)),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ),
                // Display three buttons for each action available from the main menu:
                // - Login
                // - Register
                // - Settings
                // - Quit
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Login,
                    children![
                        (ImageNode::new(login_icon), button_icon_node.clone()),
                        (
                            Text::new("Login"),
                            button_text_font.clone(),
                            TextColor(TEXT_COLOR),
                        ),
                    ]
                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Register,
                    children![
                        (ImageNode::new(register_icon), button_icon_node.clone()),
                        (
                            Text::new("Register"),
                            button_text_font.clone(),
                            TextColor(TEXT_COLOR),
                        ),
                    ]
                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Settings,
                    children![
                        (ImageNode::new(wrench_icon), button_icon_node.clone()),
                        (
                            Text::new("Settings"),
                            button_text_font.clone(),
                            TextColor(TEXT_COLOR),
                        ),
                    ]
                ),
                (
                    Button,
                    button_node,
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Quit,
                    children![
                        (ImageNode::new(exit_icon), button_icon_node),
                        (Text::new("Quit"), button_text_font, TextColor(TEXT_COLOR),),
                    ]
                ),
            ]
        )],
    ));
}
