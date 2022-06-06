use std::{env, fs};

use image::{GenericImage, GenericImageView, ImageFormat, RgbImage};
use palette::Palette;

mod palette;
mod rgb;

//such.palette: https://lospec.com/palette-list/aap-splendor128
//basic.palette: https://lospec.com/palette-list/dawnbringer-16

fn main() {
    let operation = env::args().nth(1).expect("expected `apply` or `preview`");
    match operation.as_str() {
        "preview" => gen_preview(),
        "apply" => apply(),
        _ => panic!("expected `apply` or `preview`"),
    }
}

fn apply() {
    let palette_path = env::args().nth(2).expect("give me a palette path or i mad");
    let load_path = env::args().nth(3).expect("where i load it fool");
    let save_path = env::args().nth(4).expect("where i save it fool");

    let palette_text = fs::read_to_string(palette_path)
        .expect("i couldn't load the palette file what the actual flip");

    let palette = Palette::from_palette_text(&palette_text);

    let source = image::open(load_path).expect("give me good image ples");
    let mut new = RgbImage::new(source.width(), source.height());

    for (x, y, image::Rgba([r, g, b, _a])) in source.pixels() {
        new.put_pixel(
            x,
            y,
            palette.closest_color(rgb::Rgb::from_u8(r, g, b)).to_u8(),
        );
    }

    new.save_with_format(save_path, ImageFormat::Png)
        .expect("failed to save");
}

fn gen_preview() {
    let palette_path = env::args().nth(2).expect("give me a palette path or i mad");
    let save_path = env::args().nth(3).expect("where i save it fool");

    let palette_text = fs::read_to_string(palette_path)
        .expect("i couldn't load the palette file what the actual flip");

    let palette = Palette::from_palette_text(&palette_text);

    // sqrt(256 * 256 * 256) = 16 * 16 * 16 = 4096
    let img = RgbImage::from_fn(4096, 4096, |x, y| {
        palette.closest_color(coord_to_color(x, y)).to_u8()
        // coord_to_color(x, y).to_u8()
    });

    img.save_with_format(save_path, ImageFormat::Png).unwrap();
}

fn coord_to_color(x: u32, y: u32) -> rgb::Rgb {
    rgb::Rgb::from_u8((x / 16) as u8, (y / 16) as u8, ((x + y) / 16) as u8)
}
