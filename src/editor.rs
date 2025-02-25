use std::io::{self, Stdout};

use crossterm::{
    event::{KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::terminal::Terminal;

pub struct Editor<W: io::Write> {
    should_quit: bool,
    terminal: Terminal<W>,
}

impl Default for Editor<Stdout> {
    fn default() -> Self {
        let terminal = Terminal::defauilt().unwrap();
        Self {
            should_quit: false,
            terminal,
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
    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height {
            println!("~\r");
        }
    }
    fn refresh_screen(&mut self) -> io::Result<()> {
        self.terminal.clear_screen()?;
        self.terminal.cursor_position(0, 0)?;
        if self.should_quit {
            println!("Good bye.\r");
        } else {
            self.draw_rows();
            self.terminal.cursor_position(0, 0)?;
        }
        Ok(())
    }
    fn process_keypress(&mut self) -> io::Result<()> {
        let key = Terminal::<W>::read_key()?;
        if key.code == KeyCode::Char('q') && key.modifiers.contains(KeyModifiers::CONTROL) {
            self.should_quit = true;
        }
        Ok(())
    }
}
