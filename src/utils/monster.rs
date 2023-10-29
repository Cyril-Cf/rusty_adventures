use super::consts::*;
use super::game_state::*;
use super::player::*;
use rand::Rng;

pub struct Monster {
    pub name: String,
    pub health_points: i32,
    pub base_damage: std::ops::RangeInclusive<i32>,
    pub level: usize,
    pub description: String,
    pub experience_given: i32,
    pub image: String,
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

pub fn get_initial_monster() -> Monster {
    let mut rng = rand::thread_rng();
    let random_index: usize = rng.gen_range(0..MONSTERS.len());
    let selected_monster = MONSTERS[random_index];
    let (name, description, image) = selected_monster;
    let level = 1;

    Monster {
        name: name.to_string(),
        description: description.to_string(),
        image: image.to_string(),
        base_damage: 1..=MONSTER_BASE_RANGE_MAX_POINT * 2i32.pow(level as u32),
        experience_given: MONSTER_BASE_EXPERIENCE_GIVEN * 2i32.pow(level as u32),
        health_points: MONSTER_BASE_HEALTH_POINT * 2i32.pow(level as u32),
        level: level as usize,
    }
}

pub fn get_random_monster(state: &mut GameState) -> Monster {
    let mut rng = rand::thread_rng();
    let random_index: usize = rng.gen_range(0..MONSTERS.len());
    let selected_monster = MONSTERS[random_index];
    let (name, description, image) = selected_monster;
    let level = state.player.level;

    Monster {
        name: name.to_string(),
        description: description.to_string(),
        image: image.to_string(),
        base_damage: 1..=MONSTER_BASE_RANGE_MAX_POINT * 2i32.pow(level as u32),
        experience_given: MONSTER_BASE_EXPERIENCE_GIVEN * 2i32.pow(level as u32),
        health_points: MONSTER_BASE_HEALTH_POINT * 2i32.pow(level as u32),
        level: level as usize,
    }
}

const MONSTERS: [(&str, &str, &str); 15] = [
    (
        "Greta the Fierce",
        "A fearsome warrior from the northern realms, known for her unmatched strength and courage.",
        "
            ...ASCII art for Greta the Fierce...
        "
    ),
    (
        "Thornback Terror",
        "A legendary creature with thorny spikes on its back, said to guard ancient treasures.",
        "
            ...ASCII art for Thornback Terror...
        "
    ),
    (
        "Abyssal Serpent",
        "A monstrous sea serpent that lurks in the depths, leaving terror in its wake.",
        "
            ...ASCII art for Abyssal Serpent...
        "
    ),
    (
        "Blazing Phoenix",
        "A magnificent bird of flames, symbolizing rebirth and renewal.",
        "
            ...ASCII art for Blazing Phoenix...
        "
    ),
    (
        "Shadowmaw Reaper",
        "A mysterious reaper of souls that dwells in the shadows, collecting lost spirits.",
        "
            ...ASCII art for Shadowmaw Reaper...
        "
    ),
    (
        "Frostbite Yeti",
        "A giant yeti covered in ice, dwelling in the frosty mountains of the north.",
        "
            ...ASCII art for Frostbite Yeti...
        "
    ),
    (
        "Ironclad Goliath",
        "A colossal golem made of iron, a guardian of hidden treasures.",
        "
            ...ASCII art for Ironclad Goliath...
        "
    ),
    (
        "Wispwood Enchanter",
        "An enchanting spirit of the wispwood forest, protector of its magical secrets.",
        "
            ...ASCII art for Wispwood Enchanter...
        "
    ),
    (
        "Thunderhoof Minotaur",
        "A Minotaur with thunderous hooves, challenging all who enter its labyrinth.",
        "
            ...ASCII art for Thunderhoof Minotaur...
        "
    ),
    (
        "Duskwing Harpy",
        "A harpy with wings as dark as the night, a skilled hunter in the moonlight.",
        "
            ...ASCII art for Duskwing Harpy...
        "
    ),
    (
        "Venomspine Basilisk",
        "A basilisk with venomous spines, said to turn its prey into stone with a single glance.",
        "
            ...ASCII art for Venomspine Basilisk...
        "
    ),
    (
        "Stardust Dragon",
        "A celestial dragon from the starry skies, guarding ancient constellations.",
        "
            ...ASCII art for Stardust Dragon...
        "
    ),
    (
        "Molten Core Elemental",
        "An elemental creature born of molten lava, capable of burning everything in its path.",
        "
            ...ASCII art for Molten Core Elemental...
        "
    ),
    (
        "Moonshadow Assassin",
        "An elusive assassin who moves like a shadow in the moonlit night.",
        "
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
        "
    ),
    (
        "Crystalwing Pegasus",
        "A graceful Pegasus with crystal wings, a symbol of purity and grace.",
        "
            ...ASCII art for Crystalwing Pegasus...
        "
    ),
];
