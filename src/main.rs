use image::{GenericImageView, Pixel};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let img = image::open(&args[1]).unwrap();

    let width = img.width() as usize;
    let height = img.height() as usize;

    let charset = String::from(" .:-=+*#%@");
    let mut chars = vec![vec![String::new(); width]; height];

    for (column, row, pixel) in img.pixels() {
        let value = pixel.to_luma().0[0] as f32;
        let len = (charset.len() - 1) as f32;

        let start = ((value / 255.0) * len).round() as usize;
        let end = start + 1;

        let ansi = format!("\x1b[38;2;{};{};{}m", pixel.0[0], pixel.0[1], pixel.0[2]);
        chars[row as usize][column as usize] = format!("{}{}", ansi, &charset[start..end]);
    }

    for row in 0..height {
        if row % 2 != 0 { continue; }

        for column in 0..width {
            print!("{}", chars[row][column]);
        }

        println!();
    }

    println!("\x1b[0m");
}
