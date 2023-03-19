/**
 * A simulation of the Solar System.
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 */
use bevy::{prelude::*, window::PrimaryWindow};

mod components;
use components::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Solar System".into(),
                resolution: (2000., 1400.).into(),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(spawn_system)
        .add_startup_system(spawn_camera)
        .add_system(rotate_sun)
        .add_system(revolve_mercury)
        .add_system(revolve_venus)
        .add_system(revolve_earth)
        .add_system(revolve_mars)
        .add_system(revolve_jupiter)
        .add_system(revolve_saturn)
        .add_system(revolve_uranus)
        .add_system(revolve_neptune)
        .add_system(revolve_pluto)
        .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
