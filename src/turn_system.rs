use bevy::prelude::*;
#[derive(Resource, Default)]
pub enum TurnTimer {
    Player {
        timer: Timer,
        target: Entity,
    },
    Enemy {
        timer: Timer,
        target: Entity,
    },
    #[default]
    None,
}

#[derive(Resource, Default)]
pub struct TurnOrder {
    players: Vec<Entity>,
}

#[derive(Event, Default)]
pub enum TurnEvents {
    PlayerAction {
        action: Action,
    },
    EnemyAction {
        action: Action,
    },

    EndTurn(Entity),
    StartTurn(Entity),

    #[default]
    StartBattle,
}

#[derive(Default)]
pub enum Action {
    Attack,
    Spell {
        elem: ElementType,
    },
    Defend,
    #[default]
    Pass,
}

#[derive(Default)]
pub enum ElementType {
    Earth,
    Fire,
    Water,
    Air,
    #[default]
    Void,
}

pub struct TurnSystemPlugin;
impl Plugin for TurnSystemPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TurnTimer::default())
            .insert_resource(TurnOrder::default())
            .add_event::<TurnEvents>()
            .add_systems(Update, (update_turn_timer, update_turn_order));
    }
}

pub fn update_turn_timer(
    time: Res<Time>,
    mut turn_timer: ResMut<TurnTimer>,
    mut event_writer: EventWriter<TurnEvents>,
) {
    match turn_timer.as_mut() {
        TurnTimer::Player { timer, target } => {
            timer.tick(time.delta());
            if timer.just_finished() {
                event_writer.send(TurnEvents::EndTurn(*target));
            }
        }
        TurnTimer::Enemy { timer, target } => {
            timer.tick(time.delta());
            if timer.just_finished() {
                event_writer.send(TurnEvents::EndTurn(*target));
            }
        }
        _ => (),
    }
}
#[derive(Component)]
pub struct InBattle;

pub fn update_turn_order(
    mut cmds: Commands,
    mut _event_reader: EventReader<TurnEvents>,
    mut turn_order: ResMut<TurnOrder>,
    in_battle: Query<Entity, With<InBattle>>,
) {
    for event in _event_reader.read() {
        match event {
            TurnEvents::EndTurn(e) => turn_order.players.retain(|entity| entity != e),
            TurnEvents::StartTurn(e) => turn_order.players.push(*e),
            TurnEvents::StartBattle => {
                turn_order.players = in_battle.iter().collect::<Vec<Entity>>()
            }
            _ => {}
        }
    }
}
