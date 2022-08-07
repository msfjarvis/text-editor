use crate::Terminal;
use std::io;
use std::result::Result;
use termion::event::Key;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
}

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position::default(),
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
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        };
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), io::Error> {
        Terminal::hide_cursor();
        Terminal::reposition_cursor(&Position::default());
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::reposition_cursor(&self.cursor_position);
        }
        Terminal::show_cursor();
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

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..height - 1 {
            Terminal::clear_current_line();
            if row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }
}

fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
