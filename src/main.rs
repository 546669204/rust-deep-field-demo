#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    fs,
    io::{self},
    ops::{Div, Mul},
    path::{Path, PathBuf},
};

use base64::Engine;
use lazy_static::lazy_static;
use opencv::{core::*, highgui, imgcodecs, imgproc, types::*};
use opencv::{imgcodecs::imencode, prelude::*};

lazy_static! {
    static ref DEEP_FIELD_MODULE: tch::CModule =
        tch::CModule::load("./modules/model-small-traced.pt").unwrap();
}

const BASE64: base64::engine::general_purpose::GeneralPurpose =
    base64::engine::general_purpose::STANDARD;

    
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, ppppp])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn constrain_to_multiple_of(x: f64, min_val: f64, max_val: f64) -> f64 {
    let __multiple_of = 32.;
    let mut y = math::round::ceil((x / __multiple_of).into(), 0) * __multiple_of;
    if max_val == 0. && y > max_val {
        y = math::round::floor((x / __multiple_of).into(), 0) * __multiple_of;
    }

    if y < min_val {
        y = math::round::ceil((x / __multiple_of).into(), 0) * __multiple_of;
    }

    return y;
}
fn folders(dir: &Path) -> Result<Vec<PathBuf>, io::Error> {
    Ok(fs::read_dir(dir)?
        .filter_map(|e| {
            e.ok().and_then(|d| {
                let p = d.path();
                if p.is_dir() {
                    None
                } else {
                    Some(p)
                }
            })
        })
        .collect())
}

fn forward_deep_module(img: &Mat) -> Result<Mat, Box<dyn std::error::Error>> {
    let (width, height) = (img.cols(), img.rows());
    let mut scale_height = 256. / height as f64;
    let mut scale_width = 256. / width as f64;
    if scale_width < scale_height {
        scale_height = scale_width;
    } else {
        scale_width = scale_height;
    }
    let new_height = constrain_to_multiple_of((scale_height * height as f64).into(), 0., 256.);
    let new_width = constrain_to_multiple_of((scale_width * width as f64).into(), 0., 256.);
    log::debug!("resize");
    let mut dst = Mat::default();
    imgproc::resize(
        &img,
        &mut dst,
        Size::new(new_width as i32, new_height as i32),
        0.,
        0.,
        imgproc::INTER_LINEAR,
    )?;
    log::debug!("load image buffer");
    let params = Vector::<i32>::default();
    let mut img_bytes = Vector::<u8>::default();
    imgcodecs::imencode(".jpg", &dst, &mut img_bytes, &params).unwrap();
    let mut image = tch::vision::imagenet::load_image_from_memory(img_bytes.as_slice())?;
    image = image.unsqueeze(0);
    log::debug!("forward_is");
    let outputs = DEEP_FIELD_MODULE
        .forward_is(&[tch::IValue::Tensor(image)])
        .expect("msg");
    match outputs {
        tch::IValue::Tensor(t) => {
            let max = t.max();
            let mut img = t;

            img = img.unsqueeze(1);
            img = img.upsample_bicubic2d(
                &[height.into(), width.into()],
                false,
                height as f64 / new_height,
                width as f64 / new_width,
            );

            img = img.mul(255).div(max);
            img = img.clamp(0, 255);
            img = img.squeeze();
            img = img.unsqueeze(0);
            img = img.permute(&[1, 2, 0]);
            img = img.f_contiguous()?.to_device(tch::Device::Cpu);
            let img = unsafe {
                Mat::new_rows_cols_with_data(height, width, CV_32FC1, img.data_ptr(), 0)?
            };
            Ok(img.clone())
        }
        _ => Err("not result".into()),
    }
}

fn process_image_by_base64(
    file_base64: &str,
    split_limit: Vec<Vec<u8>>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file_bytes = BASE64.decode(file_base64)?;

    let img = imgcodecs::imdecode(&VectorOfu8::from_iter(file_bytes), imgcodecs::IMREAD_COLOR)?;
    let oimg = img.clone();
    let result_mat = forward_deep_module(&img)?;
    split_limit
        .iter()
        .map(|sl| -> Result<String, Box<dyn std::error::Error>> {
            let mut dst = Mat::default();
            log::debug!("in_range! {:?}", sl);
            let mut img = Mat::default();
            result_mat.convert_to(&mut img, CV_8UC1, 1., 0.)?;
            in_range(
                &img,
                &Scalar::new(sl[0].into(), sl[0].into(), sl[0].into(), sl[0].into()),
                &Scalar::new(sl[1].into(), sl[1].into(), sl[1].into(), sl[1].into()),
                &mut dst,
            )?;
            let mut dst2 = Mat::default();
            log::debug!("blur");
            imgproc::blur(
                &dst,
                &mut dst2,
                Size::new(10, 10),
                Point::default(),
                BORDER_DEFAULT,
            )?;
            dst = dst2;
            let mut dst2 = Mat::default();

            let mut png_img = Mat::default();
            imgproc::cvt_color(&oimg, &mut png_img, imgproc::COLOR_RGB2RGBA, 0)?;
            log::debug!("split!");
            let mut rgba = VectorOfMat::default();
            split(&png_img, &mut rgba)?;
            rgba.set(3, dst)?;
            merge(&rgba, &mut dst2)?;
            let params = Vector::<i32>::default();
            let mut img_bytes = Vector::<u8>::default();
            imencode(".png", &dst2, &mut img_bytes, &params)?;
            Ok(BASE64.encode(img_bytes.as_slice()))
        })
        .collect::<Result<Vec<String>, Box<dyn std::error::Error>>>()
}

#[tauri::command]
fn ppppp(file_bytes: &str, split_limit: Vec<Vec<u8>>) -> Result<Vec<String>, String> {
    process_image_by_base64(file_bytes, split_limit).map_err(|err| err.to_string())
}
