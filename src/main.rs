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
            println!("{:?} ({})\r", b, c);
        }
        if to_ctrl_byte(b'q') == b {
            break;
        }
    }
    disable_raw_mode().unwrap();
}

fn to_ctrl_byte(c: u8) -> u8 {
    c & 0b0001_1111
}
