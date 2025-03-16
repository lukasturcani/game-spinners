use bevy::{log::LogPlugin, prelude::*, winit::WinitPlugin};

pub fn app() -> App {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .build()
            .disable::<WinitPlugin>()
            .disable::<LogPlugin>(),
    );
    app.add_systems(Startup, (setup, spawn_snake));
    app
}

#[derive(Debug, Default, Resource)]
struct Score(u16);

#[derive(Debug, Component)]
struct Food;

#[derive(Debug, Event)]
struct AteFood;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.insert_resource(Score(0));
}

fn spawn_snake(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(50.0, 50.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.5, 0.5, 0.5))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn spawn_food(mut commands: Commands) {
    todo!()
}
