// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ffi::c_void;

use opencv::{core::{Mat, MatTraitConst, Size_, Vector}, imgcodecs, imgproc::{self, COLOR_BGR2GRAY}};

use win_screenshot::prelude::*;
use base64::prelude::*;

struct Pos {
    x: f32,
    y: f32,
}

struct CardSize {
    width: f32,
    height: f32,
}

#[tauri::command]
fn process() -> [String;2] {
    let hwnd = find_window("Eternal Return").unwrap();
    let mut buf = capture_window(hwnd).unwrap();

    let src = unsafe {
        Mat::new_rows_cols_with_data_unsafe(
            buf.height as i32,
            buf.width as i32,
            opencv::core::CV_8UC4,
            buf.pixels.as_mut_ptr() as *mut c_void,
            opencv::core::Mat_AUTO_STEP,
        ).unwrap()
    };
    let src_size = src.size().unwrap();
    let src_size = Size_::new(src_size.width as f32, src_size.height as f32);
    
    let name_card_pos = Pos {
        x: 0.565,
        y: 0.863
    };

    let name_card_size = CardSize {
        width: 0.068,
        height: 0.017
    };

    let name_card_size_box = 0.08;

    let top_y = src_size.height * name_card_pos.y;
    let bottom_y = top_y + (src_size.height * name_card_size.height);
    let mut left_x = src_size.width * name_card_pos.x;
    let mut right_x = left_x + (src_size.width * name_card_size.width);
    let mut players:[Mat;3] = Default::default();
	
	let mut arr: [String; 2] = Default::default();
	let params: Vector<i32> = Vector::new();
    for idx in 0..players.len() {
        let cropped = Mat::roi(&src, opencv::core::Rect {
            x: left_x as i32,
            y: top_y as i32,
            width: right_x as i32 - left_x as i32,
            height: bottom_y as i32 - top_y as i32,
        }).unwrap();

        imgproc::cvt_color(&cropped, &mut players[idx], COLOR_BGR2GRAY, 0).unwrap();

        let mut output_buffer: Vector<u8> = Vector::new();
		imgcodecs::imencode(".jpg", &players[idx], &mut output_buffer, &params).unwrap();
		if idx > 0 {
			arr[idx-1] = BASE64_STANDARD.encode(output_buffer);
		}
		
        let next_pos = src_size.width * name_card_size.width + src_size.width * name_card_size_box;
        left_x += next_pos;
        right_x += next_pos;
    }

	arr
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
