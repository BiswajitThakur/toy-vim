use std::io::{self, Stdout};

use crossterm::{
    event::{KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::terminal::Terminal;

const VERSION: &str = env!("CARGO_PKG_VERSION");
#[derive(Default)]
struct Position {
    x: usize,
    y: usize,
}
pub struct Editor<W: io::Write> {
    should_quit: bool,
    terminal: Terminal<W>,
    cursor_position: Position,
}

impl Default for Editor<Stdout> {
    fn default() -> Self {
        let terminal = Terminal::defauilt().unwrap();
        Self {
            should_quit: false,
            terminal,
            cursor_position: Position::default(),
        }
    }
}

impl<W: io::Write> Editor<W> {
    pub fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            self.process_keypress()?;
        }
        disable_raw_mode()?;
        Ok(())
    }
    fn draw_rows(&mut self) -> io::Result<()> {
        let height = self.terminal.size().height;
        for row in 0..self.terminal.size().height - 1 {
            self.terminal.clear_current_line()?;
            if row == height / 3 {
                self.draw_welcome_msg()?;
            } else {
                writeln!(self.terminal.stdout(), "~\r")?;
            }
        }
        Ok(())
    }
    fn refresh_screen(&mut self) -> io::Result<()> {
        self.terminal.cursor_hide()?;
        self.terminal.clear_screen()?;
        self.terminal.cursor_position(0, 0)?;
        if self.should_quit {
            self.terminal.clear_screen()?;
            println!("Good bye.\r");
        } else {
            self.draw_rows()?;
            self.terminal.cursor_position(1, 0)?;
        }
        self.terminal.cursor_show()?;
        Ok(())
    }
    fn process_keypress(&mut self) -> io::Result<()> {
        let key = Terminal::<W>::read_key()?;
        if key.code == KeyCode::Char('q') && key.modifiers.contains(KeyModifiers::CONTROL) {
            self.should_quit = true;
        }
        Ok(())
    }
    fn draw_welcome_msg(&mut self) -> io::Result<()> {
        let mut welcome_message = format!("Hecto editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        writeln!(self.terminal.stdout(), "{}\r", welcome_message)
    }
}
