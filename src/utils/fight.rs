use super::consts::LOOT_TABLE;
use super::game_state::*;
use super::monster::*;
use super::player::*;
use rand::Rng;

pub fn roll_initiative(state: &mut GameState) -> bool {
    state.add_event(GameEvent {
        roll: None,
        bool_enemy_turn: None,
        description: String::from("Rolling initiative......"),
    });
    let mut rng = rand::thread_rng();
    let random_variable: i32 = rng.gen_range(0..=1);
    if random_variable == 0 {
        state.add_event(GameEvent {
            roll: None,
            bool_enemy_turn: None,
            description: String::from("You start!"),
        });
        state.player_inputs_accepted = true;
    } else {
        state.add_event(GameEvent {
            roll: None,
            bool_enemy_turn: None,
            description: String::from(format!("{} is starting first!", state.current_monster.name)),
        });
    };
    random_variable == 0
}

pub fn roll_attack(state: &mut GameState, player_attacked: bool) {
    let mut rng = rand::thread_rng();
    let roll_for_hit: i32 = rng.gen_range(0..=20);

    let mut damage: i32;
    if player_attacked {
        damage = state.current_monster.get_attack_damage();
    } else {
        damage = state.player.get_attack_damage();
    };

    let mut description = String::new();
    match roll_for_hit {
        0 => {
            description.push_str("Critical miss!");
            damage = 0;
        }
        1_i32..=19_i32 => {
            let string = if player_attacked {
                format!("You take {} damage!", damage)
            } else {
                format!("Enemy takes {} damage!", damage)
            };
            description.push_str(&string);
        }
        20 => {
            damage = damage * 2;
            let string = if player_attacked {
                format!("You take {} damage!", damage)
            } else {
                format!("Enemy takes {} damage!", damage)
            };
            description.push_str(&string);
        }
        _ => unreachable!(),
    }
    if player_attacked {
        state.player.receive_damage(damage);
        description
            .push_str(format!(" You have {} HP remaining!", state.player.health_points).as_str());
    } else {
        state.current_monster.receive_damage(damage);
        description.push_str(
            format!(
                " Ennemy has {} HP remaining!",
                state.current_monster.health_points
            )
            .as_str(),
        );
    };
    state.add_event(GameEvent {
        roll: Some(roll_for_hit.to_string()),
        bool_enemy_turn: Some(player_attacked),
        description: String::from(description),
    });
}

pub fn check_for_death(state: &mut GameState) -> bool {
    if state.player.health_points <= 0 {
        state.add_event(GameEvent {
            roll: None,
            bool_enemy_turn: None,
            description: String::from("GAME OVER..."),
        });
        state.player_inputs_accepted = false;
        state.game_over = true;
        return true;
    } else if state.current_monster.health_points <= 0 {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..LOOT_TABLE.len());
        let random_crap = LOOT_TABLE[index];
        state.add_event(GameEvent {
            roll: None,
            bool_enemy_turn: None,
            description: String::from(""),
        });
        state.add_event(GameEvent {
            roll: None,
            bool_enemy_turn: None,
            description: format!(
                "Monster has been slain! You receive {} experience points and a nice {}.",
                state.current_monster.experience_given, random_crap
            ),
        });
        let level_before = state.player.level;
        state
            .player
            .receive_experience(state.current_monster.experience_given);
        if level_before != state.player.level {
            state.add_event(GameEvent {
                roll: None,
                bool_enemy_turn: None,
                description: String::from(""),
            });
            state.add_event(GameEvent {
                roll: None,
                bool_enemy_turn: None,
                description: String::from(format!(
                    "LEVEL UP! Your new level: {}",
                    state.player.level
                )),
            });
        };
        state.add_event(GameEvent {
            roll: None,
            bool_enemy_turn: None,
            description: String::from(""),
        });
        return true;
    };
    false
}

pub fn switch_attack_turn(state: &mut GameState, give_turn_to_player: bool) {
    state.add_event(GameEvent {
        roll: None,
        bool_enemy_turn: None,
        description: String::from(""),
    });
    if give_turn_to_player {
        state.player_inputs_accepted = true;
        state.add_event(GameEvent {
            roll: None,
            bool_enemy_turn: Some(!give_turn_to_player),
            description: String::from("It's your turn! CHARGE!"),
        });
    } else {
        state.player_inputs_accepted = false;
        state.add_event(GameEvent {
            roll: None,
            bool_enemy_turn: Some(!give_turn_to_player),
            description: String::from("It's the enemy's turn!"),
        });
    }
}

pub fn start_new_battle(state: &mut GameState) {
    state.current_monster = get_random_monster(state);
    state.add_event(GameEvent {
        roll: None,
        bool_enemy_turn: None,
        description: String::from(format!(
            "A wild {} appears, brace yourself!",
            state.current_monster.name
        )),
    });
    state.add_event(GameEvent {
        roll: None,
        bool_enemy_turn: None,
        description: String::from(""),
    });
}

pub fn initiate_logs(events: &mut Vec<GameEvent>) {
    events.push(GameEvent {
        roll: None,
        bool_enemy_turn: None,
        description: String::from("New game is starting..."),
    });
    events.push(GameEvent {
        roll: None,
        bool_enemy_turn: None,
        description: String::from(""),
    });
}
