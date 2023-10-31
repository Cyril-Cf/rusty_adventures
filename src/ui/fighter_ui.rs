use crate::ui::utils::FighterInfo;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::*,
    style::Style,
    widgets::*,
    Frame,
};

pub fn render_fighter_ui(frame: &mut Frame, area: Rect, fighter_info: FighterInfo) {
    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" {} ", fighter_info.name.trim())),
        area,
    );

    let inner_fighter_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .margin(1)
        .split(area);

    frame.render_widget(
        Paragraph::new(fighter_info.image).block(Block::default().borders(Borders::ALL)),
        inner_fighter_layout[0].inner(&Margin {
            vertical: 2,
            horizontal: 2,
        }),
    );

    let mut stats = vec![
        Line::from(vec![
            Span::raw("Name: "),
            Span::styled(fighter_info.name, Style::new()),
        ]),
        Line::from(vec![
            Span::raw("Level: "),
            Span::styled(fighter_info.level.to_string(), Style::new().green()),
        ]),
        Line::from(vec![
            Span::raw("Health: "),
            Span::styled(fighter_info.health_points.to_string(), Style::new().green()),
        ]),
        Line::from(vec![
            Span::raw("Damage: "),
            Span::styled(
                fighter_info.base_damage.start().to_string(),
                Style::new().green(),
            ),
            Span::raw(" - "),
            Span::styled(
                fighter_info.base_damage.end().to_string(),
                Style::new().green(),
            ),
        ]),
    ];
    if let Some(description) = fighter_info.description {
        stats.push(Line::from(vec![
            Span::raw("Description: "),
            Span::styled(description, Style::new().green()),
        ]));
    }
    if let Some(experience) = fighter_info.experience {
        stats.push(Line::from(vec![
            Span::raw("Experience: "),
            Span::styled(experience.to_string(), Style::new().green()),
        ]));
    }
    if let Some(experience_to_level_up) = fighter_info.experience_to_level_up {
        stats.push(Line::from(vec![
            Span::raw("Experience to level up: "),
            Span::styled(experience_to_level_up.to_string(), Style::new().green()),
        ]));
    }

    frame.render_widget(
        Paragraph::new(stats)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL)),
        inner_fighter_layout[1].inner(&Margin {
            vertical: 1,
            horizontal: 1,
        }),
    );
}
