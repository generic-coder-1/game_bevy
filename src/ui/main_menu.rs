use bevy::prelude::*;

use crate::AppState;

use super::create_button;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
            .add_systems(
                Update,
                handle_main_menu_interaction.run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(OnExit(AppState::MainMenu), cleanup_main_menu);
    }
}

#[derive(Component)]
struct MainMenuUi;

#[derive(Component, Clone, Copy)]
enum MainMenuButton {
    Start,
    Quit,
}

fn cleanup_main_menu(root: Query<Entity, With<MainMenuUi>>, mut commands: Commands) {
    let root_node = root.single_inner().expect("Main Menu was set up");
    commands.entity(root_node).despawn();
}

fn handle_main_menu_interaction(
    interaction: Query<(&Interaction, &MainMenuButton), With<Button>>,
    mut state: ResMut<NextState<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    interaction.iter().for_each(|(interaction, button)| {
        if *interaction == Interaction::Pressed {
            match button {
                MainMenuButton::Start => state.set(AppState::Editing),
                MainMenuButton::Quit => {
                    exit.write(AppExit::Success);
                }
            }
        }
    });
}

fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            //whole screen
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            MainMenuUi,
        ))
        .with_children(|parent| {
            //title
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(150.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                BorderRadius::all(Val::Px(0.0)),
                //BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                bevy::ecs::children![(
                    Text::new("Bevy Test"),
                    TextColor(Color::WHITE),
                    TextFont {
                        font_size: 100.0,
                        ..Default::default()
                    }
                )],
            ));
            //buttons
            parent.spawn((
                Node {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                children![
                    create_button("Start", MainMenuButton::Start),
                    create_button("Quit", MainMenuButton::Quit),
                ],
            ));
        });
}
