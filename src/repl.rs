use rustyline::{error::ReadlineError, DefaultEditor};

const MULTI_LINE_START: &str = ":{";
const MULTI_LINE_END: &str = ":}";
const QUIT: &str = ":q";

/// REPL Input mode
#[derive(Clone, Copy, PartialEq)]
enum InputMode {
    /// Normal mode
    Normal,

    /// Multi-line mode
    /// Allows the user to input multiple lines, until closing.
    MultiLine,
}

impl InputMode {
    /// Get the prompt for the current mode
    fn prompt(&self) -> &str {
        match self {
            InputMode::Normal => "> ",
            InputMode::MultiLine => "... ",
        }
    }
}

/// REPL Contexts
pub struct ReplContext {
    rl: DefaultEditor,
    mode: InputMode,

    buffer: String,
    running: bool,
    interrupted: bool,
}

impl Default for ReplContext {
    fn default() -> Self {
        ReplContext {
            rl: DefaultEditor::new().unwrap(),
            mode: InputMode::Normal,
            buffer: String::new(),
            running: true,
            interrupted: false,
        }
    }
}

impl ReplContext {
    pub fn handle_input(&mut self, content: &String) {
        let trimmed = content.trim();
        let _ = self.rl.add_history_entry(trimmed);
        println!("Input: {:?}", trimmed);
    }

    /// Check the input is a special command
    /// If handled, return true
    pub fn handle_line(&mut self, line: &String) {
        let trimmed = line.trim();

        match trimmed {
            QUIT => {
                self.running = false;
            }
            MULTI_LINE_START => {
                self.mode = InputMode::MultiLine;
            }
            MULTI_LINE_END => {
                self.mode = InputMode::Normal;
                let content = std::mem::replace(&mut self.buffer, String::new());
                self.handle_input(&content);
            }
            _ => {
                if self.mode == InputMode::MultiLine {
                    self.buffer.push_str(line);
                    self.buffer.push('\n');
                } else {
                    self.handle_input(&line);
                }
            }
        }
    }

    pub fn start(&mut self) {
        self.running = true;
        while self.running {
            let line = self.rl.readline(self.mode.prompt());
            match line {
                Ok(line) => {
                    self.handle_line(&line);
                }
                Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                    if self.interrupted {
                        self.running = false;
                    } else {
                        self.interrupted = true;
                        eprintln!("Interrupted. To exit, press ^C or ^D once again.");
                    }
                    break;
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                    break;
                }
            }
        }
    }
}
