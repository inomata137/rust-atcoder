use fs_extra::{
    self,
    dir::{ self, CopyOptions }
};
use std::path::Path;

pub fn copy(to: &str) {
    let dir = format!("./{}", to);
    let dir = Path::new(dir.as_str());

    if dir.exists() {
        panic!("directory {} already exists!", to);
    }

    dir::create(dir, true)
        .expect(format!("failed to create a directory: {}", to).as_str());

    let ops = CopyOptions::new();
    fs_extra::copy_items(&vec![
        "./template/src",
        "./template/tests",
        "./template/Cargo.lock",
        "./template/Cargo.toml"
    ], format!("./{}", to), &ops).unwrap();
}
