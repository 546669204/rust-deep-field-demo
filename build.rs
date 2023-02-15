use glob::glob;
use std::{fs, path};

#[cfg(target_os = "windows")]
fn windows_copy_dll() {
    for entry in glob(&format!(
        "target/{}/build/torch-sys-*/out/libtorch/libtorch/lib/*.dll",
        std::env::var("PROFILE").unwrap()
    ))
    .expect("Failed to read glob pattern")
    .filter_map(Result::ok)
    .chain(
        glob("C:/tools/opencv/build/x64/vc15/bin/*.dll")
            .expect("Failed to read glob pattern")
            .filter_map(Result::ok),
    ) {
        fs::copy(
            &entry.as_os_str(),
            path::Path::new(&format!("./target/{}/", std::env::var("PROFILE").unwrap()))
                .join(&entry.file_name().unwrap()),
        )
        .unwrap();
        ()
    }
}
#[cfg(target_os = "linux")]
fn linux_copy_so() {
    for entry in glob(&format!(
        "target/{}/build/torch-sys-*/out/libtorch/libtorch/lib/*.so",
        std::env::var("PROFILE").unwrap()
    ))
    .expect("Failed to read glob pattern")
    .filter_map(Result::ok)
    .chain(
        glob("/usr/local/lib/libopencv*.so")
            .expect("Failed to read glob pattern")
            .filter_map(Result::ok),
    ) {
        fs::copy(
            &entry.as_os_str(),
            path::Path::new(&format!("./target/{}/", std::env::var("PROFILE").unwrap()))
                .join(&entry.file_name().unwrap()),
        )
        .unwrap();
        ()
    }
}


fn main() {
    #[cfg(target_os = "windows")]
    windows_copy_dll();

    #[cfg(target_os = "linux")]
    linux_copy_so();

    tauri_build::build()
}
