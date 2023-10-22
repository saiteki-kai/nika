#![allow(unused)]
extern crate termsize;

use termsize::Size;

pub fn terminal_size() -> (u16, u16) {
    let size = termsize::get().unwrap_or(Size { rows: 80, cols: 24 });

    (size.rows, size.cols)
}
