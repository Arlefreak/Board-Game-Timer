use ::timer::State;

pub static TOP_RIGHT: &'static str = "┐";
pub static VERTICAL_LINE: &'static str = "│";
pub static HORIZONTAL_LINE: &'static str = "─";
pub static TOP_LEFT: &'static str = "┌";
pub static BOTTOM_RIGHT: &'static str = "┘";
pub static BOTTOM_LEFT: &'static str = "└";

pub static BOX_WIDTH: usize = 40;

pub trait ScreenWriter {
    fn w(&self, x: usize, y: usize, text: &str);
    fn w_inv(&self, x: usize, y: usize, text: &str);
    fn w_boxed(&self, x: usize, y: usize, text: &str);
    fn draw(&self, state: &State);
}

pub mod draw;
pub mod render;
