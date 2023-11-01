use super::fighter_ui::render_fighter_ui;
use super::utils::centered_rect;
use crate::utils::consts::FIGHTS_BAR;
use crate::{ui::utils::FightInfo, GameState};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::*,
    style::Style,
    widgets::*,
    Frame,
};

pub fn render_fights_ui(frame: &mut Frame, state: &mut GameState, area: Rect) {
    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title(format!("{} loop: {}", FIGHTS_BAR, state.loop_count)),
        area.inner(&Margin {
            horizontal: 2,
            vertical: 2,
        }),
    );

    let inner_fight_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(85), Constraint::Percentage(15)])
        .margin(1)
        .split(area.inner(&Margin {
            horizontal: 2,
            vertical: 2,
        }));

    let fighters_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(inner_fight_layout[0]);

    render_fighter_ui(
        frame,
        fighters_layout[0].inner(&Margin {
            horizontal: 1,
            vertical: 1,
        }),
        state.player.get_fighter_info(),
    );
    render_fighter_ui(
        frame,
        fighters_layout[1].inner(&Margin {
            horizontal: 1,
            vertical: 1,
        }),
        state.current_monster.get_fighter_info(),
    );

    // BOTTOM PART
    let buttons_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(55),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
        ])
        .split(inner_fight_layout[1].inner(&Margin {
            vertical: 0,
            horizontal: 2,
        }));

    let texts_case = [
        ("Attack", Color::Red, 1),
        ("Seduce", Color::LightRed, 2),
        ("Flee", Color::LightYellow, 3),
    ];

    for (index, button) in texts_case.iter().enumerate() {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(if state.selected_button == button.2 {
                button.1
            } else {
                Color::White
            }));

        frame.render_widget(Paragraph::new("").block(block), buttons_layout[index + 1]);

        let text_case_button = Paragraph::new(button.0)
            .block(Block::new())
            .alignment(Alignment::Center)
            .style(Style::default().fg(button.1));

        frame.render_widget(
            text_case_button,
            centered_rect(buttons_layout[index + 1], 80, 10),
        );
    }
}
