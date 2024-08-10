use std::{env, thread, time::Duration};

use chrono::Local;
use rand::Rng as Random;
use std::mem;
use win_ocr;
use winapi::um::winuser::{INPUT, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP};

// OCRを行い、シリアル通信でArduinoへ通知する

fn main() {
    // コマンドライン引数を取得
    let args: Vec<String> = env::args().collect();

    // 引数が2つない場合はエラーを表示
    if args.len() < 3 {
        eprintln!("Usage: shiny_mas <width px> <height px> <scale> <index> <COM Name>");
        return;
    }
    let raw_width_px: i32 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Width px is not a number.");
            return;
        }
    };
    let raw_height_px: i32 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Height px is not a number.");
            return;
        }
    };

    let scale: f32 = match args[3].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Scale is not a float.Fall back to 1.0");
            1.0
        }
    };

    let monitor_index: usize = match args[4].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Monitor index is not a number.");
            0
        }
    };

    let com_name: String = match args[5].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("COM port index is not a String.");
            return;
        }
    };

    println!(
        "Width: {}, Height: {}, Scale: {}, Monitor Index: {} Com Port: {}",
        raw_width_px, raw_height_px, scale, monitor_index, &com_name
    );

    let resolution_width_px: i32 = ((raw_width_px as f32) / scale) as i32;
    let resolution_height_px: i32 = ((raw_height_px as f32) / scale) as i32;

    println!(
        "Resolution: {}x{}",
        resolution_width_px, resolution_height_px
    );
    let screen_info = ScreenInfo::new(resolution_width_px, resolution_height_px);

    let wait_duration = Duration::new(1, 0);

    // シリアル通信の設定
    // COMポートへ通信
    println!("Send data to {}", &com_name);
    let mut port = serialport::new(&com_name, 9600)
        .open()
        .expect("failed to create port");

    // delay 5sec
    thread::sleep(Duration::from_secs(5));

    loop {
        if is_playing(monitor_index, &screen_info) {
            // 再生中の場合、待機する
            println!("再生中のため、{}秒待機", wait_duration.as_secs());
        } else {
            // 再生中でない場合、ランダム再生する
            println!("楽曲選択画面にいるため、Arduinoへ通知");
            // データを送信
            let data_to_send = b"1";
            port.write_all(data_to_send)
                .expect("Failed to write data to port");
            println!("Data sent to port");
        }

        thread::sleep(wait_duration);
    }
}

fn is_playing(monitor_index: usize, screen_info: &ScreenInfo) -> bool {
    // 画面のショットを取得
    let screens = screenshots::Screen::all().expect("Failed to get screens");

    println!("Screen count: {}", screens.len());
    let primary_screen = screens[monitor_index];

    let capture = primary_screen
        .capture_area(
            0,
            0,
            ((screen_info.width as f32) * 0.2) as u32,
            ((screen_info.height as f32) * 0.2) as u32,
        )
        .unwrap();

    // 画像をyyyyMMddHHmmssfffを付与して保存
    let path = format!("./capture_{}.png", Local::now().format("%Y%m%d%H%M%S%f"));

    capture.save(&path).expect("Failed to save capture image.");

    // ショットをOCRにかける
    let ocr_result = win_ocr::ocr_with_lang(&path, "ja").expect("Failed to OCR");

    // 画像を削除
    std::fs::remove_file(path).expect("Failed to remove capture image.");

    // 結果を判定
    let trimmed_result = ocr_result.replace(char::is_whitespace, "");
    println!("OCR Result: {}", trimmed_result);

    let is_waiting_in_song_list = (trimmed_result.contains("MV")
        && trimmed_result.contains("視聴"))
        || trimmed_result.contains("楽曲選択");

    if is_waiting_in_song_list {
        println!("曲選択画面にいる");
        return false;
    } else {
        println!("曲選択画面にいない");
        return true;
    }
}

struct ScreenInfo {
    width: i32,
    height: i32,
}

impl ScreenInfo {
    pub fn new(width: i32, height: i32) -> ScreenInfo {
        ScreenInfo { width, height }
    }
}
