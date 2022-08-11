use crate::{Document, Row, Terminal};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Color;
use std::result::Result;
use std::time::{Duration, Instant};
use std::{
    env,
    io::{self, Error},
};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const STATUS_BG_COLOR: Color = Color::Rgb {
    r: 26,
    g: 28,
    b: 30,
};
const STATUS_FG_COLOR: Color = Color::Rgb {
    r: 225,
    g: 226,
    b: 229,
};

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    document: Document,
    cursor_position: Position,
    offset: Position,
    status_message: StatusMessage,
}

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

struct StatusMessage {
    text: String,
    time: Instant,
}

impl StatusMessage {
    fn from(message: String) -> Self {
        Self {
            text: message,
            time: Instant::now(),
        }
    }
}

impl Default for Editor {
    fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut initial_status = String::from("HELP: Ctrl-Q = quit, Ctrl-S = save");
        let document = if args.len() > 1 {
            let file_name = &args[1];
            let doc = Document::open(file_name);
            if let Ok(doc) = doc {
                doc
            } else {
                initial_status = format!("ERROR: Failed to open file: {}", file_name);
                Document::default()
            }
        } else {
            Document::default()
        };
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            document,
            cursor_position: Position::default(),
            offset: Position::default(),
            status_message: StatusMessage::from(initial_status),
        }
    }
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            };
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            };
        }
    }

    fn process_keypress(&mut self) -> Result<(), io::Error> {
        let pressed_key = Terminal::read_key()?;
        let KeyEvent {
            code, modifiers, ..
        } = pressed_key;
        match (code, modifiers) {
            (KeyCode::Char('q'), KeyModifiers::CONTROL) => self.should_quit = true,
            (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                if self.document.save().is_ok() {
                    self.status_message =
                        StatusMessage::from("File saved successfully.".to_string());
                } else {
                    self.status_message = StatusMessage::from("Error writing file!".to_string());
                }
            }
            (KeyCode::Char(c), KeyModifiers::NONE) => {
                self.document.insert(&self.cursor_position, c);
                self.move_cursor(KeyCode::Right);
            }
            (KeyCode::Delete, KeyModifiers::NONE) => self.document.delete(&self.cursor_position),
            (KeyCode::Backspace, KeyModifiers::NONE) => {
                if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
                    self.move_cursor(KeyCode::Left);
                    self.document.delete(&self.cursor_position);
                }
            }
            (
                KeyCode::Left
                | KeyCode::Right
                | KeyCode::Up
                | KeyCode::Down
                | KeyCode::PageUp
                | KeyCode::PageDown
                | KeyCode::Home
                | KeyCode::End,
                KeyModifiers::NONE,
            ) => self.move_cursor(code),
            _ => (),
        };
        self.scroll();
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), io::Error> {
        Terminal::hide_cursor()?;
        Terminal::reposition_cursor(&Position::default())?;
        if self.should_quit {
            Terminal::clear_screen()?;
            println!("Goodbye.\r");
        } else {
            self.draw_rows()?;
            self.draw_status_bar()?;
            self.draw_message_bar()?;
            Terminal::reposition_cursor(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            })?;
        }
        Terminal::show_cursor()?;
        Terminal::flush()
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("{} -- version {}\r", NAME, VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    fn draw_row(&self, row: &Row) {
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    fn draw_rows(&self) -> Result<(), Error> {
        let height = self.terminal.size().height;
        for terminal_row in 0..height {
            Terminal::clear_current_line()?;
            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
        Ok(())
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let mut offset = &mut self.offset;
        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }
        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    fn move_cursor(&mut self, key: KeyCode) {
        let terminal_height = self.terminal.size().height as usize;
        let Position { mut y, mut x } = self.cursor_position;
        let height = self.document.len();
        let mut width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        match key {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            KeyCode::Left => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;
                    if let Some(row) = self.document.row(y) {
                        x = row.len();
                    } else {
                        x = 0;
                    }
                }
            }
            KeyCode::Right => {
                if x < width {
                    x += 1;
                } else if y < height {
                    y += 1;
                    x = 0;
                }
            }
            KeyCode::PageUp => {
                y = if y > terminal_height {
                    y - terminal_height
                } else {
                    0
                }
            }
            KeyCode::PageDown => {
                y = if y.saturating_add(terminal_height) < height {
                    y + terminal_height
                } else {
                    height
                }
            }
            KeyCode::Home => x = 0,
            KeyCode::End => x = width,
            _ => (),
        }
        width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        if x > width {
            x = width;
        };
        self.cursor_position = Position { x, y }
    }

    fn draw_status_bar(&self) -> Result<(), Error> {
        let mut status;
        let width = self.terminal.size().width as usize;
        let mut file_name = "[No name]".to_string();
        if let Some(filename) = &self.document.file_name {
            file_name = filename.to_string();
            file_name.truncate(20);
        }
        let position = format!("({},{})", self.cursor_position.x, self.cursor_position.y);
        status = format!("{} {}", file_name, position);
        if width > status.len() {
            status.push_str(&" ".repeat(width - status.len()));
        }
        status.truncate(width);
        Terminal::set_bg_color(STATUS_BG_COLOR)?;
        Terminal::set_fg_color(STATUS_FG_COLOR)?;
        println!("{}\r", status);
        Terminal::reset_fg_color()?;
        Terminal::reset_bg_color()
    }

    fn draw_message_bar(&self) -> Result<(), Error> {
        Terminal::clear_current_line()?;
        let message = &self.status_message;
        if Instant::now() - message.time < Duration::new(5, 0) {
            let mut text = message.text.clone();
            text.truncate(self.terminal.size().width as usize);
            print!("{}", text);
        }
        Ok(())
    }
}

#[allow(unused_must_use)] // The program is already panicking, we don't need to interrupt it.
fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
