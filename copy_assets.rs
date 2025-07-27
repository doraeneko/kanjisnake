use fs_extra::dir::{CopyOptions, copy};
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir)
        .ancestors()
        .nth(3)
        .unwrap()
        .to_path_buf();

    let dest_assets = target_dir.join("assets");

    if dest_assets.exists() {
        let _ = fs::remove_dir_all(&dest_assets);
    }

    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;

    copy("assets", &target_dir, &options).expect("Fehler beim Kopieren der Assets");
}
