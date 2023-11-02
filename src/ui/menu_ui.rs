use super::utils::*;
use crate::GameState;
use ratatui::{prelude::*, widgets::*};

pub enum InputMode {
    Normal,
    Editing,
}

pub fn ui(f: &mut Frame, state: &mut GameState) {
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

                              _.--""-._
  .                         ."         ".
 / \    ,^.         /(     Y             |      )\
/   `---. |--'\    (  \__..'--   -   -- -'""-.-'  )
|        :|    `>   '.     l_..-------.._l      .'
|      __l;__ .'      "-.__.||_.-'v'-._||`"----"
 \  .-' | |  `              l._       _.'
  \/    | |                   l`^^'^^'j
        | |                _   \_____/     _
        j |               l `--__)-'(__.--' |
        | |               | /`---``-----'"1 |  ,-----.
        | |               )/  `--' '---'   \'-'  ___  `-.
        | |              //  `-'  '`----'  /  ,-'   I`.  \
      _ L |_            //  `-.-.'`-----' /  /  |   |  `. \
     '._' / \         _/(   `/   )- ---' ;  /__.J   L.__.\ :
      `._;/7(-.......'  /        ) (     |  |            | |
      `._;l _'--------_/        )-'/     :  |___.    _._./ ;
        | |                 .__ )-'\  __  \  \  I   1   / /
        `-'                /   `-\-(-'   \ \  `.|   | ,' /
                           \__  `-'    __/  `-. `---'',-'
                              )-._.-- (        `-----'
                             )(  l\ o ('..-.
                       _..--' _'-' '--'.-. |
                __,,-'' _,,-''            \ \
               f'. _,,-'                   \ \
              ()--  |                       \ \
                \.  |                       /  \
                  \ \                      |._  |
                   \ \                     |  ()|
                    \ \                     \  /
                     ) `-.                   | |
                    // .__)                  | |
                 _.//7'                      | |
               '---'                         j_| `
                                            (| |
                                             |  \
                                             |lllj
                                             |||||
    "#,
    );

    f.render_widget(
        Paragraph::new(title).block(Block::default()),
        main_layout[1],
    );

    let central_area = repositioned_rect(f.size(), 45, 30, 30, 25);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(central_area);

    let (msg, style) = match state.player_choice.input_mode {
        InputMode::Normal => (
            vec!["Press ".into(), "e".bold(), " to change it.".bold()],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                "Press ".into(),
                "Esc".bold(),
                " to stop editing, ".into(),
                "Enter".bold(),
                " to apply".into(),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Line::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(state.player_choice.input.as_str())
        .style(match state.player_choice.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Your name here "),
        );
    f.render_widget(input, chunks[1]);
    match state.player_choice.input_mode {
        InputMode::Normal => {}
        InputMode::Editing => f.set_cursor(
            chunks[1].x + state.player_choice.cursor_position as u16 + 1,
            chunks[1].y + 1,
        ),
    }

    let text: Vec<Line<'_>> = match state.player.name.as_str() {
        "" => vec![Line::from("Please chose a name!")],
        _ => {
            vec![
                Line::from(vec![
                    Span::raw("Your hero name will be "),
                    Span::styled(&state.player.name, Style::new().bold().red()),
                    ".".into(),
                ]),
                Line::from("Are you sure you want to proceed?"),
                Line::from(""),
                Line::from(vec![
                    Span::raw("Press the "),
                    Span::styled("spacebar", Style::new().bold().green()),
                    " to enter the dungeon".into(),
                ]),
            ]
        }
    };

    let name = Paragraph::new(text).wrap(Wrap { trim: true }).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Your hero ")
            .padding(Padding {
                left: 1,
                right: 1,
                top: 1,
                bottom: 1,
            }),
    );
    f.render_widget(Clear, chunks[2]);
    f.render_widget(name, chunks[2]);
}
