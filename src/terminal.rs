use crate::editor::Position;
use std::io::{self, stdout, Error, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Size {
    pub height: u16,
    pub width: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<io::Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    pub fn reposition_cursor(position: &Position) {
        let Position { mut x, mut y } = position;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;
        print!("{}", termion::cursor::Goto(x, y));
    }

    pub fn flush() -> Result<(), Error> {
        io::stdout().flush()
    }

    pub fn read_key() -> Result<Key, io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn hide_cursor() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn show_cursor() {
        print!("{}", termion::cursor::Show);
    }
}
