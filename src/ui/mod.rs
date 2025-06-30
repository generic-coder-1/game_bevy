use bevy::{
    app::{Plugin, Startup},
    color::Color,
    ecs::{bundle::Bundle, component::Component, system::Commands},
    prelude::*,
    text::{TextColor, TextFont},
    ui::{
        widget::{Button, Text},
        AlignItems, BackgroundColor, BorderRadius, JustifyContent, Node, UiRect, Val,
    },
};
use main_menu::MainMenuPlugin;

mod main_menu;

#[derive(Default)]
pub struct UiPlugin;

impl UiPlugin {
    fn startup(mut commands: Commands) {}
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, Self::startup)
            .add_plugins((MainMenuPlugin,));
    }
}

#[derive(Component)]
struct MenuUi;

fn create_button<T: Component>(button_text: impl Into<String>, button_value: T) -> impl Bundle {
    (
        Button,
        Node {
            width: Val::Px(150.0),
            height: Val::Px(50.0),
            margin: UiRect::all(Val::Px(10.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        BackgroundColor(Color::linear_rgb(
            46.0 / 255.0,
            119.0 / 255.0,
            179.0 / 255.0,
        )),
        BorderRadius::all(Val::Px(10.0)),
        button_value,
        bevy::ecs::children![(
            Text::new(button_text),
            TextColor(Color::WHITE),
            TextFont {
                font_size: 40.0,
                ..Default::default()
            }
        )],
    )
}
