use crate::ui::consts::*;
use crate::ui::utils::centered_rect;
use crate::utils::game_state::ControlType;
use crate::utils::monster::ItemDetails;
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

    let table = Table::new(vec![
        // Row can be created from simple strings.
        Row::new(vec!["Row11", "Row12", "Row13"]),
        // You can style the entire row.
        Row::new(vec!["Row21", "Row22", "Row23"]).style(Style::default().fg(Color::Blue)),
        // If you need more control over the styling you may need to create Cells directly
        Row::new(vec![
            Cell::from("Row31"),
            Cell::from("Row32").style(Style::default().fg(Color::Yellow)),
            Cell::from(Line::from(vec![
                Span::raw("Row"),
                Span::styled("33", Style::default().fg(Color::Green)),
            ])),
        ]),
        // If a Row need to display some content over multiple lines, you just have to change
        // its height.
        Row::new(vec![
            Cell::from("Row\n41"),
            Cell::from("Row\n42"),
            Cell::from("Row\n43"),
        ])
        .height(2),
    ])
    // You can set the style of the entire Table.
    .style(Style::default().fg(Color::White))
    // It has an optional header, which is simply a Row always visible at the top.
    .header(
        Row::new(vec!["Col1", "Col2", "Col3"])
            .style(Style::default().fg(Color::Yellow))
            // If you want some space between the header and the rest of the rows, you can always
            // specify some margin at the bottom.
            .bottom_margin(1),
    )
    // As any other widget, a Table can be wrapped in a Block.
    .block(Block::default().title("Table"))
    // Columns widths are constrained in the same way as Layout...
    .widths(&[
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(10),
    ])
    // ...and they can be separated by a fixed spacing.
    .column_spacing(1);
    let mut list_state = TableState::default();
    // If you wish to highligh
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
