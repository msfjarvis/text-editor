use crate::Position;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{read, Event, KeyEvent},
    execute,
    style::{self, Color, ResetColor},
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
};
use std::io::{self, stdout, Error, Write};

pub struct Size {
    pub height: u16,
    pub width: u16,
}

pub struct Terminal {
    size: Size,
}

#[allow(unused_must_use)]
impl Drop for Terminal {
    fn drop(&mut self) {
        disable_raw_mode();
    }
}

impl Terminal {
    pub fn default() -> Result<Self, Error> {
        let (cols, rows) = size()?;
        enable_raw_mode()?;
        Ok(Self {
            size: Size {
                width: cols,
                height: rows.saturating_sub(2),
            },
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() -> Result<(), Error> {
        execute!(stdout(), Clear(ClearType::All))
    }

    pub fn clear_current_line() -> Result<(), Error> {
        execute!(stdout(), Clear(ClearType::CurrentLine))
    }

    pub fn reposition_cursor(position: &Position) -> Result<(), Error> {
        let Position { mut x, mut y } = position;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;
        execute!(stdout(), MoveTo(x, y))
    }

    pub fn flush() -> Result<(), Error> {
        io::stdout().flush()
    }

    pub fn read_key() -> Result<KeyEvent, io::Error> {
        loop {
            let event = read()?;
            if let Event::Key(event) = event {
                return Ok(event);
            }
        }
    }

    pub fn hide_cursor() -> Result<(), Error> {
        execute!(stdout(), Hide)
    }

    pub fn show_cursor() -> Result<(), Error> {
        execute!(stdout(), Show)
    }

    pub fn set_bg_color(color: Color) -> Result<(), Error> {
        execute!(stdout(), style::SetBackgroundColor(color))
    }

    pub fn reset_bg_color() -> Result<(), Error> {
        execute!(stdout(), ResetColor)
    }

    pub fn set_fg_color(color: Color) -> Result<(), Error> {
        execute!(stdout(), style::SetForegroundColor(color))
    }

    pub fn reset_fg_color() -> Result<(), Error> {
        execute!(stdout(), ResetColor)
    }
}
