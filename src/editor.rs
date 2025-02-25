use std::io;

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        while !self.should_quit {
            self.process_keypress()?;
        }
        disable_raw_mode()?;
        Ok(())
    }
    fn process_keypress(&mut self) -> io::Result<()> {
        let press_key = event::read()?;
        if let Event::Key(k) = press_key {
            println!("{:?}", k);
            if k.code == KeyCode::Char('q') && k.modifiers.contains(KeyModifiers::CONTROL) {
                self.should_quit = true;
            }
        }
        Ok(())
    }
}
