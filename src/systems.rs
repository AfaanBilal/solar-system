/**
 * A simulation of the Solar System.
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 */
use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::*;

pub fn spawn_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0)
                .with_scale(Vec3::new(0.25, 0.25, 1.0)),
            texture: asset_server.load("sprites/sun.png"),
            ..default()
        },
        Sun {},
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                window.width() / 2.0 - 300.0,
                window.height() / 2.0,
                0.0,
            )
            .with_scale(Vec3::new(0.01, 0.01, 1.0)),
            texture: asset_server.load("sprites/mercury.png"),
            ..default()
        },
        Mercury {},
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                window.width() / 2.0 - 350.0,
                window.height() / 2.0,
                0.0,
            )
            .with_scale(Vec3::new(0.025, 0.025, 1.0)),
            texture: asset_server.load("sprites/venus.png"),
            ..default()
        },
        Venus {},
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                window.width() / 2.0 - 400.0,
                window.height() / 2.0,
                0.0,
            )
            .with_scale(Vec3::new(0.025, 0.025, 1.0)),
            texture: asset_server.load("sprites/earth.png"),
            ..default()
        },
        Earth {},
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                window.width() / 2.0 - 450.0,
                window.height() / 2.0,
                0.0,
            )
            .with_scale(Vec3::new(0.02, 0.02, 1.0)),
            texture: asset_server.load("sprites/mars.png"),
            ..default()
        },
        Mars {},
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                window.width() / 2.0 - 500.0,
                window.height() / 2.0,
                0.0,
            )
            .with_scale(Vec3::new(0.06, 0.06, 1.0)),
            texture: asset_server.load("sprites/jupiter.png"),
            ..default()
        },
        Jupiter {},
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                window.width() / 2.0 - 550.0,
                window.height() / 2.0,
                0.0,
            )
            .with_scale(Vec3::new(0.04, 0.04, 1.0)),
            texture: asset_server.load("sprites/saturn.png"),
            ..default()
        },
        Saturn {},
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                window.width() / 2.0 - 600.0,
                window.height() / 2.0,
                0.0,
            )
            .with_scale(Vec3::new(0.035, 0.035, 1.0)),
            texture: asset_server.load("sprites/uranus.png"),
            ..default()
        },
        Uranus {},
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                window.width() / 2.0 - 650.0,
                window.height() / 2.0,
                0.0,
            )
            .with_scale(Vec3::new(0.035, 0.035, 1.0)),
            texture: asset_server.load("sprites/neptune.png"),
            ..default()
        },
        Neptune {},
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                window.width() / 2.0 - 700.0,
                window.height() / 2.0,
                0.0,
            )
            .with_scale(Vec3::new(0.005, 0.005, 1.0)),
            texture: asset_server.load("sprites/pluto.png"),
            ..default()
        },
        Pluto {},
    ));
}

pub fn rotate_sun(mut q: Query<&mut Transform, With<Sun>>) {
    if let Ok(mut t) = q.get_single_mut() {
        t.rotate(Quat::from_axis_angle(Vec3::Z, 0.005));
    }
}

pub fn revolve_mercury(
    mut q: Query<&mut Transform, With<Mercury>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    if let Ok(mut t) = q.get_single_mut() {
        t.rotate(Quat::from_axis_angle(Vec3::Z, 0.01));
        t.rotate_around(
            Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0),
            Quat::from_axis_angle(Vec3::Z, 0.01),
        );
    }
}

pub fn revolve_venus(
    mut q: Query<&mut Transform, With<Venus>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    if let Ok(mut t) = q.get_single_mut() {
        t.rotate(Quat::from_axis_angle(Vec3::Z, 0.01));
        t.rotate_around(
            Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0),
            Quat::from_axis_angle(Vec3::Z, 0.007),
        );
    }
}

pub fn revolve_earth(
    mut q: Query<&mut Transform, With<Earth>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    if let Ok(mut t) = q.get_single_mut() {
        t.rotate(Quat::from_axis_angle(Vec3::Z, 0.01));
        t.rotate_around(
            Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0),
            Quat::from_axis_angle(Vec3::Z, 0.006),
        );
    }
}

pub fn revolve_mars(
    mut q: Query<&mut Transform, With<Mars>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    if let Ok(mut t) = q.get_single_mut() {
        t.rotate(Quat::from_axis_angle(Vec3::Z, 0.01));
        t.rotate_around(
            Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0),
            Quat::from_axis_angle(Vec3::Z, 0.005),
        );
    }
}

pub fn revolve_jupiter(
    mut q: Query<&mut Transform, With<Jupiter>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    if let Ok(mut t) = q.get_single_mut() {
        t.rotate(Quat::from_axis_angle(Vec3::Z, 0.01));
        t.rotate_around(
            Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0),
            Quat::from_axis_angle(Vec3::Z, 0.004),
        );
    }
}

pub fn revolve_saturn(
    mut q: Query<&mut Transform, With<Saturn>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    if let Ok(mut t) = q.get_single_mut() {
        t.rotate(Quat::from_axis_angle(Vec3::Z, 0.01));
        t.rotate_around(
            Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0),
            Quat::from_axis_angle(Vec3::Z, 0.003),
        );
    }
}

pub fn revolve_uranus(
    mut q: Query<&mut Transform, With<Uranus>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    if let Ok(mut t) = q.get_single_mut() {
        t.rotate(Quat::from_axis_angle(Vec3::Z, 0.01));
        t.rotate_around(
            Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0),
            Quat::from_axis_angle(Vec3::Z, 0.002),
        );
    }
}

pub fn revolve_neptune(
    mut q: Query<&mut Transform, With<Neptune>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    if let Ok(mut t) = q.get_single_mut() {
        t.rotate(Quat::from_axis_angle(Vec3::Z, 0.01));
        t.rotate_around(
            Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0),
            Quat::from_axis_angle(Vec3::Z, 0.001),
        );
    }
}

pub fn revolve_pluto(
    mut q: Query<&mut Transform, With<Pluto>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    if let Ok(mut t) = q.get_single_mut() {
        t.rotate(Quat::from_axis_angle(Vec3::Z, 0.01));
        t.rotate_around(
            Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0),
            Quat::from_axis_angle(Vec3::Z, 0.0005),
        );
    }
}
