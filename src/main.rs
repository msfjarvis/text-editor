use anyhow::{bail, Result};
use std::io::{self, stdout};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

fn main() -> Result<()> {
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
