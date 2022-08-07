use anyhow::{bail, Result};
use std::io::{self, stdout};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

#[derive(Default)]
pub struct Editor {}

impl Editor {
    pub fn run(&self) -> Result<()> {
        let _stdout = stdout().into_raw_mode()?;
        loop {
            if let Err(error) = self.process_keypress() {
                bail!(error);
            }
        }
    }

    fn process_keypress(&self) -> Result<()> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('q') => bail!("Program end"),
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
