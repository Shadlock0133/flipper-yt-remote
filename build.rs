use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use image::{Rgb, RgbImage};

// const TARGET: &str = "thumbv7em-none-eabihf";

fn main() {
    println!("cargo::rerun-if-changed=assets/");
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let mut icons = File::create(out_dir.join("icons.rs")).unwrap();
    write_mono_image(&out_dir, "assets/icon.png");
    for file in fs::read_dir("assets").unwrap() {
        let file = file.unwrap();
        if !file.file_type().unwrap().is_file() {
            continue;
        }
        let path = file.path();
        write_icon_const(
            &mut icons,
            &out_dir,
            &path.file_stem().unwrap().to_str().unwrap().to_uppercase(),
            path,
        );
    }

    let fw_path = Path::new("../../deps/flipperzero-firmware");

    let incs = [
        "",
        "furi",
        // "furi/core",
        "lib/ble_profile",
        "lib/cmsis_core",
        "lib/libusb_stm32/inc",
        "lib/mlib",
        "lib/stm32wb_cmsis/Include",
        "lib/stm32wb_copro/wpan",
        "lib/stm32wb_hal/Inc",
        "targets/f7/ble_glue",
        "targets/f7/inc",
        "targets/f7/furi_hal",
        "targets/furi_hal_include",
    ];

    let mut cc = cc::Build::new();
    cc.define("STM32WB55xx", None);
    for inc in &incs {
        cc.include(fw_path.join(inc));
    }
    cc.flag("-ffreestanding")
        .std("gnu2x")
        .file(fw_path.join("lib/ble_profile/extra_profiles/hid_profile.c"))
        .file(fw_path.join("lib/ble_profile/extra_services/hid_service.c"))
        .compile("bt_profile_hid");

    // let bindgen = bindgen::builder().use_core();
    // for inc in &incs {
    //     bindgen = bindgen.clang_arg("-I");
    //     bindgen = bindgen.clang_arg(fw_path.join(inc).to_string_lossy());
    // }
    // bindgen
    //     .clang_args(["-target", TARGET])
    //     .clang_args(["-std=gnu2x", "-ffreestanding"])
    //     .header(
    //         fw_path
    //             .join("lib/ble_profile/extra_profiles/hid_profile.h")
    //             .to_string_lossy(),
    //     )
    //     .header_contents("furi_ble/profile_interface.h", "")
    //     .header_contents("profile_interface.h", "")
    //     .allowlist_item("ble_profile_hid.*")
    //     .generate()
    //     .unwrap()
    //     .write_to_file(out_dir.join("bt_bindings.rs"))
    //     .unwrap();
}

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

#[track_caller]
fn write_mono_image(out_dir: &Path, src: impl AsRef<Path>) {
    let image = image::open(&src).unwrap().into_rgb8();
    fs::write(
        out_dir.join(src.as_ref().with_extension("icon").file_name().unwrap()),
        rgb_to_mono(&image),
    )
    .unwrap();
}

fn write_icon_const(
    output_file: &mut File,
    out_dir: &Path,
    name: &str,
    src: impl AsRef<Path>,
) {
    let image = image::open(&src).unwrap().into_rgb8();
    let (width, height) = image.dimensions();
    fs::write(
        out_dir.join(src.as_ref().with_extension("icon").file_name().unwrap()),
        rgb_to_mono(&image),
    )
    .unwrap();
    writeln!(
        output_file,
        "pub const {name}: sys::Icon = icon!({width}, {height}, \"{}\");",
        src.as_ref().file_stem().unwrap().to_str().unwrap()
    )
    .unwrap();
}
