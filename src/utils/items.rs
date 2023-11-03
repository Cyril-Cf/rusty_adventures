use super::consts::PLAYER_BASE_HEALTH_POINT;
use crate::GameState;
use ratatui::widgets::Row;

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

pub trait ItemActions {
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
    fn use_item(&self, state: &mut GameState);
}

impl ItemActions for Item {
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

    fn use_item(&self, state: &mut GameState) {
        match self {
            Item::Potion(potion) => potion.use_item(state),
        }
    }
}

impl ItemActions for HealthPotion {
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

    fn use_item(&self, state: &mut GameState) {
        match self {
            HealthPotion::SmallPotion => {
                if state.player.remaining_health_points + 10 > state.player.total_health_points {
                    state.player.remaining_health_points = state.player.total_health_points;
                } else {
                    state.player.remaining_health_points += 10
                }
            }
            HealthPotion::MediumPotion => {
                if state.player.remaining_health_points + 50 > state.player.total_health_points {
                    state.player.remaining_health_points = state.player.total_health_points;
                } else {
                    state.player.remaining_health_points += 50
                }
            }
            HealthPotion::GiantPotion => {
                state.player.remaining_health_points = state.player.total_health_points;
            }
        }
    }
}

impl From<Item> for Row<'_> {
    fn from(item: Item) -> Row<'static> {
        Row::new(vec![item.get_name(), item.get_description()])
    }
}
