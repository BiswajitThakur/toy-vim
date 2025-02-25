use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};

#[derive(Default)]
pub struct Editor {}

impl Editor {
    pub fn run(&self) {
        enable_raw_mode().unwrap();
        loop {
            if let Event::Key(k) = event::read().unwrap() {
                println!("{:?}", k);
                if k.code == KeyCode::Char('q') && k.modifiers.contains(KeyModifiers::CONTROL) {
                    break;
                }
            }
        }
        disable_raw_mode().unwrap();
    }
}
