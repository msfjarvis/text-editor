use anyhow::{bail, Result};
use std::io::{self, stdout};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

#[derive(Default)]
pub struct Editor {}

impl Editor {
    pub fn run(&self) -> Result<()> {
        let _stdout = stdout().into_raw_mode()?;
        for key in io::stdin().keys() {
            match key {
                Ok(key) => match key {
                    Key::Char(c) => {
                        if c.is_control() {
                            println!("{:?} \r", c as u8);
                        } else {
                            println!("{:?} ({}) \r", c as u8, c);
                        }
                    }
                    Key::Ctrl('q') => break,
                    _ => println!("{:?} \r", key),
                },
                Err(e) => bail!(e),
            }
        }
        Ok(())
    }
}
