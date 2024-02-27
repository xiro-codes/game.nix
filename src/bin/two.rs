use bevy::prelude::*;

// Components
#[derive(Component)]
struct Name(String);
#[derive(Component)]
struct Health(i32);
#[derive(Component)]
struct Attack(i32);
#[derive(Component)]
struct Defense(i32);

#[derive(Component)]
struct Participant;

#[derive(Component)]
struct TurnMarker;

// Resource
#[derive(Resource)]
struct TurnOrder(Vec<Entity>);

// Systems
fn turn_system(
    mut turn_order: ResMut<TurnOrder>,
    query: Query<(Entity, &Health), With<Participant>>,
) {
    // Get entities sorted by health for the turn order
    turn_order.0 = query.iter().map(|(e, _)| e).collect::<Vec<Entity>>();
    turn_order
        .0
        .sort_by_key(|entity| query.get(*entity).unwrap().1 .0);
}

fn attack_system(
    mut commands: Commands,
    turn_order: Res<TurnOrder>,
    query: Query<(&Attack, &Name), With<TurnMarker>>,
    mut targets: Query<&mut Health, (Without<TurnMarker>, With<Participant>)>,
) {
    if let Some(entity) = turn_order.0.first() {
        if let Ok((attack, name)) = query.get(*entity) {
            info!("{} attacks for {} damage!", name.0, attack.0);
            // For simplicity, let's assume we attack the next entity in the turn order
            if let Some(target_entity) = turn_order.0.get(1) {
                if let Ok(mut health) = targets.get_mut(*target_entity) {
                    health.0 -= attack.0;
                    info!("{} takes damage! Health: {}", name.0, health.0);
                }
            }
        }
    }
    // End turn by removing TurnMarker component
    for entity in turn_order.0.iter() {
        if let Ok(mut _marker) = query.get(*entity) {
            commands.entity(*entity).remove::<TurnMarker>();
        }
    }
}

fn next_turn_system(mut commands: Commands, mut query: Query<(Entity, &mut TurnMarker)>) {
    // End of round, assign new turn marker to next entity
    if let Some((entity, mut marker)) = query.iter_mut().next() {
        info!("Turn Over");
        commands.entity(entity).remove::<TurnMarker>();
    }
    if let Some((entity, _)) = query.iter_mut().next() {
        info!("Turn Start");
        commands.entity(entity).insert(TurnMarker);
    }
}

// Setup
fn setup(mut commands: Commands) {
    // Spawn entities
    commands.spawn((
        Name("Player".into()),
        Health(50),
        Attack(8),
        Defense(2),
        Participant,
        TurnMarker,
    ));

    commands.spawn((
        Name("Enemy".into()),
        Health(50),
        Attack(8),
        Defense(2),
        Participant,
    ));

    // Initialize turn order
    commands.insert_resource(TurnOrder(Vec::new()));
}

// Bevy App
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (turn_system, attack_system, next_turn_system))
        .run();
}
