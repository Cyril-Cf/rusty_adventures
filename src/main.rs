use crossterm::ExecutableCommand;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::io::{self, stdout};
use ui::app_ui::render_app_ui;
use ui::game_over_ui::render_game_over_ui;
use ui::menu_ui::*;
use utils::game_state::GameState;

mod ui;
mod utils;

fn main() -> io::Result<()> {
    let mut game_state = GameState::default();

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    run_menu(&mut terminal, &mut game_state)?;

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    run_app(&mut terminal, &mut game_state)?;

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    run_game_over(&mut terminal, &mut game_state)
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    state: &mut GameState,
) -> io::Result<()> {
    state.initiate();
    loop {
        terminal.draw(|frame| render_app_ui(frame, state))?;

        if state.game_over {
            disable_raw_mode()?;
            stdout().execute(LeaveAlternateScreen)?;
            return Ok(());
        }

        state.loop_count += 1;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        stdout().execute(LeaveAlternateScreen)?;
                        return Ok(());
                    }
                    KeyCode::Left => state.move_horizontal(-1),
                    KeyCode::Right => state.move_horizontal(1),
                    KeyCode::Up => state.move_vertical(-1),
                    KeyCode::Down => state.move_vertical(1),
                    KeyCode::Enter => state.select_button(),
                    _ => {}
                }
            }
        }
    }
}

fn run_menu(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    state: &mut GameState,
) -> io::Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, state))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match state.player_choice.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('e') => {
                            state.player_choice.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char(' ') => {
                            if state.player.name != "" {
                                disable_raw_mode()?;
                                stdout().execute(LeaveAlternateScreen)?;
                                return Ok(());
                            }
                        }
                        _ => {}
                    },
                    InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Enter => state.submit_name(),
                        KeyCode::Char(to_insert) => {
                            state.enter_char(to_insert);
                        }
                        KeyCode::Backspace => {
                            state.delete_char();
                        }
                        KeyCode::Left => {
                            state.move_cursor_left();
                        }
                        KeyCode::Right => {
                            state.move_cursor_right();
                        }
                        KeyCode::Esc => {
                            state.player_choice.input_mode = InputMode::Normal;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
}

fn run_game_over(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    state: &mut GameState,
) -> io::Result<()> {
    loop {
        terminal.draw(|frame| render_game_over_ui(frame, state))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        stdout().execute(LeaveAlternateScreen)?;
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
    }
}
