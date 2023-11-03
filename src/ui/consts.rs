use crate::utils::game_state::{FightButtons, InventoryButtons, MonsterSlayedButtons};
use ratatui::prelude::Color;

pub const FIGHT_UI_BUTTONS: [(&str, Color, FightButtons); 4] = [
    ("Attack", Color::Red, FightButtons::Attack),
    ("Spell", Color::LightRed, FightButtons::Spell),
    ("Inventory", Color::LightYellow, FightButtons::Inventory),
    ("Flee", Color::DarkGray, FightButtons::Flee),
];

pub const MONSTER_SLAYED_UI_BUTTONS: [(&str, Color, MonsterSlayedButtons); 2] = [
    ("Continue", Color::Red, MonsterSlayedButtons::Continue),
    (
        "Stop your mission",
        Color::LightRed,
        MonsterSlayedButtons::Skip,
    ),
];

pub const INVENTORY_UI_BUTTONS: [(&str, Color, InventoryButtons); 2] = [
    ("Use", Color::Red, InventoryButtons::Use(0)),
    ("Cancel", Color::Blue, InventoryButtons::Cancel),
];
