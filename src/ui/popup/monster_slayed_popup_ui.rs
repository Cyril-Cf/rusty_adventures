use crate::ui::consts::*;
use crate::ui::utils::centered_rect;
use crate::utils::game_state::ControlType;
use crate::utils::items::ItemActions;
use crate::GameState;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::*,
    style::Style,
    widgets::*,
    Frame,
};

pub fn render_monster_slayed_popup(frame: &mut Frame, state: &mut GameState, area: Rect) {
    let popup_area = centered_rect(area, 60, 80);
    frame.render_widget(Clear, popup_area);

    let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
        .margin(1)
        .split(popup_area);

    let mut text = vec![Line::from(format!(
        "Monster has been slain! You receive {} experience points.",
        state.current_monster.experience_given
    ))];

    if state.slained_monsters.last().unwrap().loot.level_up {
        text.push(Line::from(""));
        text.push(Line::from(vec![
            Span::raw("LEVEL UP! You are now level "),
            Span::styled(state.player.level.to_string(), Style::new().bold().red()),
            Span::raw("!"),
        ]));
    }

    if let Some(item) = &state.slained_monsters.last().unwrap().loot.item {
        text.push(Line::from(""));
        text.push(Line::from(vec![
            Span::raw("The enemy dropped "),
            Span::styled(item.get_name(), Style::new().green()),
            Span::raw(" ("),
            Span::styled(item.get_description(), Style::new().bold()),
            Span::raw(")!"),
        ]));
    }

    let paragraph = Paragraph::new(text)
        .block(Block::new().borders(Borders::ALL))
        .style(Style::new().white().on_black())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, inner_layout[0]);

    let buttons_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(inner_layout[1].inner(&Margin {
            vertical: 0,
            horizontal: 2,
        }));

    for (index, button) in MONSTER_SLAYED_UI_BUTTONS.iter().enumerate() {
        let color = match &state.controls_type {
            ControlType::MonsterSlayedControls(button_selected) => {
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
            centered_rect(buttons_layout[index], 50, 30),
        );
    }
}
