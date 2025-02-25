use std::io::{self, Stdout};

use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyEvent},
    execute,
    terminal::{Clear, ClearType},
};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal<W: io::Write> {
    size: Size,
    stdout: W,
}

impl Terminal<Stdout> {
    pub fn defauilt() -> io::Result<Self> {
        let (width, height) = crossterm::terminal::size()?;
        Ok(Self {
            size: Size { width, height },
            stdout: io::stdout(),
        })
    }
}

impl<W: io::Write> Terminal<W> {
    pub fn size(&self) -> &Size {
        &self.size
    }
    pub fn clear_screen(&mut self) -> io::Result<()> {
        execute!(self.stdout, Clear(ClearType::FromCursorUp))
    }
    pub fn cursor_position(&mut self, x: u16, y: u16) -> io::Result<()> {
        let x = x.saturating_add(1);
        let y = y.saturating_add(1);
        execute!(self.stdout, MoveTo(x, y))
    }
    pub fn read_key() -> io::Result<KeyEvent> {
        loop {
            if let Event::Key(k) = event::read()? {
                return Ok(k);
            }
        }
    }
}
