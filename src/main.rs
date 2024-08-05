use std::{env, thread, time::Duration};

use chrono::Local;
use rand::Rng as Random;
use std::mem;
use win_ocr;
use winapi::um::winuser::{INPUT, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP};

fn main() {
    // コマンドライン引数を取得
    let args: Vec<String> = env::args().collect();

    // 引数が2つない場合はエラーを表示
    if args.len() < 3 {
        eprintln!("Usage: shiny_mas <width px> <height px> <scale> <index>");
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

    println!(
        "Width: {}, Height: {}, Scale: {}, Monitor Index: {}",
        raw_width_px, raw_height_px, scale, monitor_index
    );

    let resolution_width_px: i32 = ((raw_width_px as f32) / scale) as i32;
    let resolution_height_px: i32 = ((raw_height_px as f32) / scale) as i32;

    println!(
        "Resolution: {}x{}",
        resolution_width_px, resolution_height_px
    );
    let screen_info = ScreenInfo::new(resolution_width_px, resolution_height_px);

    let wait_duration = Duration::new(1, 0);

    // 待機
    println!("Waiting for 5 seconds...");
    println!("1. シャニソンをウィンドウに戻す");
    println!("2. デスクトップをクリック");
    println!("3. シャニソンのタイトルバーをクリック");
    println!("4. Alt + Enterでフルスクリーンにする");
    thread::sleep(Duration::new(5, 0));
    println!("Start!");

    loop {
        if is_playing(monitor_index, &screen_info) {
            // 再生中の場合、待機する
            println!("再生中のため、{}秒待機", wait_duration.as_secs());
        } else {
            // 再生中でない場合、ランダム再生する
            println!("楽曲選択画面にいるため、ランダム再生");
            start_mv(&screen_info);

            // 右下に移動してカーソルを隠す
            set_pos_win32(resolution_width_px, resolution_height_px);
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

struct ButtonPosition {
    name: String,
    x_pos: i32,
    y_pos: i32,
}

fn set_pos_win32(x: i32, y: i32) {
    unsafe {
        winapi::um::winuser::SetCursorPos(x, y);
    }
}

fn click_pos_win32(x: i32, y: i32) {
    set_pos_win32(x, y);

    unsafe {
        // マウスの左クリック押下
        let mut input = mem::zeroed::<INPUT>();
        input.type_ = winapi::um::winuser::INPUT_MOUSE;
        let mouse = input.u.mi_mut();
        mouse.dwFlags = MOUSEEVENTF_LEFTDOWN;
        // mouse.dx = x;
        // mouse.dy = y;

        winapi::um::winuser::SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
    }

    unsafe {
        // マウスの左クリック解放
        let mut input = mem::zeroed::<INPUT>();
        input.type_ = winapi::um::winuser::INPUT_MOUSE;
        let mouse = input.u.mi_mut();
        mouse.dwFlags = MOUSEEVENTF_LEFTUP;
        // mouse.dx = x;
        // mouse.dy = y;

        winapi::um::winuser::SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
    }
}

fn click_position(button_pos: &ButtonPosition) {
    println!(
        "click_position [{}] : x: {}, y: {}",
        button_pos.name, button_pos.x_pos, button_pos.y_pos
    );

    click_pos_win32(button_pos.x_pos, button_pos.y_pos);
}

fn start_mv(screen: &ScreenInfo) {
    // ランダム→MV再生→スタート→中央クリック を押す一連の動作
    const HEIGHT: f32 = 10.0;
    const WIDTH: f32 = 20.0;

    println!(
        "screen.width: {}, screen.height: {}",
        screen.width, screen.height
    );

    let y_pos_val = ((screen.height as f32) * (9.1 / HEIGHT)) as i32;

    let random_button = ButtonPosition {
        name: String::from("ランダム"),
        x_pos: ((screen.width as f32) * (6.0 / WIDTH)) as i32,
        y_pos: y_pos_val,
    };

    let mv_watch_button = ButtonPosition {
        name: String::from("MV視聴"),
        x_pos: ((screen.width as f32) * (15.0 / WIDTH)) as i32,
        y_pos: y_pos_val,
    };

    let mv_start_button = ButtonPosition {
        name: String::from("スタート"),
        x_pos: ((screen.width as f32) * (18.0 / WIDTH)) as i32,
        y_pos: y_pos_val,
    };

    // ランダムボタンをランダム回数押す
    let random_count = rand::thread_rng().gen_range(1..4);
    println!("Shuffling...");
    for _ in 0..random_count {
        click_position(&random_button);
        thread::sleep(Duration::new(1, 0));
    }
    println!("{} time(s)!", random_count);

    // 待機
    thread::sleep(Duration::new(3, 0));

    // スタートボタンを押す
    click_position(&mv_watch_button);

    // 待機
    thread::sleep(Duration::new(2, 0));

    // MV再生ボタンを押す
    click_position(&mv_start_button);

    // 待機
    thread::sleep(Duration::new(1, 0));
}
