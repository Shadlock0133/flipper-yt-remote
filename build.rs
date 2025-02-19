use std::{env, fs, path::PathBuf};

use image::Rgb;

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let image = image::open("src/icon.png").unwrap().into_rgb8();
    assert_eq!(image.dimensions(), (10, 10));
    let mut icon_data = [0; 1 + 10 * 2];
    for (y, row) in image.rows().enumerate() {
        for (x, pixel) in row.enumerate() {
            icon_data[y * 2 + (x / 8) + 1] |=
                ((pixel == &Rgb([0; 3])) as u8) << (x % 8);
        }
    }
    fs::write(out_dir.join("icon.icon"), icon_data).unwrap();
}
