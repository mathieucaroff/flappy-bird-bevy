use rand::prelude::random;
use bevy::time::FixedTimestep;

use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

/** Components **/

#[derive(Component, Debug)]
struct Bird;

#[derive(Component, Debug)]
struct Wall;

#[derive(Default, Deref, DerefMut)]
struct WallVec(Vec<Entity>);


#[derive(Component, Debug, Clone, Copy, PartialEq)]
struct Position (Vec2);

#[derive(Component, Debug, Clone, Copy, PartialEq)]
struct Speed (Vec2);

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

/** Events */

struct GameOverEvent;


/** Constants **/

const BIRD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

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
        .add_system(bird_movement.after(bird_movement_input))
        // Post-update
        .add_system_to_stage(CoreStage::PostUpdate, entity_transform_update)

        // .add_system(game_over)
        .add_plugins(DefaultPlugins)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}


fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle{
        transform: Transform {
            scale: Vec3{x: 1.0, y: -1.0, z: 1.0},
            ..default()
        },
        ..default()
    });
}

fn spawn_bird(
    mut commands: Commands,
) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: BIRD_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Bird)
        .insert(Position(Vec2{x: -200.0, y: 500.0}))
        .insert(Speed(Vec2{x:0.0, y:0.0}))
        .insert(Size::square(30.0));
}

fn bird_movement_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut bird_speed: Query<&mut Speed, With<Bird>>,
) {
    if let Some(mut speed) = bird_speed.iter_mut().next() {
        speed.0.y *= 0.98;

        if keyboard_input.pressed(KeyCode::Space) {
            speed.0.y = -20.0
        } else {
            speed.0.y += 2.0
        }
    }
}

fn bird_movement(
    mut birds: Query<(&mut Position, &mut Speed, With<Bird>)>,
) {
    if let Some((mut pos, speed, ())) = birds.iter_mut().next() {
        pos.0.x += speed.0.x;
        pos.0.y += speed.0.y;

        if pos.0.y > 200.0 {
            pos.0.y = 200.0;
        }
    }
}

fn entity_transform_update(
    mut q: Query<(&Position, &Size, &mut Transform)>
) {
    for (pos, size, mut transform) in q.iter_mut() {
        transform.translation.x = pos.0.x;
        transform.translation.y = pos.0.y;
        transform.scale.x = size.width;
        transform.scale.y = size.height;
    }
}