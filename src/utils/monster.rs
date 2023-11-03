use super::consts::*;
use super::game_state::*;
use super::items::*;
use super::player::*;
use crate::ui::utils::{FightInfo, FighterInfo};
use rand::Rng;

#[derive(Clone)]
pub struct Loot {
    pub level_up: bool,
    pub item: Option<Item>,
}

#[derive(Clone)]

pub struct Monster {
    pub name: String,
    pub remaining_health_points: i32,
    pub total_health_points: i32,
    pub base_damage: std::ops::RangeInclusive<i32>,
    pub level: usize,
    pub description: String,
    pub experience_given: i32,
    pub image: String,
    pub loot: Loot,
}

impl Attack for Monster {
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

impl FightInfo for Monster {
    fn get_fighter_info(&self) -> crate::ui::utils::FighterInfo {
        FighterInfo {
            base_damage: self.base_damage.clone(),
            description: Some(self.description.clone()),
            experience: None,
            experience_to_level_up: None,
            remaining_health_points: self.remaining_health_points,
            total_health_points: self.total_health_points,
            image: self.image.clone(),
            level: self.level,
            name: self.name.clone(),
        }
    }
}

pub fn get_initial_monster() -> Monster {
    let mut rng = rand::thread_rng();
    let random_index: usize = rng.gen_range(0..MONSTERS.len());
    let selected_monster = MONSTERS.get(random_index).unwrap();
    let (name, description, image, loot) = selected_monster;
    let level = 1;

    Monster {
        name: name.to_string(),
        description: description.to_string(),
        image: image.to_string(),
        base_damage: 1..=MONSTER_BASE_RANGE_MAX_POINT * 2i32.pow(level as u32),
        experience_given: MONSTER_BASE_EXPERIENCE_GIVEN * 2i32.pow(level as u32),
        // health_points: MONSTER_BASE_HEALTH_POINT * 2i32.pow(level as u32),
        total_health_points: MONSTER_BASE_HEALTH_POINT * 2i32.pow(level as u32),
        remaining_health_points: 5,
        level: level as usize,
        loot: loot.clone(),
    }
}

pub fn get_random_monster(state: &mut GameState) -> Monster {
    let mut rng = rand::thread_rng();
    let random_index: usize = rng.gen_range(0..MONSTERS.len());
    let selected_monster = MONSTERS.get(random_index).unwrap();
    let (name, description, image, loot) = selected_monster;
    let level = state.player.level;

    Monster {
        name: name.to_string(),
        description: description.to_string(),
        image: image.to_string(),
        base_damage: 1..=MONSTER_BASE_RANGE_MAX_POINT + level as i32,
        experience_given: MONSTER_BASE_EXPERIENCE_GIVEN * 2i32.pow(level as u32),
        // health_points: MONSTER_BASE_HEALTH_POINT * 2i32.pow(level as u32),
        total_health_points: MONSTER_BASE_HEALTH_POINT * 2i32.pow(level as u32),
        remaining_health_points: 5,
        level: level as usize,
        loot: loot.clone(),
    }
}

const MONSTERS: [(&str, &str, &str, Loot); 10] = [
    (
        "Greta the Fierce",
        "A formidable warrior from the northern realms, known for her unmatched strength and courage.",
        r#"
        w*W*W*W*w
         \"."."/
          //`\\
         (/a a\)
         (\_-_/) 
        .-~'='~-.
       /`~`"Y"`~`\
      / /(_ * _)\ \
        "#,
        Loot {
            level_up: false,
            item: Some(Item::Potion(HealthPotion::SmallPotion)),
        },
    ),
    (
        "Zog the Mischievous",
        "A cunning trickster who enjoys playing pranks on lost travelers.",
        r#"
        .-^.
        | o |
        |   |
        | o |
        '---'
        "#,
        Loot {
            level_up: false,
            item: Some(Item::Potion(HealthPotion::SmallPotion)),
        },
    ),
    (
        "Spike the Spiky",
        "A spiky monster that loves hugs (caution, it's prickly!).",
        r#"
          /-.-\
        /  /  /\
        |o   o|
        \ ^ /
          \ /
        "#,
        Loot {
            level_up: false,
            item: Some(Item::Potion(HealthPotion::SmallPotion)),
        },
    ),
    (
        "Mystica the Enigmatic",
        "A mysterious creature that speaks in riddles and grants wishes.",
        r#"
        .-^-.
        |? ?|
        | * |
        |   |
        '-.-'
        "#,
        Loot {
            level_up: false,
            item: None,
        },
    ),
    (
        "Fluffy the Fluffball",
        "A giant furball with an insatiable appetite for treats.",
        r#"
         /^\
        /   \
        |o o|
        \ ^ /
         \_/
        "#,
        Loot {
            level_up: false,
            item: Some(Item::Potion(HealthPotion::MediumPotion)),
        },
    ),
    (
        "Squeaky the Noisy",
        "A monster that emits strange and confusing noises at all times.",
        r#"
         .---.
        /o o o\
        |  ~  |
        \ - - /
         '---'
        "#,
        Loot {
            level_up: false,
            item: None,
        },
    ),
    (
        "Glimmer the Shiny",
        "A shiny creature that captivates attention with its dazzling glow.",
        r#"
          /\
         /o\
        | * |
         \/
        "#,
        Loot {
            level_up: false,
            item: Some(Item::Potion(HealthPotion::SmallPotion)),
        },
    ),
    (
        "Sandy the Sandman",
        "A dream guardian who can make you experience your wildest dreams.",
        r#"
          .-.
         /o o\
        |  *  |
         \ - /
          '-'
        "#,
        Loot {
            level_up: false,
            item: Some(Item::Potion(HealthPotion::SmallPotion)),
        },
    ),
    (
        "Whiskers the Whiskered",
        "A giant feline with long whiskers that predicts the weather.",
        r#"
        /\_/\ 
       | ^ ^ |
        \ * / 
         \_/   
        "#,
        Loot {
            level_up: false,
            item: None,
        },
    ),
    (
        "Gloop the Gooey",
        "A gooey creature that can transform into anything.",
        r#"
        .---.
        |* *|
        | * |
        | * |
        '---'
        "#,
        Loot {
            level_up: false,
            item: Some(Item::Potion(HealthPotion::GiantPotion)),
        },
    ),
];
