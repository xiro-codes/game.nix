use std::default;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::WindowResolution,
};
const RESOLUTION: Vec2 = Vec2::new(1200.0, 640.0);
use game::turn_system;
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(RESOLUTION.x, RESOLUTION.y),
                    title: "game".into(),
                    ..default()
                }),
                ..default()
            }),
            turn_system::TurnSystemPlugin,
        ))
        .add_systems(Startup, (setup_scene))
        .run();
}
fn setup_scene(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    cmds.spawn(Camera2dBundle::default());
    create_player_team(&mut cmds, &mut meshes, &mut materials);
}
fn create_player_team(
    cmds: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let meshes = vec![
        Mesh2dHandle(meshes.add(Rectangle::new(25., 23.))),
        Mesh2dHandle(meshes.add(Rectangle::new(20., 25.))),
        Mesh2dHandle(meshes.add(Rectangle::new(18., 20.))),
        Mesh2dHandle(meshes.add(Rectangle::new(15., 18.))),
    ];
    let colors = vec![
        materials.add(Color::SILVER),
        materials.add(Color::BLUE),
        materials.add(Color::RED),
        materials.add(Color::GREEN),
    ];
    for (idx, (mesh, material)) in meshes.iter().zip(colors).enumerate() {
        cmds.spawn((MaterialMesh2dBundle {
            mesh: mesh.clone(),
            material,
            transform: Transform::from_xyz(-400., 40. * (idx as f32), 0.),
            ..default()
        },));
    }
}
