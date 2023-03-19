/**
 * A simulation of the Solar System.
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 */
use bevy::{prelude::*, window::PrimaryWindow};

mod components;
mod systems;

use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Solar System".into(),
                resolution: (1800., 1000.).into(),
                canvas: Some("canvas".to_string()),
                fit_canvas_to_parent: false,
                prevent_default_event_handling: false,
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_startup_system(spawn_system)
        .add_startup_system(spawn_title)
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

pub fn spawn_title(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn(
        TextBundle::from_section(
            "Solar System",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(25.),
                left: Val::Px(window.width() / 2. - 70.),
                ..default()
            },
            ..default()
        }),
    );

    commands.spawn(
        TextBundle::from_section(
            "Afaan Bilal | afaan.dev",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.,
                color: Color::GRAY,
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(70.),
                left: Val::Px(window.width() / 2. - 80.),
                ..default()
            },
            ..default()
        }),
    );
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
        ..default()
    });
}
