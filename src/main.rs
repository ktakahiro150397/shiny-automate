use std::{env, thread, time::Duration};

use std::mem;
use winapi::um::winuser::{INPUT, INPUT_KEYBOARD, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP};

fn main() {
    // コマンドライン引数を取得
    let args: Vec<String> = env::args().collect();

    // 引数が2つない場合はエラーを表示
    if args.len() < 3 {
        eprintln!("Usage: shiny_mas <width px> <height px> [scale]");
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

    let mut scale: f32 = match args[3].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Scale is not a float.Fall back to 1.0");
            1.0
        }
    };

    println!(
        "Width: {}, Height: {}, Scale: {}",
        raw_width_px, raw_height_px, scale
    );

    let resolution_width_px: i32 = ((raw_width_px as f32) / scale) as i32;
    let resolution_height_px: i32 = ((raw_height_px as f32) / scale) as i32;

    println!(
        "Resolution: {}x{}",
        resolution_width_px, resolution_height_px
    );
    let screen = Screen::new(resolution_width_px, resolution_height_px);

    let wait_duration = Duration::new(150, 0);

    // 待機
    println!("Waiting for 5 seconds...");
    println!("1. シャニソンをウィンドウに戻す");
    println!("2. デスクトップをクリック");
    println!("3. シャニソンのタイトルバーをクリック");
    println!("4. Alt + Enterでフルスクリーンにする");
    thread::sleep(Duration::new(5, 0));
    println!("Start!");

    loop {
        println!("start_mv!");
        start_mv(&screen);

        // 右下に移動してカーソルを隠す
        set_pos_win32(resolution_width_px, resolution_height_px);

        // 再生中待機
        println!("Waiting for {} seconds...", wait_duration.as_secs());
        thread::sleep(wait_duration);
    }
}

struct Screen {
    width: i32,
    height: i32,
}

impl Screen {
    pub fn new(width: i32, height: i32) -> Screen {
        Screen { width, height }
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
        let mut mouse = input.u.mi_mut();
        mouse.dwFlags = MOUSEEVENTF_LEFTDOWN;
        // mouse.dx = x;
        // mouse.dy = y;

        winapi::um::winuser::SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
    }

    unsafe {
        // マウスの左クリック解放
        let mut input = mem::zeroed::<INPUT>();
        input.type_ = winapi::um::winuser::INPUT_MOUSE;
        let mut mouse = input.u.mi_mut();
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

fn start_mv(screen: &Screen) {
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

    let center_resume_button = ButtonPosition {
        name: String::from("中央 再生ボタン"),
        x_pos: ((screen.width as f32) * (10.0 / WIDTH)) as i32,
        y_pos: y_pos_val,
    };

    // ランダムボタンを押す
    click_position(&random_button);

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

    // 画面中央をクリック
    click_position(&center_resume_button);
    thread::sleep(Duration::new(1, 0));
    click_position(&center_resume_button);

    // 待機
    thread::sleep(Duration::new(1, 0));
}
