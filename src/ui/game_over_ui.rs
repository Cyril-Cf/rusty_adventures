use super::utils::*;
use crate::GameState;
use ratatui::{prelude::*, widgets::*};

pub fn render_game_over_ui(f: &mut Frame, state: &mut GameState) {
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(f.size());

    let title = Text::from(
        r#"
  _____  _    _  _____ _________     __  _____  _    _ _   _  _____ ______ ____  _   _ 
  |  __ \| |  | |/ ____|__   __\ \   / / |  __ \| |  | | \ | |/ ____|  ____/ __ \| \ | |
  | |__) | |  | | (___    | |   \ \_/ /  | |  | | |  | |  \| | |  __| |__ | |  | |  \| |
  |  _  /| |  | |\___ \   | |    \   /   | |  | | |  | | . ` | | |_ |  __|| |  | | . ` |
  | | \ \| |__| |____) |  | |     | |    | |__| | |__| | |\  | |__| | |___| |__| | |\  |
  |_|  \_\\____/|_____/   |_|     |_|    |_____/ \____/|_| \_|\_____|______\____/|_| \_|




                           ,--.
                          {    }
                          K,   }
                         /  `Y`
                    _   /   /
                   {_'-K.__/
                     `/-.__L._
                     /  ' /`\_}
                    /  ' /     
            ____   /  ' /
     ,-'~~~~    ~~/  ' /_
   ,'             ``~~~%%',
  (                     %  Y
 {                      %% I
{      -                 %  `.
|       ',                %  )
|        |   ,..__      __. Y
|    .,_./  Y ' / ^Y   J   )|
\           |' /   |   |   ||
 \          L_/    . _ (_,.'(
  \,   ,      ^^""' / |      )
    \_  \          /,L]     /
      '-_`-,       ` `   ./`
         `-(_            )
             ^^\..___,.--`
    "#,
    );

    f.render_widget(
        Paragraph::new(title).block(Block::default()),
        main_layout[1],
    );

    let central_area = repositioned_rect(f.size(), 45, 30, 30, 15);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(central_area);

    let text: Vec<Line<'_>> = vec![
        Line::from("GAME OVER!"),
        Line::from(vec![
            Span::styled(&state.current_monster.name, Style::new().bold().red()),
            " has killed you. Poor thing.".into(),
        ]),
        Line::from("Please, do come back and try again..."),
        Line::from(""),
        Line::from("..or not, if you're too afraid!"),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::raw("Press "),
            Span::styled("q", Style::new().bold().red()),
            " to quit".into(),
        ]),
    ];

    let content = Paragraph::new(text).wrap(Wrap { trim: true }).block(
        Block::default().borders(Borders::ALL).padding(Padding {
            left: 2,
            right: 2,
            top: 2,
            bottom: 2,
        }),
    );
    f.render_widget(Clear, chunks[1]);
    f.render_widget(content, chunks[1]);
}
