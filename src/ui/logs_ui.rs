use crate::GameState;
use ratatui::{prelude::*, widgets::*};
pub fn render_logs_ui(frame: &mut Frame, state: &mut GameState, area: Rect) {
    const HISTORY_BAR: &str = " Logs ";

    frame.render_widget(
        Block::default().borders(Borders::ALL).title(HISTORY_BAR),
        area,
    );

    let vertical_scroll = 0;
    let paragraph = Paragraph::new(
        state
            .events
            .clone()
            .into_iter()
            .map(|event| event.into())
            .collect::<Vec<Line>>(),
    )
    .wrap(Wrap { trim: true })
    .scroll((state.scroll_state.current_scroll_line as u16, 0))
    .block(Block::new().borders(Borders::RIGHT));
    let scrollbar = Scrollbar::default()
        .orientation(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));
    let mut scrollbar_state =
        ScrollbarState::new(state.events.iter().len()).position(vertical_scroll);

    frame.render_widget(
        paragraph,
        area.inner(&Margin {
            vertical: 2,
            horizontal: 2,
        }),
    );
    frame.render_stateful_widget(
        scrollbar,
        area.inner(&Margin {
            vertical: 1,
            horizontal: 1,
        }),
        &mut scrollbar_state,
    );
}
