//! Reproduces this bug: https://discord.com/channels/691052431525675048/742569353878437978/1303423780378644540
//! Repro steps: run this example, press any key, it will panic when the key is released

use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Component)]
struct MyComponent(KeyCode);

#[derive(Resource, Default, Debug, Deref, DerefMut)]
struct MyComponentIndex(HashMap<KeyCode, Entity>);

#[derive(Event)]
struct MyEvent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (trigger_on_remove_hook, debug_entities))
        .init_resource::<MyComponentIndex>()
        .add_event::<MyEvent>()
        .run();
}

fn setup(world: &mut World) {
    world.register_component_hooks::<MyComponent>().on_remove(
        |mut world, _entity, _component_id| {
            world.commands().spawn_empty();
        },
    );
}

fn trigger_on_remove_hook(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    my_component_q: Query<(Entity, &MyComponent)>,
) {
    for (entity, my_component) in &my_component_q {
        if !keys.pressed(my_component.0) {
            commands.entity(entity).despawn();
        }
    }

    for key in keys.get_just_pressed() {
        commands.spawn(MyComponent(*key));
    }
}

fn log_vec(vec: Vec<impl std::fmt::Display>) {
    let output = vec
        .iter()
        .map(|item| format!("{}", item))
        .collect::<Vec<_>>()
        .join(", ");
    println!("{}", output);
}

fn debug_entities(entities_q: Query<Entity>, mut last_entities: Local<Vec<Entity>>) {
    let all_entities: Vec<Entity> = entities_q.iter().collect();
    if all_entities != *last_entities {
        println!(">> Entities changed!");
        log_vec(all_entities.clone());
        *last_entities = all_entities;
    }
}
