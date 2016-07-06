extern crate rustbox;

use self::rustbox::{RustBox, Color};
use ::screen_writter::*;
use ::timer::{State, Mode};

impl ScreenWriter for RustBox {
    fn w(&self, x: usize, y: usize, text: &str) {
        self.print(x, y, rustbox::RB_BOLD, Color::White, Color::Black, text);
    }

    fn w_inv(&self, x: usize, y: usize, text: &str) {
        self.print(x, y, rustbox::RB_BOLD, Color::Black, Color::White, text);
    }

    fn w_boxed(&self, x: usize, y: usize, text: &str) {
        self.w(x, y, VERTICAL_LINE);
        self.w(x + 2, y, text);
        self.w(x + BOX_WIDTH - 1, y, VERTICAL_LINE);
    }

    fn draw(&self, state: &State) {
        self.clear();
        self.present();

        match state.mode {
            Mode::Menu => {
                self.w(0, 0, &::screen_writter::render::make_label(" Menu "));
                self.w_boxed(0, 1, "Press q to finish.");
                self.w(0, 2, &::screen_writter::render::make_bottom());
            }
            Mode::SetTime => {
                self.w(0, 0, "Set Time");
            }
            Mode::TimerStart => {
            }
            Mode::TimerStop => {
            }
            Mode::TimerPause => {
            }
            Mode::TimerFinish => {
            }
        }
        self.present();
    }
}
