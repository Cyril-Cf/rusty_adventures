use crate::ui::consts::*;
use crate::ui::utils::centered_rect;
use crate::utils::game_state::ControlType;
use crate::GameState;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::*,
    style::Style,
    widgets::*,
    Frame,
};

pub fn render_inventory_popup(frame: &mut Frame, state: &mut GameState, area: Rect) {
    let popup_area = centered_rect(area, 80, 40);
    frame.render_widget(Clear, popup_area);

    let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
        .margin(1)
        .split(popup_area);

    let table = Table::new(
        state
            .player
            .inventory
            .clone()
            .into_iter()
            .map(|item| item.into())
            .collect::<Vec<Row>>(),
    )
    .style(Style::default().fg(Color::White))
    .header(
        Row::new(vec!["Item", "Description"])
            .style(Style::default().fg(Color::Yellow))
            .bottom_margin(1),
    )
    .block(Block::default())
    .widths(&[Constraint::Percentage(30), Constraint::Percentage(70)]);
    let mut list_state = TableState::default();
    frame.render_stateful_widget(table, inner_layout[0], &mut list_state);

    let buttons_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(inner_layout[1].inner(&Margin {
            vertical: 0,
            horizontal: 2,
        }));

    for (index, button) in INVENTORY_UI_BUTTONS.iter().enumerate() {
        let color = match &state.controls_type {
            ControlType::InventoryControls(button_selected) => {
                if button_selected == &button.2 {
                    button.1
                } else {
                    Color::White
                }
            }
            _ => Color::White,
        };
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(color));

        frame.render_widget(Paragraph::new("").block(block), buttons_layout[index]);

        let text_case_button = Paragraph::new(button.0)
            .block(Block::new())
            .alignment(Alignment::Center)
            .style(Style::default().fg(button.1));

        frame.render_widget(
            text_case_button,
            centered_rect(buttons_layout[index], 50, 50),
        );
    }
}
