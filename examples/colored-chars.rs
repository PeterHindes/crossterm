use std::io::{stdout, Write};

use crossterm::{
    execute,
    style::{Colorize,PrintStyledContent, Styler},
};

fn main() {
    let f = 'f';
    let fric = "fric";
    execute!(
        stdout(),
        PrintStyledContent(fric.underlined())
    ).unwrap();
    execute!(
        stdout(),
        PrintStyledContent(f.red().underlined().bold())
    ).unwrap();
}
