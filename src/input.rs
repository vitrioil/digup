use crossterm::event::{read, Event, KeyCode};
use std::io::{self, Write};

pub fn get_search_terms() -> io::Result<Vec<String>> {
    let mut search_terms = Vec::new();

    println!("Enter search terms (press Enter on empty line to finish):");

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut current_input = String::new();

        loop {
            if let Event::Key(event) = read()? {
                match event.code {
                    KeyCode::Char(c) => {
                        current_input.push(c);
                        print!("{}", c);
                        io::stdout().flush()?;
                    }
                    KeyCode::Backspace => {
                        if current_input.pop().is_some() {
                            print!("\x08 \x08");
                            io::stdout().flush()?;
                        }
                    }
                    KeyCode::Enter => {
                        println!();
                        break;
                    }
                    KeyCode::Esc => return Ok(search_terms),
                    _ => {}
                }
            }
        }

        if current_input.trim().is_empty() {
            break;
        } else {
            search_terms.push(current_input.trim().to_string());
        }
    }

    Ok(search_terms)
}
