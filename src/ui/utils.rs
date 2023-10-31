use ratatui::prelude::*;

pub fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn repositioned_rect(
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

pub struct FighterInfo {
    pub health_points: i32,
    pub base_damage: std::ops::RangeInclusive<i32>,
    pub experience: Option<i32>,
    pub level: usize,
    pub image: String,
    pub name: String,
    pub description: Option<String>,
    pub experience_to_level_up: Option<i32>,
}
pub trait FightInfo {
    fn get_fighter_info(&self) -> FighterInfo;
}
