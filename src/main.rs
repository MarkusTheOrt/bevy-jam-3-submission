use bevy::{
    prelude::*,
    window::WindowResolution,
};

const WINDOW_TITLE: &str = "MarkusTheOrt's Bevy-Jam-3 Game!";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: WINDOW_TITLE.to_owned(),
                resizable: false,
                resolution: WindowResolution::new(1280.0, 720.0),
                ..default()
            }),
            ..default()
        }))
        .run();
}
