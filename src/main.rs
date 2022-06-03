use bevy::prelude::*;
use bevy::math::vec3;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(load_dog)
        .add_system(dog_movement)
        .run();
}

#[derive(Component)]
struct Dog;

#[derive(Component, Deref, DerefMut)]
struct Position(Vec3);

fn load_dog(
    mut commands: Commands,
    server: Res<AssetServer>
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let handle: Handle<Image> = server.load("dog-png-30.png");
    commands
        .spawn()
        .insert(Dog)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                scale: vec3(1.0, 1.0, 0.0),
                translation: vec3(0.0, 0.0, 0.0),
                ..default()
            },
            texture: handle,
            ..default()
        });
}

fn dog_movement(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Dog>>
) {
    let mut dog_transform = query.single_mut();
    let mut direction = vec3(0.0, 0.0, 0.0);

    if keys.pressed(KeyCode::A) {
        direction.x += -1.0;
    }
    if keys.pressed(KeyCode::D) {
        direction.x += 1.0;
    }
    if keys.pressed(KeyCode::W) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::S) {
        direction.y += -1.0;
    }

    if keys.pressed(KeyCode::LShift) {
        direction.x *= 3.0;
        direction.y *= 3.0;
    }

    dog_transform.translation.x += direction.x;
    dog_transform.translation.y += direction.y;
}