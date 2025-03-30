use std::time::Duration;

use bevy::{
    app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*, utils::error, winit::WinitPlugin,
};

const GRAY4: Color = Color::srgb(51. / 255., 51. / 255., 51. / 255.);

pub fn app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(2)))
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
        Mesh2d(meshes.add(Rectangle::new(
            box_size.width as f32,
            box_size.height as f32,
        ))),
        MeshMaterial2d(materials.add(Color::srgb(0.5, 0.5, 0.5))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scale: 0.1,
            ..OrthographicProjection::default_2d()
        },
        Transform::from_xyz(box_size.width as f32 / 2., box_size.height as f32 / 2., 0.0),
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
        Mesh2d(meshes.add(Rectangle::new(1., 1.))),
        MeshMaterial2d(materials.add(Color::srgb(0.5, 0.5, 0.5))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn move_snakes(mut snakes: Query<(&mut Transform, &mut Snake)>, box_size: Res<BoxSize>) {
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
