use anyhow::Result;
use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn main() -> Result<()> {
    let _stdout = stdout().into_raw_mode()?;
    for b in io::stdin().bytes() {
        let b = b?;
        let c = b as char;
        if c.is_control() {
            println!("{:?} \r", b);
        } else {
            println!("{:?} ({}) \r", b, c);
        }
        if b == to_control_byte('q') {
            break;
        }
        println!("{}", c);
    }
    Ok(())
}

/// Given a character, computes the byte sequence for Ctrl + <char>.
/// This lets callers compare the returned byte against [io::stdin]'s
/// output to react to control sequences.
fn to_control_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}
