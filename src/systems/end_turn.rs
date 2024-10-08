use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Point)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState, #[resource] map: &Map) {
    let mut player_hp = <(&Health, &Point)>::query().filter(component::<Player>());
    let amulet_pos = <&Point>::query()
        .filter(component::<AmuletOfYala>())
        .iter(ecs)
        .next();
    let current_state = turn_state.clone();

    let mut new_state = match current_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state,
    };

    player_hp.iter(ecs).for_each(|(hp, pos)| {
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }
        if map.tiles[Map::map_idx(*pos)] == TileType::Exit {
            new_state = TurnState::NextLevel;
        }
        if Some(pos) == amulet_pos {
            new_state = TurnState::Victory;
        }
    });

    *turn_state = new_state;
}
