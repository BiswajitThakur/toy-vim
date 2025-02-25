use std::io;

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};

#[derive(Default)]
pub struct Editor {}

impl Editor {
    pub fn run(&self) -> io::Result<()> {
        enable_raw_mode()?;
        loop {
            self.process_keypress()?;
        }
    }
    fn process_keypress(&self) -> io::Result<()> {
        let press_key = event::read()?;
        if let Event::Key(k) = press_key {
            println!("{:?}", k);
            if k.code == KeyCode::Char('q') && k.modifiers.contains(KeyModifiers::CONTROL) {
                disable_raw_mode()?;
                std::process::exit(0);
            }
        }
        Ok(())
    }
}
