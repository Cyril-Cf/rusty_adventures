use std::time::Instant;

use super::fight::*;
use super::items::ItemActions;
use super::monster::*;
use super::player::*;
use crate::ui::consts::{FIGHT_UI_BUTTONS, INVENTORY_UI_BUTTONS, MONSTER_SLAYED_UI_BUTTONS};
use crate::ui::menu_ui::InputMode;
use ratatui::prelude::*;

#[derive(Clone)]
pub struct GameEvent {
    pub roll: Option<String>,
    pub description: String,
    pub bool_enemy_turn: Option<bool>,
    pub timestamp: Instant,
}

impl GameEvent {
    pub fn neutral(description: &str) -> Self {
        GameEvent {
            roll: None,
            description: description.to_string(),
            bool_enemy_turn: None,
            timestamp: Instant::now(),
        }
    }
    pub fn user_attack(description: &str, roll: &str) -> Self {
        GameEvent {
            roll: Some(roll.to_string()),
            description: description.to_string(),
            bool_enemy_turn: Some(false),
            timestamp: Instant::now(),
        }
    }
    pub fn monster_attack(description: &str, roll: &str) -> Self {
        GameEvent {
            roll: Some(roll.to_string()),
            description: description.to_string(),
            bool_enemy_turn: Some(true),
            timestamp: Instant::now(),
        }
    }
    pub fn switch_attack(description: &str, bool_enemy_turn: bool) -> Self {
        GameEvent {
            roll: None,
            description: description.to_string(),
            bool_enemy_turn: Some(bool_enemy_turn),
            timestamp: Instant::now(),
        }
    }
}

impl From<GameEvent> for Line<'_> {
    fn from(event: GameEvent) -> Line<'static> {
        let mut spans: Vec<Span> = Vec::new();
        if let Some(roll) = event.roll {
            match roll.as_str() {
                "0" => {}
                _ => {
                    spans.push(Span::styled(
                        "Roll: ",
                        Style::default().fg(Color::LightYellow),
                    ));
                    spans.push(Span::styled(
                        roll,
                        Style::default().add_modifier(Modifier::BOLD),
                    ));
                    spans.push(Span::raw(". "));
                }
            }
        }
        let mut style = Style::default();
        if let Some(bool_enemy_turn) = event.bool_enemy_turn {
            if bool_enemy_turn {
                style = style.red();
            } else {
                style = style.green();
            }
        }
        let chars_to_print = (event.timestamp.elapsed().as_secs_f32() * 15.0) as usize;
        let current_content: String = event.description.chars().take(chars_to_print).collect();
        spans.push(Span::styled(current_content, style));
        let mut line = Line::from(spans);
        if let Some(bool_enemy_turn) = event.bool_enemy_turn {
            if bool_enemy_turn {
                line = line.alignment(Alignment::Right);
            } else {
                line = line.alignment(Alignment::Left);
            }
        }
        line
    }
}

pub struct ScrollState {
    pub current_scroll_line: i32,
}

pub struct PlayerChoice {
    pub input: String,
    pub cursor_position: usize,
    pub input_mode: InputMode,
    pub messages: Vec<String>,
}

#[derive(Copy, Clone, Debug)]
pub enum ControlType {
    FightControls(FightButtons),
    MonsterSlayedControls(MonsterSlayedButtons),
    InventoryControls(InventoryButtons),
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum InventoryButtons {
    Use(usize),
    Cancel,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MonsterSlayedButtons {
    Continue,
    Skip,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FightButtons {
    Attack,
    Inventory,
    Spell,
    Flee,
}

trait IncrementDecrement {
    fn increment_horizontal(&self, current: usize) -> Self;
    fn decrement_horizontal(&self, current: usize) -> Self;
    fn increment_vertical(&self, current: usize, state: &GameState) -> Self;
    fn decrement_vertical(&self, current: usize, state: &GameState) -> Self;
}

impl IncrementDecrement for ControlType {
    fn increment_horizontal(&self, current: usize) -> Self {
        match self {
            ControlType::FightControls(_) => {
                if let Some(button) = FIGHT_UI_BUTTONS.get(current + 1) {
                    return ControlType::FightControls(button.2);
                };
                *self
            }
            ControlType::MonsterSlayedControls(_) => {
                if let Some(button) = MONSTER_SLAYED_UI_BUTTONS.get(current + 1) {
                    return ControlType::MonsterSlayedControls(button.2);
                };
                *self
            }
            ControlType::InventoryControls(_) => {
                if let Some(button) = INVENTORY_UI_BUTTONS.get(current + 1) {
                    return ControlType::InventoryControls(button.2);
                };
                *self
            }
        }
    }

    fn increment_vertical(&self, current: usize, state: &GameState) -> Self {
        match self {
            ControlType::FightControls(_) => *self,
            ControlType::MonsterSlayedControls(_) => *self,
            ControlType::InventoryControls(_) => {
                if current <= state.player.inventory.len() {
                    if let Some(_) = state.player.inventory.get(current + 1) {
                        return ControlType::InventoryControls(InventoryButtons::Use(current + 1));
                    };
                }
                *self
            }
        }
    }

    fn decrement_horizontal(&self, current: usize) -> Self {
        match self {
            ControlType::FightControls(_) => {
                if current > 0 {
                    if let Some(button) = FIGHT_UI_BUTTONS.get(current - 1) {
                        return ControlType::FightControls(button.2);
                    };
                }
                *self
            }
            ControlType::MonsterSlayedControls(_) => {
                if current > 0 {
                    if let Some(button) = MONSTER_SLAYED_UI_BUTTONS.get(current - 1) {
                        return ControlType::MonsterSlayedControls(button.2);
                    };
                }
                *self
            }
            ControlType::InventoryControls(_) => {
                if current > 0 {
                    if let Some(button) = INVENTORY_UI_BUTTONS.get(current - 1) {
                        return ControlType::InventoryControls(button.2);
                    };
                }
                *self
            }
        }
    }

    fn decrement_vertical(&self, current: usize, state: &GameState) -> Self {
        match self {
            ControlType::FightControls(_) => *self,
            ControlType::MonsterSlayedControls(_) => *self,
            ControlType::InventoryControls(_) => {
                if current > 0 {
                    if let Some(_) = state.player.inventory.get(current - 1) {
                        return ControlType::InventoryControls(InventoryButtons::Use(current - 1));
                    };
                }
                *self
            }
        }
    }
}

pub enum PopupType {
    MonsterSlayed,
    Inventory,
}

pub struct GameState {
    pub selected_button: usize,
    pub player: Player,
    pub scroll_state: ScrollState,
    pub events: Vec<GameEvent>,
    pub current_monster: Monster,
    pub slained_monsters: Vec<Monster>,
    pub player_inputs_accepted: bool,
    pub game_over: bool,
    pub player_choice: PlayerChoice,
    pub popup_type: Option<PopupType>,
    pub controls_type: ControlType,
}

impl Default for GameState {
    fn default() -> Self {
        let mut events: Vec<GameEvent> = Vec::new();
        initiate_logs(&mut events);

        let player = Player::create_player("".to_string());

        GameState {
            selected_button: 1,
            current_monster: get_initial_monster(),
            player,
            scroll_state: ScrollState {
                current_scroll_line: 0,
            },
            events,
            slained_monsters: Vec::new(),
            player_inputs_accepted: false,
            game_over: false,
            player_choice: PlayerChoice {
                input: String::new(),
                input_mode: InputMode::Editing,
                messages: Vec::new(),
                cursor_position: 0,
            },
            popup_type: None,
            controls_type: ControlType::FightControls(FightButtons::Attack),
        }
    }
}

impl GameState {
    // GAME

    pub fn initiate(&mut self) {
        start_new_battle(self);
        let bool_player_starts = roll_initiative(self);
        if !bool_player_starts {
            roll_attack(self, true);
            check_for_death(self);
            switch_attack_turn(self, !bool_player_starts);
        }
    }

    pub fn let_player_attack(&mut self) {
        if self.player_inputs_accepted && !self.game_over {
            roll_attack(self, false);
            check_for_death(self);

            if !self.game_over && self.current_monster.remaining_health_points > 0 {
                self.let_monster_attack();
            }
        }
    }

    pub fn let_monster_attack(&mut self) {
        switch_attack_turn(self, false);
        roll_attack(self, true);
        let bool_death_occured = check_for_death(self);
        if !bool_death_occured {
            switch_attack_turn(self, true);
        }
    }

    pub fn add_event(&mut self, event: GameEvent) {
        self.events.push(event);
        if self.events.len() > 30 {
            self.scroll_state.current_scroll_line = self.events.len() as i32 - 30;
        }
    }

    pub fn move_horizontal(&mut self, value: i32) {
        match &self.controls_type {
            ControlType::FightControls(button_selected) => {
                if let Some(current_index) = FIGHT_UI_BUTTONS
                    .iter()
                    .position(|&(_, _, b)| b == *button_selected)
                {
                    match value {
                        -1 => {
                            self.controls_type =
                                self.controls_type.decrement_horizontal(current_index);
                        }
                        1 => {
                            self.controls_type =
                                self.controls_type.increment_horizontal(current_index);
                        }
                        _ => panic!("Value must be -1 or 1"),
                    }
                };
            }
            ControlType::MonsterSlayedControls(button_selected) => {
                if let Some(current_index) = MONSTER_SLAYED_UI_BUTTONS
                    .iter()
                    .position(|&(_, _, b)| b == *button_selected)
                {
                    match value {
                        -1 => {
                            self.controls_type =
                                self.controls_type.decrement_horizontal(current_index);
                        }
                        1 => {
                            self.controls_type =
                                self.controls_type.increment_horizontal(current_index);
                        }
                        _ => panic!("Value must be -1 or 1"),
                    }
                };
            }
            ControlType::InventoryControls(button_selected) => {
                if let Some(current_index) = INVENTORY_UI_BUTTONS.iter().position(|&(_, _, b)| {
                    match (&b, &button_selected) {
                        (&InventoryButtons::Cancel, &InventoryButtons::Cancel) => true,
                        (&InventoryButtons::Use(_), &InventoryButtons::Use(_)) => true,
                        _ => false,
                    }
                }) {
                    match value {
                        -1 => {
                            self.controls_type =
                                self.controls_type.decrement_horizontal(current_index);
                        }
                        1 => {
                            self.controls_type =
                                self.controls_type.increment_horizontal(current_index);
                        }
                        _ => panic!("Value must be -1 or 1"),
                    }
                };
            }
        }
    }

    pub fn move_vertical(&mut self, value: i32) {
        match &self.controls_type {
            ControlType::FightControls(_) => match value {
                -1 => {
                    if self.scroll_state.current_scroll_line != 0 {
                        self.scroll_state.current_scroll_line -= 1;
                    }
                }
                1 => {
                    if self.scroll_state.current_scroll_line != self.events.len() as i32 {
                        self.scroll_state.current_scroll_line += 1;
                    }
                }
                _ => panic!("Value must be -1 or 1"),
            },
            ControlType::InventoryControls(button_selected) => match button_selected {
                InventoryButtons::Use(current_index) => match value {
                    -1 => {
                        self.controls_type =
                            self.controls_type.decrement_vertical(*current_index, &self);
                    }
                    1 => {
                        self.controls_type =
                            self.controls_type.increment_vertical(*current_index, &self);
                    }
                    _ => panic!("Value must be -1 or 1"),
                },
                _ => {}
            },
            _ => {}
        }
    }

    pub fn select_button(&mut self) {
        match &self.controls_type {
            ControlType::FightControls(selected_button) => match selected_button {
                FightButtons::Attack => {
                    self.let_player_attack();
                }
                FightButtons::Inventory => {
                    self.controls_type = ControlType::InventoryControls(InventoryButtons::Cancel);
                    self.popup_type = Some(PopupType::Inventory);
                }
                _ => {}
            },
            ControlType::MonsterSlayedControls(selected_button) => {
                if selected_button == &MonsterSlayedButtons::Continue {
                    self.controls_type = ControlType::FightControls(FightButtons::Attack);
                    self.popup_type = None;
                    self.events = Vec::new();
                    self.initiate();
                }
            }
            ControlType::InventoryControls(selected_button) => match selected_button {
                InventoryButtons::Cancel => {
                    self.controls_type = ControlType::FightControls(FightButtons::Attack);
                    self.popup_type = None;
                }
                InventoryButtons::Use(item_index) => {
                    if !self.player.inventory.is_empty() {
                        if item_index >= &0 {
                            let item = self.player.inventory.remove(*item_index);
                            item.use_item(self);
                            self.add_event(GameEvent::neutral(&format!(
                                "{} has been used !({})",
                                item.get_name(),
                                item.get_description()
                            )));
                            self.controls_type = ControlType::FightControls(FightButtons::Attack);
                            self.popup_type = None;
                            self.let_monster_attack();
                        }
                    }
                }
            },
        }
    }

    // MENU

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.player_choice.cursor_position.saturating_sub(1);
        self.player_choice.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.player_choice.cursor_position.saturating_add(1);
        self.player_choice.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        self.player_choice
            .input
            .insert(self.player_choice.cursor_position, new_char);

        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.player_choice.cursor_position != 0;
        if is_not_cursor_leftmost {
            let current_index = self.player_choice.cursor_position;
            let from_left_to_current_index = current_index - 1;
            let before_char_to_delete = self
                .player_choice
                .input
                .chars()
                .take(from_left_to_current_index);
            let after_char_to_delete = self.player_choice.input.chars().skip(current_index);
            self.player_choice.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.player_choice.input.len())
    }

    pub fn reset_cursor(&mut self) {
        self.player_choice.cursor_position = 0;
    }

    pub fn submit_name(&mut self) {
        self.player.name = self.player_choice.input.clone();
        self.player_choice.input.clear();
        self.reset_cursor();
        self.player_choice.input_mode = InputMode::Normal;
    }
}
