use ratatui::prelude::*;

pub fn centered_rect(
    r: Rect,
    percent_x_left: u16,
    percent_x_right: u16,
    percent_y_top: u16,
    percent_y_bottom: u16,
) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(percent_y_top),
            Constraint::Percentage(100 - percent_y_top - percent_y_bottom),
            Constraint::Percentage(percent_y_bottom),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(percent_x_left),
            Constraint::Percentage(100 - percent_x_left - percent_x_right),
            Constraint::Percentage(percent_x_right),
        ])
        .split(popup_layout[1])[1]
}
