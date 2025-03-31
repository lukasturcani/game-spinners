use std::time::Duration;

use bevy::{color::palettes::css::*, prelude::*};

pub fn app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(150)))
        .add_systems(Startup, (setup, spawn_snake))
        .add_systems(FixedUpdate, move_snakes);
    app
}

#[derive(Debug, Default, Resource)]
struct Score(u16);

#[derive(Debug, Resource)]
struct BoxOutline {
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
    head_position: (i16, i16),
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let box_outline = BoxOutline {
        width: 60,
        height: 30,
    };
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scale: 0.05,
            ..OrthographicProjection::default_2d()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    let border_width = 1.0;
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(
            border_width,
            (box_outline.height + 2) as f32 + border_width,
        ))),
        MeshMaterial2d(materials.add(Color::from(GRAY))),
        Transform::from_xyz(-((box_outline.width + 2) as f32) / 2., 0.0, 0.0),
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(
            border_width,
            (box_outline.height + 2) as f32 + border_width,
        ))),
        MeshMaterial2d(materials.add(Color::from(GRAY))),
        Transform::from_xyz(((box_outline.width + 2) as f32) / 2., 0.0, 0.0),
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new((box_outline.width + 2) as f32, border_width))),
        MeshMaterial2d(materials.add(Color::from(GRAY))),
        Transform::from_xyz(0.0, -((box_outline.height + 2) as f32) / 2., 0.0),
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new((box_outline.width + 2) as f32, border_width))),
        MeshMaterial2d(materials.add(Color::from(GRAY))),
        Transform::from_xyz(0.0, ((box_outline.height + 2) as f32) / 2., 0.0),
    ));
    commands.insert_resource(Score(0));
    commands.insert_resource(box_outline);
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
        Mesh2d(meshes.add(Rectangle::new(1., 1.))),
        MeshMaterial2d(materials.add(Color::from(LIMEGREEN))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn move_snakes(mut snakes: Query<(&mut Transform, &mut Snake)>, box_outline: Res<BoxOutline>) {
    let max_height = box_outline.height as i16 / 2;
    let max_width = box_outline.width as i16 / 2;
    for (mut transform, mut snake) in snakes.iter_mut() {
        match snake.direction {
            Direction::Up => {
                if snake.head_position.1 == max_height {
                    snake.head_position.1 = -max_height;
                } else {
                    snake.head_position.1 -= 1;
                }
            }
            Direction::Down => {
                if snake.head_position.1 == -max_height {
                    snake.head_position.1 = max_height;
                } else {
                    snake.head_position.1 += 1;
                }
            }
            Direction::Left => {
                if snake.head_position.0 == -max_width {
                    snake.head_position.0 = max_width;
                } else {
                    snake.head_position.0 -= 1;
                }
            }
            Direction::Right => {
                if snake.head_position.0 == max_width {
                    snake.head_position.0 = -max_width;
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
