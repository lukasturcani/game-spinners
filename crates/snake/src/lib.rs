use std::time::Duration;

use bevy::{color::palettes::css::*, prelude::*};

pub fn app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(250)))
        .add_systems(Startup, (setup, spawn_snake))
        .add_systems(FixedUpdate, move_snakes);
    app
}

#[derive(Debug, Default, Resource)]
struct Score(u16);

#[derive(Debug, Resource)]
struct BoxSize {
    width: u16,
    height: u16,
}

#[derive(Debug, Component)]
struct Food;

#[derive(Debug, Event)]
struct AteFood;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Component)]
struct Snake {
    direction: Direction,
    head_position: (u16, u16),
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let box_size = BoxSize {
        width: 10,
        height: 10,
    };

    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scale: 0.01,
            ..OrthographicProjection::default_2d()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(1.))),
        MeshMaterial2d(materials.add(Color::from(RED))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    commands.insert_resource(Score(0));
    commands.insert_resource(box_size);
}

fn spawn_snake(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Snake {
            direction: Direction::Right,
            head_position: (0, 0),
        },
        Mesh2d(meshes.add(Rectangle::new(1.34, 1.34))),
        MeshMaterial2d(materials.add(Color::from(LIMEGREEN))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn move_snakes(mut snakes: Query<(&mut Transform, &mut Snake)>, box_size: Res<BoxSize>) {
    return;
    for (mut transform, mut snake) in snakes.iter_mut() {
        match snake.direction {
            Direction::Up => {
                if snake.head_position.1 == 0 {
                    snake.head_position.1 = box_size.height - 1;
                } else {
                    snake.head_position.1 -= 1;
                }
            }
            Direction::Down => {
                if snake.head_position.1 == box_size.height - 1 {
                    snake.head_position.1 = 0;
                } else {
                    snake.head_position.1 += 1;
                }
            }
            Direction::Left => {
                if snake.head_position.0 == 0 {
                    snake.head_position.0 = box_size.width - 1;
                } else {
                    snake.head_position.0 -= 1;
                }
            }
            Direction::Right => {
                if snake.head_position.0 == box_size.width - 1 {
                    snake.head_position.0 = 0;
                } else {
                    snake.head_position.0 += 1;
                }
            }
        }
        transform.translation.x = snake.head_position.0 as f32;
        transform.translation.y = snake.head_position.1 as f32;
    }
}

fn spawn_food(mut commands: Commands) {
    todo!()
}
