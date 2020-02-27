use std::io::{stdout, Write};

use crossterm::{
    execute,
    style::{Colorize,PrintStyledContent, Styler},
};

fn main() {
    let f = 'f';
    let fruit = "fruit";

    execute!(
        stdout(),
        PrintStyledContent(f.green().bold())
    ).unwrap();

    execute!(
        stdout(),
        PrintStyledContent(fruit.red().underlined())
    ).unwrap();
}
