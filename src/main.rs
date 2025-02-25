use std::io::{self, Read};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

fn main() {
    enable_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let c = b as char;
        if c == 'q' {
            break;
        }
        if c.is_control() {
            println!("{:?} \r", b);
        } else {
            println!()
        }
        println!("{:?} ({})\r", b, c);
    }
    disable_raw_mode().unwrap();
}
