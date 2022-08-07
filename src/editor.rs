use anyhow::Result;
use std::io::{self, stdout};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();
        loop {
            if let Err(error) = self.process_keypress() {
                panic!("{}", error);
            };
            if self.should_quit {
                break;
            }
        }
    }

    fn process_keypress(&mut self) -> Result<()> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        };
        Ok(())
    }
}

fn read_key() -> std::result::Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}
