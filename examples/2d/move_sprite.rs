//! Renders a 2D scene containing a single, moving sprite.

use std::time::Duration;

use bevy::prelude::*;
use bevy_internal::{winit::WinitSettings, core_pipeline::clear_color::ClearColorConfig};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .insert_resource(WinitSettings {
        //     focused_mode:bevy::winit::UpdateMode::ReactiveLowPower {
        //         max_wait: Duration::MAX,
        //     },
        //     unfocused_mode: bevy::winit::UpdateMode::ReactiveLowPower {
        //         max_wait: Duration::MAX,
        //     },
        //     ..default()
        // })
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Default,
        },
        ..default()
    });
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("branding/icon.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        Direction::Up,
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("branding/icon.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
    ));
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Up => transform.translation.y += 350. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 350. * time.delta_seconds(),
        }

        if transform.translation.y > 800. {
            *logo = Direction::Down;
        } else if transform.translation.y < -800. {
            *logo = Direction::Up;
        }
    }
}
