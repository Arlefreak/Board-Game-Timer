extern crate board_game_timer;
extern crate rustbox;

use board_game_timer::screen_writter::ScreenWriter;
use board_game_timer::timer::{ State, Mode, Status};

// use rustbox::{RustBox, Color, Key};
use rustbox::{RustBox, Key};

fn main() {
    let state = State {
        mode: Mode::Menu,
        status: Status::Clean,
        title: String::from("Menu")
    };

    {
        let rustbox = RustBox::init(Default::default()).unwrap();
        rustbox.draw(&state);

        loop {
            if let rustbox::Event::KeyEvent(mkey) = rustbox.poll_event(false)
                .ok()
                    .expect("poll failed") {
                        match mkey {
                            Key::Char('q') => { break; }
                            _ => { }
                        }
                    }
        }
    }
}
