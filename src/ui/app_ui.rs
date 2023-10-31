use super::fights_ui::*;
// use super::hud_ui::*;
use super::logs_ui::*;
use crate::GameState;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_app_ui(frame: &mut Frame, state: &mut GameState) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(95), Constraint::Percentage(5)])
        .split(frame.size());

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(main_layout[0]);

    render_fights_ui(frame, state, inner_layout[0]);
    render_logs_ui(frame, state, inner_layout[1]);

    let footer = Paragraph::new("Select options: ← → Enter | Scroll: 🠗 🠕 | quit: <q>")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Center);

    frame.render_widget(footer, main_layout[1]);
}
