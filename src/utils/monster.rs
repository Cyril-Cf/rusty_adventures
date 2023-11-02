use super::consts::*;
use super::game_state::*;
use super::player::*;
use crate::ui::utils::{FightInfo, FighterInfo};
use rand::Rng;

#[derive(Clone)]
pub enum HealthPotion {
    SmallPotion,
    MediumPotion,
    GiantPotion,
}

#[derive(Clone)]
pub enum Item {
    Potion(HealthPotion),
}

pub trait ItemDetails {
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
}

impl ItemDetails for Item {
    fn get_name(&self) -> String {
        match self {
            Item::Potion(potion) => potion.get_name(),
        }
    }

    fn get_description(&self) -> String {
        match self {
            Item::Potion(potion) => potion.get_description(),
        }
    }
}

impl ItemDetails for HealthPotion {
    fn get_name(&self) -> String {
        match self {
            HealthPotion::SmallPotion => String::from("Small health potion"),
            HealthPotion::MediumPotion => String::from("Medium health potion"),
            HealthPotion::GiantPotion => String::from("Giant health potion"),
        }
    }

    fn get_description(&self) -> String {
        match self {
            HealthPotion::SmallPotion => String::from("Restores 10 HP"),
            HealthPotion::MediumPotion => String::from("Restores 50 HP"),
            HealthPotion::GiantPotion => String::from("Restores all HP"),
        }
    }
}
#[derive(Clone)]
pub struct Loot {
    pub level_up: bool,
    pub item: Option<Item>,
}

#[derive(Clone)]

pub struct Monster {
    pub name: String,
    pub health_points: i32,
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
        self.health_points -= attack_damage;
    }
    fn get_health_points(&self) -> i32 {
        self.health_points
    }
}

impl FightInfo for Monster {
    fn get_fighter_info(&self) -> crate::ui::utils::FighterInfo {
        FighterInfo {
            base_damage: self.base_damage.clone(),
            description: Some(self.description.clone()),
            experience: None,
            experience_to_level_up: None,
            health_points: self.health_points,
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
        health_points: MONSTER_BASE_HEALTH_POINT * 2i32.pow(level as u32),
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
        health_points: MONSTER_BASE_HEALTH_POINT * 2i32.pow(level as u32),
        level: level as usize,
        loot: loot.clone(),
    }
}

const MONSTERS: [(&str, &str, &str, Loot); 10] = [
    (
        "Greta the Fierce",
        "Une redoutable guerrière des royaumes du nord, connue pour sa force et son courage inégalés.",
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
        "Un farceur rusé qui aime jouer des tours aux voyageurs égarés.",
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
        "Un monstre hérissé de pointes qui adore les câlins (attention, il pique !).",
        r#"
          /-.-\
        /  /  /\
        |o   o|
        \  ^  /  
          \ /    
        "#,
        Loot {
            level_up: false,
            item: Some(Item::Potion(HealthPotion::SmallPotion)),
        },
    ),
    (
        "Mystica the Enigmatic",
        "Une créature mystérieuse qui parle en énigmes et accorde des souhaits.",
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
        "Une boule de poils géante avec un appétit insatiable pour les friandises.",
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
        "Un monstre qui émet des bruits étranges et déroutants à tout moment.",
        r#"
         .---.
        /o o o\
        |  ~  |
        \ - - /
         '---'    
        "#,
        Loot {
            level_up: false,
            item: None
        },
    ),
    (
        "Glimmer the Shiny",
        "Une créature brillante qui attire l'attention avec son éclat éblouissant.",
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
        "Un gardien des rêves qui peut vous faire vivre vos rêves les plus fous.",
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
        "Un félin géant avec de longues moustaches qui prévoit la météo.",
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
        "Une créature gluante qui peut se transformer en n'importe quoi.",
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
