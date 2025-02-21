use std::{env, fs, path::PathBuf};

use image::{Rgb, RgbImage};

fn rgb_to_mono(image: &RgbImage) -> Vec<u8> {
    let (width, height) = image.dimensions();
    let row_size = width.div_ceil(8);

    let mut res = vec![0; (row_size * height) as usize + 1];

    for (y, row) in image.rows().enumerate() {
        for (x, pixel) in row.enumerate() {
            res[y * row_size as usize + (x / 8) + 1] |=
                ((pixel == &Rgb([0; 3])) as u8) << (x % 8);
        }
    }

    res
}

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let image = image::open("src/icon.png").unwrap().into_rgb8();
    fs::write(out_dir.join("icon.icon"), rgb_to_mono(&image)).unwrap();
}
