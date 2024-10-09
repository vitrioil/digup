use chrono::{NaiveDateTime, TimeZone, Utc};
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone)] // Added this line
pub struct HistoryEntry {
    pub timestamp: Option<NaiveDateTime>,
    pub command: String,
}

pub fn read_history() -> io::Result<Vec<HistoryEntry>> {
    let home_dir = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let history_path = format!("{}/.zsh_history", home_dir);
    let path = Path::new(&history_path);
    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();
    let mut history = Vec::new();

    let re = Regex::new(r"^: (\d+):\d+;(.*)$").unwrap();

    for line in lines {
        if let Ok(cmd_line) = line {
            if let Some(caps) = re.captures(&cmd_line) {
                // Parse timestamp and command
                let timestamp = caps.get(1).and_then(|m| m.as_str().parse::<i64>().ok());
                let command = caps.get(2).map_or("", |m| m.as_str()).to_string();

                // Handle multiline commands
                let command = command.replace("\\\n", " ").replace("\n", " ");

                // Convert timestamp to NaiveDateTime
                let datetime = timestamp.map(|ts| NaiveDateTime::from_timestamp(ts, 0));

                history.push(HistoryEntry {
                    timestamp: datetime,
                    command,
                });
            } else {
                // If the line doesn't match the regex, treat it as a command without timestamp
                history.push(HistoryEntry {
                    timestamp: None,
                    command: cmd_line,
                });
            }
        }
    }

    // Reverse the history to show the most recent entries first
    history.reverse();

    Ok(history)
}
