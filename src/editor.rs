use crate::terminal::Terminal;
use std::io;
use std::result::Result;
use termion::event::Key;

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
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
        if let Key::Ctrl('q') = pressed_key {
            self.should_quit = true;
        };
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), io::Error> {
        Terminal::clear_screen();
        Terminal::reposition_cursor(0, 0);
        if self.should_quit {
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::reposition_cursor(0, 0);
        }
        Terminal::flush()
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height - 1 {
            println!("~\r");
        }
    }
}

fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
