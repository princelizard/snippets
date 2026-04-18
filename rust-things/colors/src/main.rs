use std::time::Duration;
use crossterm::{event::{Event, poll, read}, style::Stylize, terminal::enable_raw_mode};
use owo_colors::OwoColorize;
fn main() {
    enable_raw_mode().unwrap();
    let mut i: u8 = 0;

    loop {
        if poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(_) = read().unwrap() {
                break;
            }
        }

        let out_string: String = "Hellorld!".to_string();
        match i {
            0 => {println!("{}\r", out_string.red()); i+=1},
            1 => {println!("{}\r", out_string.yellow()); i+=1},
            2 => {println!("{}\r", out_string.green()); i+=1},
            3 => {println!("{}\r", out_string.cyan()); i+=1},
            4 => {println!("{}\r", out_string.blue()); i+=1},
            5 => {println!("{}\r", out_string.fg_rgb::<127, 0, 255>()); i+=1},
            _ => i = 0,
        }

    }
}
