mod history;
mod input;
mod filter;
mod ui;

use crate::filter::filter_commands;
use crate::history::read_history;
use crate::input::get_search_terms;
use crate::ui::run_app;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self};
use tui::backend::CrosstermBackend;
use tui::Terminal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read history
    let history = read_history()?;

    // Get search terms
    let search_terms = get_search_terms()?;

    // Filter commands
    let filtered_entries = filter_commands(&history, &search_terms);

    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the app and capture the selected command
    let selected_command = run_app(&mut terminal, &filtered_entries)?;

    // Before restoring the terminal, output the selected command
    if let Some(cmd) = selected_command {
        // Output the selected command to stdout
        println!("{}", cmd);
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
