//! Hijacking move_sprite.rs to repro bug
//! Repro steps:
//! - In WASM
//! - In a monitor that supports HDR
//! you can check that by running this command on the JS console:
//! console.log("SDR:", window.matchMedia("(dynamic-range: standard)").matches)
//! console.log("HDR:", window.matchMedia("(dynamic-range: high)").matches)
//! You should see 'true' printed on both
//! In order to compile and run, do (on MacOS at least):
//! cargo build --release --example move_sprite --target wasm32-unknown-unknown
//! wasm-bindgen --out-name wasm_example \
//! --out-dir examples/wasm/target \
//! --target web target/wasm32-unknown-unknown/release/examples/move_sprite.wasm && python3 -m http.server --directory examples/wasm
//! You can also check a build of this here: https://metinu.com/bevyblackshader/

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
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
        camera: Camera {
            hdr: true,
            ..default()
        },
        ..default()
    });
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("my_faulty_texture.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        Direction::Up,
    ));
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
        }

        if transform.translation.y > 200. {
            *logo = Direction::Down;
        } else if transform.translation.y < -200. {
            *logo = Direction::Up;
        }
    }
}
