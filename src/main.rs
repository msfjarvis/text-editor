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
        if c == 'q' {
            break;
        }
        println!("{}", c);
    }
    Ok(())
}
