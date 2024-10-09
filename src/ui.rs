use crate::history::HistoryEntry;
use chrono::{Local, TimeZone}; // Added TimeZone here
use crossterm::event::{self, Event as CEvent, KeyCode};
use std::io;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem, ListState};
use tui::Terminal;

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    items: &[HistoryEntry],
) -> io::Result<Option<String>> {
    let mut list_state = ListState::default();
    list_state.select(Some(0));

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);

            let list_items: Vec<ListItem> = items
                .iter()
                .map(|entry| {
                    let timestamp_str = entry
                        .timestamp
                        .map(|ts| {
                            let datetime = Local.from_local_datetime(&ts).unwrap();
                            datetime.format("%Y-%m-%d %H:%M:%S").to_string()
                        })
                        .unwrap_or_else(|| "No Timestamp".to_string());
                    let content = format!("{} - {}", timestamp_str, entry.command);
                    ListItem::new(Spans::from(content))
                })
                .collect();

            let list = List::new(list_items)
                .block(Block::default().borders(Borders::ALL).title("History"))
                .highlight_style(Style::default().bg(Color::Blue))
                .highlight_symbol(">> ");

            f.render_stateful_widget(list, chunks[0], &mut list_state);
        })?;

        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let CEvent::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Up => {
                        let i = match list_state.selected() {
                            Some(i) => {
                                if i == 0 {
                                    items.len().saturating_sub(1)
                                } else {
                                    i - 1
                                }
                            }
                            None => 0,
                        };
                        list_state.select(Some(i));
                    }
                    KeyCode::Down => {
                        let i = match list_state.selected() {
                            Some(i) => {
                                if i >= items.len().saturating_sub(1) {
                                    0
                                } else {
                                    i + 1
                                }
                            }
                            None => 0,
                        };
                        list_state.select(Some(i));
                    }
                    KeyCode::Enter => {
                        if let Some(i) = list_state.selected() {
                            return Ok(Some(items[i].command.clone()));
                        }
                    }
                    KeyCode::Esc => {
                        return Ok(None);
                    }
                    _ => {}
                }
            }
        }
    }
}
