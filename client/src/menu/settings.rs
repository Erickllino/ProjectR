use super::{MenuState, SelectedOption, SettingsState, SettingsButtonAction};
use crate::{DisplayQuality, NORMAL_BUTTON, TEXT_COLOR, VolumeSetting};
use bevy::{color::palettes::css::CRIMSON,audio::Volume, prelude::*};




pub fn plugin(app: &mut App) {
    app.init_state::<SettingsState>()
        .add_systems(OnEnter(SettingsState::Menu), settings_menu_setup)
        .add_systems(OnEnter(SettingsState::Display), display_settings_menu_setup)
        .add_systems(OnEnter(SettingsState::Volume), sound_settings_menu_setup)
        .add_systems(
            Update,
            setting_button::<DisplayQuality>.run_if(in_state(SettingsState::Display)),
        )
        .add_systems(
            Update,
            setting_button::<VolumeSetting>.run_if(in_state(SettingsState::Volume)),
        )
        .add_systems(Update, update_volume)
        // Runs across the whole settings section so the "Back" buttons on the
        // Display/Sound sub-screens are handled too (button colors come from the
        // global `button_system` registered in `menu_plugin`).
        .add_systems(
            Update,
            settings_menu_action.run_if(in_state(MenuState::Settings)),
        );
}

#[derive(Component)]
struct OnSettingsMenuScreen;

// Tag component used to tag entities added on the display settings menu screen
#[derive(Component)]
struct OnDisplaySettingsMenuScreen;

// Tag component used to tag entities added on the sound settings menu screen
#[derive(Component)]
struct OnSoundSettingsMenuScreen;


fn settings_menu_setup(mut commands: Commands) {
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
        DespawnOnExit(SettingsState::Menu),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        OnSettingsMenuScreen,
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
                    (SettingsButtonAction::SettingsDisplay, "Display"),
                    (SettingsButtonAction::SettingsSound, "Sound"),
                    (SettingsButtonAction::BackToMainMenu, "Back"),
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

fn setting_button<T: Resource + Component + PartialEq + Copy>(
    interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
    selected_query: Single<(Entity, &mut BackgroundColor), With<SelectedOption>>,
    mut commands: Commands,
    mut setting: ResMut<T>,
) {
    let (previous_button, mut previous_button_color) = selected_query.into_inner();
    for (interaction, button_setting, entity) in &interaction_query {
        if *interaction == Interaction::Pressed && *setting != *button_setting {
            *previous_button_color = NORMAL_BUTTON.into();
            commands.entity(previous_button).remove::<SelectedOption>();
            commands.entity(entity).insert(SelectedOption);
            *setting = *button_setting;
        }
    }
}

fn display_settings_menu_setup(mut commands: Commands, display_quality: Res<DisplayQuality>) {
    fn button_node() -> Node {
        Node {
            width: px(200),
            height: px(65),
            margin: UiRect::all(px(20)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }
    }
    fn button_text_style() -> impl Bundle {
        (
            TextFont {
                font_size: 33.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
        )
    }

    let display_quality = *display_quality;
    commands.spawn((
        DespawnOnExit(SettingsState::Display),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        OnDisplaySettingsMenuScreen,
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
                // Create a new `Node`, this time not setting its `flex_direction`. It will
                // use the default value, `FlexDirection::Row`, from left to right.
                (
                    Node {
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(CRIMSON.into()),
                    Children::spawn((
                        // Display a label for the current setting
                        Spawn((Text::new("Display Quality"), button_text_style())),
                        SpawnWith(move |parent: &mut ChildSpawner| {
                            for quality_setting in [
                                DisplayQuality::Low,
                                DisplayQuality::Medium,
                                DisplayQuality::High,
                            ] {
                                let mut entity = parent.spawn((
                                    Button,
                                    Node {
                                        width: px(150),
                                        height: px(65),
                                        ..button_node()
                                    },
                                    BackgroundColor(NORMAL_BUTTON),
                                    quality_setting,
                                    children![(
                                        Text::new(format!("{quality_setting:?}")),
                                        button_text_style(),
                                    )],
                                ));
                                if display_quality == quality_setting {
                                    entity.insert(SelectedOption);
                                }
                            }
                        })
                    ))
                ),
                // Display the back button to return to the settings screen
                (
                    Button,
                    button_node(),
                    BackgroundColor(NORMAL_BUTTON),
                    SettingsButtonAction::BackToSettings,
                    children![(Text::new("Back"), button_text_style())]
                )
            ]
        )],
    ));
}

fn sound_settings_menu_setup(mut commands: Commands, volume: Res<VolumeSetting>) {
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

    let volume = *volume;
    let button_node_clone = button_node.clone();
    commands.spawn((
        DespawnOnExit(SettingsState::Volume),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        OnSoundSettingsMenuScreen,
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
                (
                    Node {
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(CRIMSON.into()),
                    Children::spawn((
                        Spawn((Text::new("Volume"), button_text_style.clone())),
                        SpawnWith(move |parent: &mut ChildSpawner| {
                            for volume_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
                                let mut entity = parent.spawn((
                                    Button,
                                    Node {
                                        width: px(30),
                                        height: px(65),
                                        ..button_node_clone.clone()
                                    },
                                    BackgroundColor(NORMAL_BUTTON),
                                    VolumeSetting(volume_setting),
                                ));
                                if volume == VolumeSetting(volume_setting) {
                                    entity.insert(SelectedOption);
                                }
                            }
                        })
                    ))
                ),
                (
                    Button,
                    button_node,
                    BackgroundColor(NORMAL_BUTTON),
                    SettingsButtonAction::BackToSettings,
                    children![(Text::new("Back"), button_text_style)]
                )
            ]
        )],
    ));
}

fn update_volume(volume: Res<VolumeSetting>, mut global_volume: ResMut<GlobalVolume>) {
    if volume.is_changed() {
        // Convert the discrete volume setting (0-9) to a linear volume (0.0-1.0)
        let linear_volume = (volume.0 as f32) / 9.0;
        global_volume.volume = Volume::Linear(linear_volume);
    }
}

pub fn settings_menu_action(
    interaction_query: Query<
        (&Interaction, &SettingsButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut settings_stage: ResMut<NextState<SettingsState>>,
) {
    for (interaction, settings_button_menu_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match settings_button_menu_action {
                SettingsButtonAction::SettingsDisplay => {
                    settings_stage.set(SettingsState::Display);
                }
                SettingsButtonAction::SettingsSound => {
                    settings_stage.set(SettingsState::Volume);
                }
                SettingsButtonAction::BackToMainMenu => {
                    menu_state.set(MenuState::Main);
                    settings_stage.set(SettingsState::Disabled);
                }   
                SettingsButtonAction::BackToSettings => {
                    settings_stage.set(SettingsState::Menu);
                }
            }
        }
    }
}