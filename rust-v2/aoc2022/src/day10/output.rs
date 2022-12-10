use termion::color;

use crate::ocr::ocr;

pub struct Output {
    /// The buffer in which colorized output is collected.
    buffer_colorized: String,

    /// The buffer in which the plain output is collected.
    buffer_plain: String,
}

impl Output {
    pub fn new() -> Output {
        Output {
            buffer_colorized: String::new(),
            buffer_plain: String::new(),
        }
    }

    pub fn write(&mut self, pixel_active: bool) {
        if pixel_active {
            self.buffer_colorized.push_str(&colorize(color::LightGreen));
            self.buffer_colorized.push('#');
            self.buffer_plain.push('#');
        } else {
            self.buffer_colorized.push_str(&colorize(color::LightBlack));
            self.buffer_colorized.push('.');
            self.buffer_plain.push('.');
        }
    }

    pub fn next_line(&mut self) {
        self.buffer_colorized.push_str(&colorize(color::Reset));
        self.buffer_colorized.push('\n');
        self.buffer_plain.push('\n');
    }

    pub fn read(&self, use_color: bool) -> String {
        if use_color {
            self.buffer_colorized.clone()
        } else {
            self.buffer_plain.clone()
        }
    }

    pub fn ocr(&self) -> String {
        ocr(&self.buffer_plain)
    }
}

fn colorize<C: color::Color + Copy>(c: C) -> String {
    format!("{}{}", color::Fg(c), color::Bg(c))
}
