use super::consts::*;
use super::game_state::GameState;
use super::items::ItemActions;
use crate::ui::utils::{FightInfo, FighterInfo};
use crate::utils::items::Item;
use rand::Rng;

pub trait Attack {
    fn get_attack_damage(&self) -> i32;
    fn receive_damage(&mut self, attack_damage: i32);
    fn get_remaining_health_points(&self) -> i32;
    fn get_total_health_points(&self) -> i32;
}

pub struct Player {
    pub remaining_health_points: i32,
    pub total_health_points: i32,
    pub base_damage: std::ops::RangeInclusive<i32>,
    pub experience: i32,
    pub level: usize,
    pub name: String,
    pub experience_to_level_up: i32,
    pub image: String,
    pub inventory: Vec<Item>,
}

impl Attack for Player {
    fn get_attack_damage(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let roll_for_hit: i32 = rng.gen_range(self.base_damage.clone());
        roll_for_hit
    }
    fn receive_damage(&mut self, attack_damage: i32) {
        self.remaining_health_points -= attack_damage;
    }
    fn get_remaining_health_points(&self) -> i32 {
        self.remaining_health_points
    }
    fn get_total_health_points(&self) -> i32 {
        self.total_health_points
    }
}

impl FightInfo for Player {
    fn get_fighter_info(&self) -> crate::ui::utils::FighterInfo {
        FighterInfo {
            base_damage: self.base_damage.clone(),
            description: None,
            experience: Some(self.experience),
            experience_to_level_up: None,
            remaining_health_points: self.remaining_health_points,
            total_health_points: self.total_health_points,
            image: self.image.clone(),
            level: self.level,
            name: self.name.clone(),
        }
    }
}

impl Player {
    pub fn receive_experience(&mut self, experience_gained: i32) {
        self.experience_to_level_up -= experience_gained;
        if self.experience_to_level_up <= 0 {
            self.level += 1;
            self.total_health_points = PLAYER_BASE_HEALTH_POINT * 2i32.pow(self.level as u32);
            self.base_damage = 1..=PLAYER_BASE_RANGE_MAX_POINT + self.level as i32;
            if self.experience_to_level_up < 0 {
                self.experience = self.experience_to_level_up.abs();
                self.experience_to_level_up =
                    PLAYER_BASE_EXPERIENCE_NECESSARY * 2i32.pow(self.level as u32);
            }
        } else {
            self.experience += experience_gained;
        }
    }

    pub fn create_player(name: String) -> Player {
        let level = 1;

        let portrait = r#"
            ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡠⠤⠖⢒⠂⢤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
            ⠀⢀⣀⠀⠀⠀⠀⠀⢠⠖⠁⠀⠀⠀⠀⠀⠀⠢⣥⣢⠀⠀⠀⠀⠀⣠⣤⠀
            ⢀⣟⣿⣦⠀⠀⠀⣰⡿⠿⠷⠶⣄⠀⠀⢠⠾⠟⠛⠛⢷⡀⠀⢀⡼⣿⣇⡇
            ⠈⠛⠛⠿⢕⡂⢴⠁⠀⠀⠀⢀⠈⠆⠠⣮⣴⢤⡀⣀⣸⣗⣶⡧⠒⠉⠉⠁
            ⠀⠀⠀⠀⠀⢹⠀⠀⠴⣺⣿⣿⠇⠀⠀⠛⡿⣽⣿⣽⠿⠛⢻⠀⠀⠀⠀⠀
            ⠀⠀⠀⠀⠀⡌⠀⠀⠈⠉⢩⠀⠀⠀⠀⠀⣸⣒⣄⠀⠀⠀⠀⠇⠀⠀⠀⠀
            ⠀⠀⠀⠀⠀⡇⠀⢀⡴⠖⠉⠛⠓⠲⠶⠾⠿⠿⠿⢏⡳⡀⠄⣾⠀⠀⠀⠀
            ⠀⠀⠀⠀⠀⠃⠀⠞⠀⣀⣀⣀⣀⣀⣀⣀⣤⣤⣶⣿⣇⢧⠀⣿⠀⠀⠀⠀
            ⠀⠀⠀⠀⠀⡄⠀⠀⠀⠈⠫⢽⣽⣉⣹⣁⣧⣿⠟⣱⣿⣾⢀⣿⠀⠀⠀⠀
            ⠀⠀⠀⠀⠀⢃⠀⠀⠀⠀⠀⠀⠉⠙⠩⠤⠭⣶⣋⡟⢸⢁⣿⠏⠀⠀⠀⠀
            ⠀⠀⠀⠀⠀⠀⠱⡀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠛⠝⡇⣘⡾⠋⠀⠀⠀⠀⠀
            ⠀⠀⠀⠀⠀⠀⠀⠈⠢⣀⠀⠀⠀⠀⠀⠀⠀⣠⣴⣷⠋⠀⠀⠀⠀⠀⠀⠀
            ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠑⠠⠤⠤⠤⠤⠾⠟⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀
        "#;

        Player {
            remaining_health_points: PLAYER_BASE_HEALTH_POINT * 2i32.pow(level as u32),
            total_health_points: PLAYER_BASE_HEALTH_POINT * 2i32.pow(level as u32),
            base_damage: 1..=PLAYER_BASE_RANGE_MAX_POINT + level as i32,
            name,
            level,
            experience: 0,
            experience_to_level_up: PLAYER_BASE_EXPERIENCE_NECESSARY * 2i32.pow(level as u32),
            image: portrait.to_string(),
            inventory: Vec::new(),
        }
    }
}
