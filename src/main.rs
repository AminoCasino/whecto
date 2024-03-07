use std::io::{self, stdout, Read};
use crossterm::terminal::enable_raw_mode;

fn main() {
    let _stdout = enable_raw_mode().unwrap();
    
    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let c = b as char;
        // ! NOTE: crossterm's raw mode is different than termion's
        // ! raw mode. With termion's raw mode, at this point in
        // ! the tutorial, each key press would be printed - even
        // ! F2, ESC, Tab, Home, End, Insert, Delete, etc.
        // ! With crossterm such is not the case.
        if c.is_control() {
            println!("{:?}\r", b);
        } else {
            println!("{:?} ({})", b, c);
        }
        if c == 'q' {
            break;
        }
    }
}
