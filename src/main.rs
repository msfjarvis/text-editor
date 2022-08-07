use anyhow::Result;
use std::io::{self, Read};

fn main() -> Result<()> {
    for b in io::stdin().bytes() {
        let c = b? as char;
        if c == 'q' {
            break;
        }
        println!("{}", c);
    }
    Ok(())
}
