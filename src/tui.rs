use std::time::Duration;

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph, Wrap},
    Frame, Terminal,
};

use crate::converter::{Converter, Mode};

pub fn run() -> Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    let mut mode = Mode::FarToCel;
    let mut converter = Converter::default();

    loop {
        terminal.draw(|f| render(f, &converter, &mode))?;

        match handle_input(&mut converter, &mode)? {
            Input::Mode(m) => mode = m,
            Input::None => {}
            Input::Quit => break,
        }
    }
    Ok(())
}

enum Input {
    Mode(Mode),
    Quit,
    None,
}

fn handle_input(converter: &mut Converter, current_mode: &Mode) -> Result<Input> {
    if !crossterm::event::poll(Duration::from_millis(250))? {
        return Ok(Input::None);
    }

    let key = match crossterm::event::read()? {
        crossterm::event::Event::Key(key) => key,
        _ => return Ok(Input::None),
    };

    if key.kind != KeyEventKind::Press {
        return Ok(Input::None);
    }

    match key.code {
        KeyCode::Enter => {
            converter.calculate(current_mode);
            Ok(Input::None)
        }
        KeyCode::Backspace => {
            converter.clear();
            Ok(Input::None)
        }
        KeyCode::Char('q') => Ok(Input::Quit),
        KeyCode::Char(' ') => Ok(match current_mode {
            Mode::FarToCel => Input::Mode(Mode::CelToFar),
            Mode::CelToFar => Input::Mode(Mode::FarToCel),
        }),
        KeyCode::Char(typed) => {
            converter.input_char(typed);
            Ok(Input::None)
        }
        _ => return Ok(Input::None),
    }
}

fn render(f: &mut Frame, converter: &Converter, mode: &Mode) {
    let outer_layout = Rect::new(1, 1, 60, 9);

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(Rect::new(2, 3, 58, 3));

    let hint = Paragraph::new(Line::from(vec![
        "<0-9>".green(),
        ": input number; ".white(),
        "<Enter>".green(),
        ": calculate; ".white(),
        "<Backspace>".green(),
        ": clear; ".white(),
        "<Space>".green(),
        ": swap units; ".white(),
        "<q>".green(),
        ": quit".white(),
    ]))
    .wrap(Wrap { trim: false });

    let outer_block = Block::bordered()
        .title("Farenheit/Celsius Converter".cyan())
        .title_alignment(Alignment::Center);

    let (input_title, output_title): (&str, &str) = match mode {
        Mode::FarToCel => ("Farenheit", "Celsius"),
        Mode::CelToFar => ("Celsius", "Farenheit"),
    };

    let input_pane =
        Paragraph::new(converter.raw_input()).block(Block::bordered().title(input_title));

    let output_pane =
        Paragraph::new(converter.calculated()).block(Block::bordered().title(output_title));

    f.render_widget(input_pane, inner_layout[0]);
    f.render_widget(output_pane, inner_layout[1]);

    f.render_widget(outer_block, outer_layout);
    f.render_widget(hint, Rect::new(2, 7, 58, 2))
}
