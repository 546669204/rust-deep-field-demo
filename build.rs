use glob::glob;
use std::{fs, path};

#[cfg(target_os = "windows")]
fn windows_copy_dll(){
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
}
#[cfg(target_os = "linux")]
fn linux_copy_so(){
  for entry in glob("target/release/build/torch-sys-*/out/libtorch/libtorch/lib/*.so")
    .expect("Failed to read glob pattern")
    .filter_map(Result::ok)
    .chain(
        glob("/usr/local/lib/libopencv*.so")
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
}


fn main() {
    #[cfg(target_os = "windows")]
    windows_copy_dll();

    #[cfg(target_os = "windows")]
    linux_copy_so();

    tauri_build::build()
}
