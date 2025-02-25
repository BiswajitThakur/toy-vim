use std::io;

use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
}

impl Editor {
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
        for _ in 0..23 {
            println!("~\r");
        }
    }
    fn refresh_screen(&self) -> io::Result<()> {
        execute!(io::stdout(), Clear(ClearType::FromCursorUp), MoveTo(0, 0))?;
        if self.should_quit {
            println!("Good bye.\r");
        } else {
            self.draw_rows();
            execute!(io::stdout(), MoveTo(1, 0))?;
        }
        Ok(())
    }
    fn process_keypress(&mut self) -> io::Result<()> {
        let press_key = event::read()?;
        if let Event::Key(k) = press_key {
            if k.code == KeyCode::Char('q') && k.modifiers.contains(KeyModifiers::CONTROL) {
                self.should_quit = true;
            }
        }
        Ok(())
    }
}
