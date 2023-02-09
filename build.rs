use glob::glob;
use std::{env, fs, path};

fn main() {
  for entry in glob("target/release/build/torch-sys-*/out/libtorch/libtorch/lib/*.dll")
    .expect("Failed to read glob pattern")
    .filter_map(Result::ok)
    .chain(
        glob("C:/tools/opencv/build/x64/vc15/bin/*.dll")
            .expect("Failed to read glob pattern")
            .filter_map(Result::ok),
    )
  {
    fs::copy(
        &entry.as_os_str(),
        path::Path::new("./target/release/").join(&entry.file_name().unwrap()),
    )
    .unwrap();
    ()
  }
  
  tauri_build::build()
}
