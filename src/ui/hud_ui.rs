use crate::GameState;
use ratatui::prelude::Rect;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::*,
    style::Style,
    widgets::*,
    Frame,
};

pub fn render_hud_ui(frame: &mut Frame, state: &mut GameState, area: Rect) {
    const HUD_BAR: &str = " My character ";

    frame.render_widget(Block::new().borders(Borders::TOP).title(HUD_BAR), area);

    let inner_layout = Layout::default()
        .horizontal_margin(2)
        .vertical_margin(2)
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(40),
        ])
        .split(area);

    let stats = vec![
        Line::from(vec![
            Span::raw("Name: "),
            Span::styled(&state.player.name, Style::new()),
        ]),
        Line::from(vec![
            Span::raw("Level: "),
            Span::styled(state.player.level.to_string(), Style::new().green()),
        ]),
        Line::from(vec![
            Span::raw("Experience: "),
            Span::styled(state.player.experience.to_string(), Style::new().green()),
        ]),
        Line::from(vec![
            Span::raw("Next level: "),
            Span::styled(
                state.player.experience_to_level_up.to_string(),
                Style::new().green(),
            ),
        ]),
        Line::from(vec![
            Span::raw("Health: "),
            Span::styled(state.player.health_points.to_string(), Style::new().green()),
        ]),
    ];

    frame.render_widget(
        Paragraph::new(stats).alignment(Alignment::Left).block(
            Block::default().borders(Borders::ALL).padding(Padding {
                left: 1,
                right: 1,
                top: 0,
                bottom: 0,
            }),
        ),
        inner_layout[1],
    );

    let shrek = Text::from(
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
    ",
    );

    frame.render_widget(
        Paragraph::new(shrek).block(Block::default().borders(Borders::ALL)),
        inner_layout[0],
    );
}
