use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::time::FixedTimestep;
use rand::prelude::random;

/** Components **/

#[derive(Component, Debug)]
struct Bird;

#[derive(Component, Debug)]
struct Wall;

#[derive(Default, Deref, DerefMut)]
struct WallVec(Vec<Entity>);

#[derive(Component, Debug, Clone, Copy, PartialEq)]
struct Position(Vec2);

#[derive(Component, Debug, Clone, Copy, PartialEq)]
struct Speed(Vec2);

#[derive(Component, Debug)]
struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

fn size_to_vec2(s: &Size) -> Vec2 {
    Vec2 {
        x: s.width,
        y: s.height,
    }
}

/** Events */

struct GameOverEvent;

/** Constants **/

const MIN_GAP_SIZE: f32 = 150.0;
const MAX_GAP_SIZE: f32 = 350.0;

const BIRD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const WALL_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

fn main() {
    App::new()
        // Setup
        .add_event::<GameOverEvent>()
        .insert_resource(WindowDescriptor {
            title: "Bird!".to_string(),
            width: 500.0,
            height: 500.0,
            ..default()
        })
        // .insert_resource(WallVec::default())
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_bird)
        // Loop
        .add_system(bird_movement_input)
        .add_system(movement)
        .add_system(bird_movement.after(bird_movement_input).after(movement))
        // Post-update
        .add_system_to_stage(CoreStage::PostUpdate, collision)
        .add_system_to_stage(CoreStage::PostUpdate, game_over.after(collision))
        .add_system_to_stage(CoreStage::PostUpdate, entity_transform_update)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(2.5))
                .with_system(wall_spawner)
                .with_system(wall_despawner),
        )
        .add_plugins(DefaultPlugins)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}

// Setup systems

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        transform: Transform {
            scale: Vec3 {
                x: 1.0,
                y: -1.0,
                z: 1.0,
            },
            ..default()
        },
        ..default()
    });
}

fn spawn_bird(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: BIRD_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Bird)
        .insert(Position(Vec2 {
            x: -200.0,
            y: 500.0,
        }))
        .insert(Speed(Vec2 { x: 0.0, y: 0.0 }))
        .insert(Size::square(30.0));
}

// Looping Systems

fn wall_spawner(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();

    let gap_size = random::<f32>() * (MAX_GAP_SIZE - MIN_GAP_SIZE) + MIN_GAP_SIZE;

    let gap_position = (random::<f32>() - 0.5) * (window.height() - gap_size);

    let upper_height = window.height() / 2.0 + gap_position - gap_size / 2.0;
    let lower_height = window.height() / 2.0 - gap_position - gap_size / 2.0;

    let upper_center = (-window.height() + upper_height) / 2.0;
    let lower_center = (window.height() - lower_height) / 2.0;

    // upper wall
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Wall)
        .insert(Position(Vec2 {
            x: window.width() / 2.0,
            y: upper_center,
        }))
        .insert(Speed(Vec2 { x: -2.0, y: 0.0 }))
        .insert(Size {
            width: 10.0,
            height: upper_height,
        });

    // lower_wall
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Wall)
        .insert(Position(Vec2 {
            x: window.width() / 2.0,
            y: lower_center,
        }))
        .insert(Speed(Vec2 { x: -2.0, y: 0.0 }))
        .insert(Size {
            width: 10.0,
            height: lower_height,
        });
}

fn wall_despawner(
    mut windows: Res<Windows>,
    mut commands: Commands,
    q: Query<(Entity, &Position), With<Wall>>,
) {
    let window = windows.get_primary().unwrap();

    for (ent, pos) in q.iter() {
        if pos.0.x < -window.width() / 2.0 {
            commands.entity(ent).despawn();
        }
    }
}

fn bird_movement_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut bird_speed: Query<&mut Speed, With<Bird>>,
) {
    if let Some(mut speed) = bird_speed.iter_mut().next() {
        if keyboard_input.pressed(KeyCode::Space) {
            speed.0.y = -10.0
        } else {
            speed.0.y += 1.0
        }
    }
}

fn bird_movement(windows: Res<Windows>, mut birds: Query<(&mut Position, &mut Speed, With<Bird>)>) {
    let window = windows.get_primary().unwrap();

    for (mut pos, mut speed, ()) in birds.iter_mut().next() {
        let half_height = window.height() / 2.0;
        if pos.0.y > half_height {
            pos.0.y = half_height;
        }
        if pos.0.y < -half_height {
            pos.0.y = -half_height;
        }

        speed.0.y *= 0.8
    }
}

fn movement(mut walls: Query<(&mut Position, &mut Speed)>) {
    for (mut pos, speed) in walls.iter_mut() {
        pos.0.x += speed.0.x;
        pos.0.y += speed.0.y;
    }
}

fn collision(
    mut game_over_writer: EventWriter<GameOverEvent>,
    bird: Query<(&Position, &Size), With<Bird>>,
    walls: Query<(&Position, &Size), With<Wall>>,
) {
    for (bird_pos, bird_size) in bird.iter() {
        for (wall_pos, wall_size) in walls.iter() {
            let bird_min = bird_pos.0 - size_to_vec2(bird_size) / 2.0;
            let bird_max = bird_pos.0 + size_to_vec2(bird_size) / 2.0;
            let wall_min = wall_pos.0 - size_to_vec2(wall_size) / 2.0;
            let wall_max = wall_pos.0 + size_to_vec2(wall_size) / 2.0;
            if bird_min.x < wall_max.x
                && bird_max.x > wall_min.x
                && bird_min.y < wall_max.y
                && bird_max.y > wall_min.y
            {
                game_over_writer.send(GameOverEvent);
            }
        }
    }
}

fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    mut walls: Query<Entity, With<Wall>>,
) {
    if reader.iter().next().is_some() {
        for wal in walls.iter() {
            commands.entity(wal).despawn();
        }
    }
}

fn entity_transform_update(mut q: Query<(&Position, &Size, &mut Transform)>) {
    for (pos, size, mut transform) in q.iter_mut() {
        transform.translation.x = pos.0.x;
        transform.translation.y = pos.0.y;
        transform.scale.x = size.width;
        transform.scale.y = size.height;
    }
}
