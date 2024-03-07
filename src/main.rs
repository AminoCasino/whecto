use std::io::{self, stdout, Read};
use crossterm::terminal::enable_raw_mode;

fn main() {
    let _stdout = enable_raw_mode().unwrap();
    
    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;
        println!("{c}");
        if c == 'q' {
            break;
        }
    }
}
