mod scrollbar_state;

use crate::md_reader::scrollbar_state::RScrollbarState;
use ratatui::DefaultTerminal;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::*;
use ratatui::widgets::{Paragraph, Scrollbar, ScrollbarOrientation, Wrap};
use std::time::Duration;

pub fn md_reader(contents: &str) -> Result<(), String> {
    let terminal = ratatui::init();
    let markdown = tui_markdown::from_str(contents);
    run(terminal, markdown).map_err(|e| format!("E58 Failed to run ratatui app: {e}"))?;
    ratatui::restore();
    Ok(())
}

pub fn run(mut terminal: DefaultTerminal, text: Text) -> Result<(), String> {
    let mut state = RScrollbarState::new(text.height());
    loop {
        if event::poll(Duration::from_millis(15))
            .map_err(|e| format!("Failed to read crossterm events({e})"))?
        {
            match event::read() {
                Ok(event::Event::Key(key)) if key.kind == event::KeyEventKind::Press => {
                    if !on_key(key, &mut state) {
                        break;
                    }
                }
                _ => {}
            }
        }

        terminal
            .draw(|frame| {
                let [body, scrollbar] =
                    Layout::horizontal([Constraint::Fill(1), Constraint::Length(1)])
                        .areas(frame.area());
                state.view_height = body.height as usize;

                let scroll_pos = state
                    .position
                    .min(text.height().saturating_sub(state.view_height))
                    as u16;
                Paragraph::new(text.clone())
                    .scroll((scroll_pos, 0))
                    .wrap(Wrap { trim: false })
                    .render(body, frame.buffer_mut());
                Scrollbar::new(ScrollbarOrientation::VerticalRight).render(
                    scrollbar,
                    frame.buffer_mut(),
                    &mut (&mut state).into(),
                );
            })
            .map_err(|e| format!("Failed to draw ratatui frame ({e})"))?;
    }
    Ok(())
}

fn on_key(key: KeyEvent, state: &mut RScrollbarState) -> bool {
    use ratatui::crossterm::event::KeyCode::*;
    match (key.modifiers, key.code) {
        (_, Char('k') | Up) => state.up(),
        (_, Char('j') | Down) => state.down(),
        (_, Char('g') | Home) => state.top(),
        (_, Char('G') | End) => state.bottom(),
        (_, Char('b')) | (_, PageUp) | (KeyModifiers::SHIFT, Char(' ')) => state.view_height_up(),
        (_, Char('f')) | (_, PageDown) | (KeyModifiers::NONE, Char(' ')) => {
            state.view_height_down()
        }
        (_, Char('q')) | (_, Esc) | (KeyModifiers::CONTROL, Char('c')) => {
            return false;
        }
        _ => {}
    }
    true
}
