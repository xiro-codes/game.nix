use bevy::{
    ecs::query,
    gizmos,
    input::keyboard::Key,
    math,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    transform,
    window::WindowResolution,
};
use bevy_sepax2d::prelude::{
    sepax2d::{sat_overlap, Circle as SpxCircle},
    *,
};
use rand::thread_rng;
use rand::Rng;

const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

#[derive(Component)]
struct Player {
    movement_speed: f32,
    rotation_speed: f32,
}
#[derive(Resource)]
struct Config {
    ship_spawn_speed: f32,
    ship_movement_speed: f32,
    ship_rotation_speed: f32,

    rock_spawn_speed: f32,
    rock_movement_speed: f32,
}
#[derive(Component)]
enum Spawner {
    Rock {
        timer: Timer,
        sizes: Vec<(Mesh2dHandle, f32)>,
        life: u8,
    },
    Ship {
        timer: Timer,
    },
}

#[derive(Resource)]
struct SpawnArea {
    rect: Rect,
}

#[derive(Resource)]
struct CenterArea {
    rect: Rect,
}
#[derive(Resource)]
struct PlayArea {
    rect: Rect,
}

#[derive(Component)]
struct Rock;

#[derive(Component, Default)]
enum MoveTo {
    Player {
        rotation_speed: f32,
        movement_speed: f32,
    },

    Point {
        movement_speed: f32,
        x: f32,
        y: f32,
    },
    #[default]
    None,
}
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(BOUNDS.x, BOUNDS.y),
                    title: "game".into(),
                    ..default()
                }),
                ..default()
            }),
            SepaxPlugin,
        ))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(SpawnArea {
            rect: Rect::new(-BOUNDS.x, -BOUNDS.y, BOUNDS.x, BOUNDS.y),
        })
        .insert_resource(CenterArea {
            rect: Rect::new(-BOUNDS.x / 3., -BOUNDS.y / 3., BOUNDS.x / 3., BOUNDS.y / 3.),
        })
        .insert_resource(PlayArea {
            rect: Rect::new(-BOUNDS.x / 2., -BOUNDS.y / 2., BOUNDS.x / 2., BOUNDS.y / 2.),
        })
        .add_systems(Startup, setup_scene)
        .add_systems(
            FixedUpdate,
            (
                player_movement,
                draw_spawn_defs,
                rotate_to_player,
                rotate_to_point,
                player_collision.after(player_movement),
                rock_despawn.after(rotate_to_point).after(rotate_to_player),
            ),
        )
        .add_systems(Update, (spawn_rocks, spawn_ships))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup_scene(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    cmds.spawn(Camera2dBundle::default());
    create_player(&mut cmds, &mut meshes, &mut materials);
    spawn_rock_spawner(&mut cmds, &mut meshes, &mut materials);
    spawn_ship_spawner(&mut cmds, &mut meshes, &mut materials);
}

fn draw_spawn_defs(
    mut gizmos: Gizmos,
    spawn: Res<SpawnArea>,
    exlude: Res<CenterArea>,
    play: Res<PlayArea>,
) {
    gizmos.rect_2d(Vec2::ZERO, 0., spawn.rect.size(), Color::RED);
    gizmos.rect_2d(Vec2::ZERO, 0., exlude.rect.size(), Color::BLUE);
    gizmos.rect_2d(Vec2::ZERO, 0., play.rect.size(), Color::WHITE);
}

fn spawn_ship_spawner(
    cmds: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let shape = SpxCircle::new((0., 0.), 10.0);
    cmds.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 10.0 })),
            material: materials.add(Color::RED),
            transform: Transform::from_xyz(0., 0., 3.),
            ..default()
        },
        Sepax {
            convex: Convex::Circle(shape),
        },
        Spawner::Ship {
            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
        },
    ));
}
fn spawn_rock_spawner(
    cmds: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let sizes = vec![
        (Mesh2dHandle(meshes.add(RegularPolygon::new(10.0, 5))), 10.0),
        (Mesh2dHandle(meshes.add(RegularPolygon::new(25.0, 8))), 25.0),
        (Mesh2dHandle(meshes.add(RegularPolygon::new(30.0, 6))), 30.0),
        (Mesh2dHandle(meshes.add(RegularPolygon::new(40.0, 9))), 40.0),
    ];
    let shape = SpxCircle::new((0., 0.), 25.0);
    cmds.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 25.0 })),
            material: materials.add(Color::BLUE),
            ..default()
        },
        Sepax {
            convex: Convex::Circle(shape),
        },
        Spawner::Rock {
            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
            life: 5,
            sizes,
        },
    ));
}
fn find_vertexes(center: Vec2, angle: f32, dims: Vec2) -> (Vec2, Vec2, Vec2, Vec2) {
    // def get_corners_from_rectangle(center: Vector, angle: float, dimensions: Vector):
    //# create the (normalized) perpendicular vectors
    //v1 = Vector(cos(angle), sin(angle))
    //v2 = Vector(-v1[1], v1[0])  # rotate by 90

    //# scale them appropriately by the dimensions
    //v1 *= dimensions[0] / 2
    //v2 *= dimensions[1] / 2

    //# return the corners by moving the center of the rectangle by the vectors
    //return [
    //   center + v1 + v2,
    //   center - v1 + v2,
    //   center - v1 - v2,
    //   center + v1 - v2,
    //]
    //
    let mut v1 = Vec2::new(f32::cos(angle), f32::sin(angle));
    let mut v2 = Vec2::new(-v1.y, v1.x);
    v1 *= dims.x / 2.;
    v2 *= dims.y / 2.;
    (
        center - v1 + v2,
        center - v1 - v2,
        center + v1 + v2, //
        center + v1 - v2,
    )
}
fn spawn_ships(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    spawn: Res<SpawnArea>,
    exlude: Res<CenterArea>,
    time: Res<Time>,
    mut query: Query<(&mut Spawner, &Transform)>,
) {
    for (mut s, t) in query.iter_mut() {
        let (timer,) = match s.as_mut() {
            Spawner::Ship { timer } => (timer,),
            _ => continue,
        };
        timer.tick(time.delta());
        if timer.just_finished() {
            let (left_top, left_bottom, right_top, _) =
                find_vertexes(spawn.rect.center(), 0.0, spawn.rect.size());

            let x_range = left_top.x..right_top.x;
            let y_range = left_bottom.y..left_top.y;
            // Ensure the spawn point is outside the exclusion zone
            let mut rng = thread_rng();
            let mut spawn_point = Vec2::new(rng.gen_range(x_range), rng.gen_range(y_range));
            while exlude.rect.contains(spawn_point) {
                let x_range = left_top.x..right_top.x;
                let y_range = left_bottom.y..left_top.y;
                spawn_point = Vec2::new(rng.gen_range(x_range), rng.gen_range(y_range));
            }
            create_ship(&mut cmds, &mut meshes, &mut materials, spawn_point);
        }
    }
}

fn spawn_rocks(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    spawn: Res<SpawnArea>,
    exlude: Res<CenterArea>,
    time: Res<Time>,
    mut query: Query<(&mut Spawner, &Transform)>,
) {
    for (mut s, t) in query.iter_mut() {
        let (timer, sizes, life) = match s.as_mut() {
            Spawner::Rock { timer, sizes, life } => (timer, sizes, life),
            _ => continue,
        };
        timer.tick(time.delta());
        if timer.just_finished() {
            let (left_top, left_bottom, right_top, _) =
                find_vertexes(spawn.rect.center(), 0.0, spawn.rect.size());

            let x_range = left_top.x..right_top.x;
            let y_range = left_bottom.y..left_top.y;
            // Ensure the spawn point is outside the exclusion zone
            let mut rng = thread_rng();
            let mut spawn_point = Vec2::new(rng.gen_range(x_range), rng.gen_range(y_range));
            while exlude.rect.contains(spawn_point) {
                let x_range = left_top.x..right_top.x;
                let y_range = left_bottom.y..left_top.y;
                spawn_point = Vec2::new(rng.gen_range(x_range), rng.gen_range(y_range));
            }
            create_rock(&mut cmds, &mut meshes, &mut materials, &sizes, spawn_point);
        }
    }
}
fn create_ship(
    cmds: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    spawn_point: Vec2,
) {
    let transform = Transform::from_xyz(spawn_point.x, spawn_point.y, 2.);
    let shape = SpxCircle::new((0., 0.), 25.0);
    cmds.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Triangle2d::new(
                Vec2::Y * 20.0,
                Vec2::new(-20.0, -20.0),
                Vec2::new(20.0, -20.0),
            ))),
            material: materials.add(Color::PINK),
            transform,
            ..default()
        },
        Rock,
        Sepax {
            convex: Convex::Circle(shape),
        },
        Movable { axes: Vec::new() },
        MoveTo::Player {
            movement_speed: 50.,
            rotation_speed: 3.0,
        },
    ));
}
fn create_rock(
    cmds: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    sizes: &Vec<(Mesh2dHandle, f32)>,
    spawn_point: Vec2,
) {
    let mut rng = thread_rng();
    let size = rng.gen_range(0..sizes.len());
    let mut transform = Transform::from_xyz(spawn_point.x, spawn_point.y, 2.);
    info!("spawned rock at {:?}", transform);
    let (mesh, size) = sizes.get(size).unwrap();
    let shape = SpxCircle::new((0., 0.), *size);
    cmds.spawn((
        MaterialMesh2dBundle {
            mesh: mesh.clone(),
            material: materials.add(Color::rgb(0.4, 0.8, 0.1)),
            transform,
            ..default()
        },
        Rock,
        Sepax {
            convex: Convex::Circle(shape),
        },
        Movable { axes: Vec::new() },
        MoveTo::Point {
            movement_speed: 50.,
            x: 0.,
            y: 0.,
        },
    ));
}
fn create_player(
    cmds: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let shape = SpxCircle::new((0., 0.), 25.0);
    cmds.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Triangle2d::new(
                Vec2::Y * 25.0,
                Vec2::new(-25.0, -25.0),
                Vec2::new(25.0, -25.0),
            ))),
            material: materials.add(Color::rgb(0.4, 0.8, 0.1)),
            ..default()
        },
        Sepax {
            convex: Convex::Circle(shape),
        },
        Movable { axes: Vec::new() },
        Player {
            movement_speed: 100.0,
            rotation_speed: 5.0,
        },
    ));
}
fn player_collision(
    mut query: Query<(&Player, &mut Transform, &Sepax)>,
    targets: Query<&Sepax, Without<Player>>,
) {
    for (player, transform, bbox) in query.iter() {
        for targets in targets.iter() {
            if sat_overlap(targets.shape(), bbox.shape()) {
                info!("Hit thing");
            }
        }
    }
}
fn rock_despawn(
    mut cmds: Commands,
    query: Query<(Entity, &Sepax), With<Rock>>,
    mut targets: Query<(&Sepax, &mut Spawner), Without<Rock>>,
) {
    for (e, s) in query.iter() {
        for (targets, mut spawner) in targets.iter_mut() {
            if let Spawner::Rock {
                timer: _,
                sizes: _,
                life: _,
            } = spawner.as_mut()
            {
                if sat_overlap(targets.shape(), s.shape()) {
                    cmds.entity(e).despawn();
                }
            } else {
                continue;
            }
        }
    }
}
fn spawner_size_system() {}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    for (player, mut transform) in &mut query {
        let mut rotation_factor = 0.0;
        let mut movement_factor = 0.0;
        if keyboard_input.pressed(KeyCode::KeyA) {
            rotation_factor += 1.0;
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            rotation_factor -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::KeyW) {
            movement_factor += 1.0;
        }

        // update the ship rotation around the Z axis (perpendicular to the 2D plane of the screen)
        transform.rotate_z(rotation_factor * player.rotation_speed * time.delta_seconds());

        // get the ship's forward vector by applying the current rotation to the ships initial facing
        // vector
        let movement_direction = transform.rotation * Vec3::Y;
        // get the distance the ship will move based on direction, the ship's movement speed and delta
        // time
        let movement_distance = movement_factor * player.movement_speed * time.delta_seconds();
        // create the change in translation using the new movement direction and distance
        let translation_delta = movement_direction * movement_distance;
        // update the ship translation with our new translation delta
        transform.translation += translation_delta;

        // bound the ship within the invisible level bounds
        let extents = Vec3::from((BOUNDS / 2.0, 0.0));
        transform.translation = transform.translation.min(extents).max(-extents);
    }
}
fn rotate_to_player(
    time: Res<Time>,
    mut query: Query<(&MoveTo, &mut Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_transform = player_query.single();
    // get the player translation in 2D
    let player_translation = player_transform.translation.xy();
    for (config, mut enemy_transform) in &mut query {
        let (rotation_speed, movment_speed) = match config {
            MoveTo::Player {
                rotation_speed,
                movement_speed,
            } => (rotation_speed.to_owned(), movement_speed.to_owned()),
            _ => continue,
        };

        let enemy_forward = (enemy_transform.rotation * Vec3::Y).xy();

        let to_player = (player_translation - enemy_transform.translation.xy()).normalize();

        let forward_dot_player = enemy_forward.dot(to_player);

        if (forward_dot_player - 1.0).abs() < f32::EPSILON {
            continue;
        }

        let enemy_right = (enemy_transform.rotation * Vec3::X).xy();

        let right_dot_player = enemy_right.dot(to_player);

        let rotation_sign = -f32::copysign(1.0, right_dot_player);

        let max_angle = forward_dot_player.clamp(-1.0, 1.0).acos(); // clamp acos for safety

        let rotation_angle = rotation_sign * (rotation_speed * time.delta_seconds()).min(max_angle);

        enemy_transform.rotate_z(rotation_angle);

        let movement_direction = enemy_transform.rotation * Vec3::Y;
        let movement_distance = movment_speed * time.delta_seconds();
        let translation_delta = movement_direction * movement_distance;
        enemy_transform.translation += translation_delta;

        let extents = Vec3::from((BOUNDS / 2.0, 0.0));
        enemy_transform.translation = enemy_transform.translation;
    }
}
fn rotate_to_point(time: Res<Time>, mut query: Query<(&MoveTo, &mut Transform), Without<Player>>) {
    for (config, mut enemy_transform) in &mut query {
        let (movment_speed, x, y) = match config {
            MoveTo::Point {
                movement_speed,
                x,
                y,
            } => (movement_speed.to_owned(), x.to_owned(), y.to_owned()),
            _ => continue,
        };

        let enemy_forward = (enemy_transform.rotation * Vec3::Y).xy();

        let to_player = (Vec2::new(x, y) - enemy_transform.translation.xy()).normalize();

        let forward_dot_player = enemy_forward.dot(to_player);

        //if (forward_dot_player - 1.0).abs() < f32::EPSILON {
        //    info!("Im stoped");
        //    continue;
        //}

        let enemy_right = (enemy_transform.rotation * Vec3::X).xy();

        let right_dot_player = enemy_right.dot(to_player);

        let rotation_sign = -f32::copysign(1.0, right_dot_player);

        let max_angle = forward_dot_player.clamp(-1.0, 1.0).acos(); // clamp acos for safety

        let rotation_angle = rotation_sign * (50.0 * time.delta_seconds()).min(max_angle);

        enemy_transform.rotate_z(rotation_angle);

        let movement_direction = enemy_transform.rotation * Vec3::Y;
        let movement_distance = movment_speed * time.delta_seconds();
        let translation_delta = movement_direction * movement_distance;
        enemy_transform.translation += translation_delta;

        enemy_transform.translation = enemy_transform.translation;
    }
}
