use super::fight::*;
use super::monster::*;
use super::player::*;
use crate::ui::menu_ui::InputMode;
use ratatui::prelude::*;

#[derive(Clone)]
pub struct GameEvent {
    pub roll: Option<String>,
    pub description: String,
    pub bool_enemy_turn: Option<bool>,
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
        spans.push(Span::styled(event.description, style));
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
        }
    }
}

impl GameState {
    pub fn initiate(&mut self) {
        start_new_battle(self);
        let bool_player_starts = roll_initiative(self);
        if !bool_player_starts {
            roll_attack(self, true);
            check_for_death(self);
            switch_attack_turn(self, !bool_player_starts);
        }
    }

    pub fn add_event(&mut self, event: GameEvent) {
        self.events.push(event);

        // TRIED TO ADD DELAY TO CHAR INSERT - NOT FUNCTIONAL
        // self.events.push(GameEvent {
        //     description: " ".to_owned(),
        // });
        // let delay = std::time::Duration::from_millis(1);

        // if let Some(last_event) = self.events.last_mut() {
        //     let last_description = &mut last_event.description;
        //     for c in event.description.chars() {
        //         if let Some(_) = last_description.pop() {
        //             last_description.push(c);
        //             last_description.push(' ');
        //             std::thread::sleep(delay);
        //         }
        //     }
        // }

        // TODO: GET VERTICAL SIZE DYNAMICALLY
        if self.events.len() > 35 {
            self.scroll_state.current_scroll_line = self.events.len() as i32 - 35;
        }
    }

    pub fn move_horizontal(&mut self, value: i32) {
        match value {
            -1 => {
                if self.selected_button != 1 {
                    self.selected_button -= 1;
                }
            }
            1 => {
                if self.selected_button != 3 {
                    self.selected_button += 1;
                }
            }
            _ => panic!("Value must be -1 or 1"),
        }
    }

    pub fn move_vertical(&mut self, value: i32) {
        match value {
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
        }
    }

    pub fn select_button(&mut self) {
        if self.selected_button == 1 && self.player_inputs_accepted && !self.game_over {
            roll_attack(self, false);
            let bool_death_occured = check_for_death(self);

            if bool_death_occured && !self.game_over {
                self.initiate();
            } else if !self.game_over {
                switch_attack_turn(self, false);
                roll_attack(self, true);
                let bool_death_occured = check_for_death(self);
                if !bool_death_occured {
                    switch_attack_turn(self, true);
                }
            }
        }
    }

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
