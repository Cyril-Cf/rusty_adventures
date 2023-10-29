use crate::GameState;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::*,
    style::Style,
    widgets::*,
    Frame,
};

pub fn render_fights_ui(frame: &mut Frame, state: &mut GameState, area: Rect) {
    const FIGHTS_BAR: &str = " Fights ";

    frame.render_widget(
        Block::default().borders(Borders::ALL).title(FIGHTS_BAR),
        area,
    );

    let inner_fight_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(85), Constraint::Percentage(15)])
        .margin(1)
        .split(area);

    // TOP PART
    let monster_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(inner_fight_layout[0]);

    let stats = vec![
        Line::from(vec![
            Span::raw("Name: "),
            Span::styled(&state.current_monster.name, Style::new()),
        ]),
        Line::from(vec![
            Span::raw("Level: "),
            Span::styled(
                state.current_monster.level.to_string(),
                Style::new().green(),
            ),
        ]),
        Line::from(vec![
            Span::raw("Health: "),
            Span::styled(
                state.current_monster.health_points.to_string(),
                Style::new().green(),
            ),
        ]),
        Line::from(vec![
            Span::raw("Damage: "),
            Span::styled(
                state.current_monster.base_damage.start().to_string(),
                Style::new().green(),
            ),
            Span::raw(" - "),
            Span::styled(
                state.current_monster.base_damage.end().to_string(),
                Style::new().green(),
            ),
        ]),
        Line::from(vec![
            Span::raw("Description: "),
            Span::styled(&state.current_monster.description, Style::new()),
        ]),
    ];

    frame.render_widget(
        Paragraph::new(stats)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).padding(Padding {
                left: 1,
                right: 1,
                top: 1,
                bottom: 1,
            })),
        monster_layout[0].inner(&Margin {
            vertical: 2,
            horizontal: 2,
        }),
    );

    frame.render_widget(
        Paragraph::new(state.current_monster.image.clone())
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL)),
        monster_layout[1].inner(&Margin {
            vertical: 2,
            horizontal: 2,
        }),
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
        .split(inner_fight_layout[1]);

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

        let text_case_button = Paragraph::new(button.0)
            .block(block)
            .alignment(Alignment::Center)
            .style(Style::default().fg(button.1));

        frame.render_widget(text_case_button, buttons_layout[index + 1]);
    }
}
