use ::std;
use ::screen_writter::*;

pub fn str_repeat(s: String, n: usize) -> String {
    std::iter::repeat(s).take(n).collect::<Vec<_>>().join("")
}

pub fn make_label(s: &str) -> String {
    let prefix_size = 1;

    let prefix = str_repeat(String::from(HORIZONTAL_LINE), prefix_size);
    let rest_of_line = str_repeat(String::from(HORIZONTAL_LINE),
                                  BOX_WIDTH - s.len() - prefix_size - 2);

    String::from(TOP_LEFT) + &prefix + s + &rest_of_line + TOP_RIGHT
}

pub fn make_bottom() -> String {
    let mut line = String::new();
    for _ in 0..(BOX_WIDTH - 2) {
        line.push_str(HORIZONTAL_LINE)
    }
    String::from(BOTTOM_LEFT) + &line + BOTTOM_RIGHT
}
