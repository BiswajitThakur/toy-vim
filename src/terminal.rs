use std::io::{self, Stdout};

use crossterm::{
    cursor::{self, MoveTo},
    event::{self, Event, KeyEvent},
    execute,
    terminal::{Clear, ClearType},
};
#[derive(Default)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl AsRef<Position> for Position {
    fn as_ref(&self) -> &Self {
        self
    }
}

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
    pub fn stdout(&mut self) -> &mut W {
        &mut self.stdout
    }
    pub fn clear_screen(&mut self) -> io::Result<()> {
        execute!(self.stdout, Clear(ClearType::All))
    }
    pub fn clear_current_line(&mut self) -> io::Result<()> {
        execute!(self.stdout, Clear(ClearType::CurrentLine))
    }
    pub fn cursor_position<T: AsRef<Position>>(&mut self, pos: T) -> io::Result<()> {
        let &Position { x, y } = pos.as_ref();
        let x = x.saturating_add(1);
        let y = y.saturating_add(1);
        execute!(self.stdout, MoveTo(x, y))
    }
    pub fn cursor_hide(&mut self) -> io::Result<()> {
        execute!(self.stdout, cursor::Hide)
    }
    pub fn read_key() -> io::Result<KeyEvent> {
        loop {
            if let Event::Key(k) = event::read()? {
                return Ok(k);
            }
        }
    }
    pub fn cursor_show(&mut self) -> io::Result<()> {
        execute!(self.stdout, cursor::Show)
    }
}
