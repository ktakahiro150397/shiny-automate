use std::{os::windows, thread, time::Duration};

use mouse_rs::{types::keys::Keys, Mouse};

use std::mem;
use winapi::{
    shared::{
        minwindef::{BOOL, FALSE, LPARAM, TRUE},
        windef::HWND,
    },
    um::winuser::{
        EnumWindows, GetWindowTextW, SendInput, SetFocus, SetForegroundWindow, INPUT,
        INPUT_KEYBOARD, KEYEVENTF_KEYUP, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, VK_CONTROL,
    },
};
// mod shiny_mas;

fn main() {
    let mouse = Mouse::new();

    const MONITOR_RESOLUTION_WIDTH_PIXEL: i32 = (3840.0 / 1.25) as i32;
    const MONITOR_RESOLUTION_HEIGHT_PIXEL: i32 = (2560.0 / 1.25) as i32;

    let screen = Screen::new(
        MONITOR_RESOLUTION_WIDTH_PIXEL,
        MONITOR_RESOLUTION_HEIGHT_PIXEL,
    );

    let wait_duration = Duration::new(120, 0);

    // 待機
    println!("Waiting for 5 seconds...");
    thread::sleep(Duration::new(5, 0));
    println!("Start!");

    loop {
        // マウス位置初期化
        // mouse.move_to(0, 0).expect("Mouse move_to failed.");

        println!("start_mv!");
        start_mv(&screen, &mouse);

        // 右下に移動してカーソルを隠す
        set_pos_win32(
            MONITOR_RESOLUTION_WIDTH_PIXEL,
            MONITOR_RESOLUTION_HEIGHT_PIXEL,
        );

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

// impl ButtonPosition {
//     fn get_width_click_position(&self, screen: &Screen) -> i32 {
//         println!(
//             "width_scale: {}, screen.width: {}",
//             self.x_pos, screen.width
//         );
//         screen.width / self.x_pos as i32
//     }

//     fn get_height_click_position(&self, screen: &Screen) -> i32 {
//         screen.height / self.y_pos as i32
//     }
// }

unsafe fn create_input(key_code: u16, flags: u32) -> INPUT {
    let mut input = mem::zeroed::<INPUT>();

    input.type_ = INPUT_KEYBOARD;
    let mut ki = input.u.ki_mut();
    ki.wVk = key_code;
    ki.dwFlags = flags;
    input
}

fn set_pos_win32(x: i32, y: i32) {
    unsafe {
        winapi::um::winuser::SetCursorPos(x, y);
    }
    println!("SetCursorPos: x: {}, y: {}", x, y);
}

fn click_pos_win32(x: i32, y: i32) {
    set_pos_win32(x, y);

    unsafe {
        // マウスの左クリック押下
        let mut input = mem::zeroed::<INPUT>();
        input.type_ = winapi::um::winuser::INPUT_MOUSE;
        let mut mouse = input.u.mi_mut();
        mouse.dwFlags = MOUSEEVENTF_LEFTDOWN;
        mouse.dx = x;
        mouse.dy = y;

        winapi::um::winuser::SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
    }

    unsafe {
        // マウスの左クリック解放
        let mut input = mem::zeroed::<INPUT>();
        input.type_ = winapi::um::winuser::INPUT_MOUSE;
        let mut mouse = input.u.mi_mut();
        mouse.dwFlags = MOUSEEVENTF_LEFTUP;
        mouse.dx = x;
        mouse.dy = y;

        winapi::um::winuser::SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
    }
}

fn click_position(mouse: &Mouse, button_pos: &ButtonPosition) {
    println!(
        "click_position [{}] : x: {}, y: {}",
        button_pos.name, button_pos.x_pos, button_pos.y_pos
    );

    click_pos_win32(button_pos.x_pos, button_pos.y_pos);

    // mouse
    //     .move_to(button_pos.x_pos, button_pos.y_pos)
    //     .expect("Mouse move_to failed.");
    // mouse.click(&Keys::LEFT).expect("LEFT click failed.");
}

fn start_mv(screen: &Screen, mouse: &Mouse) {
    // ランダム→MV再生→スタート→中央クリック を押す一連の動作
    const HEIGHT: f32 = 10.0;
    const WIDTH: f32 = 20.0;

    println!(
        "screen.width: {}, screen.height: {}",
        screen.width, screen.height
    );

    let random_button = ButtonPosition {
        name: String::from("ランダム"),
        x_pos: ((screen.width as f32) * (6.0 / WIDTH)) as i32,
        y_pos: ((screen.height as f32) * (9.5 / HEIGHT)) as i32,
    };

    let mv_watch_button = ButtonPosition {
        name: String::from("MV視聴"),
        x_pos: ((screen.width as f32) * (15.0 / WIDTH)) as i32,
        y_pos: ((screen.height as f32) * (9.5 / HEIGHT)) as i32,
    };

    let mv_start_button = ButtonPosition {
        name: String::from("スタート"),
        x_pos: ((screen.width as f32) * (18.0 / WIDTH)) as i32,
        y_pos: ((screen.height as f32) * (9.5 / HEIGHT)) as i32,
    };

    let center_resume_button = ButtonPosition {
        name: String::from("中央 再生ボタン"),
        x_pos: ((screen.width as f32) * (10.0 / WIDTH)) as i32,
        y_pos: ((screen.height as f32) * (5.0 / HEIGHT)) as i32,
    };

    // ランダムボタンを押す
    click_position(mouse, &random_button);

    // 待機
    thread::sleep(Duration::new(3, 0));

    // スタートボタンを押す
    click_position(mouse, &mv_watch_button);

    // 待機
    thread::sleep(Duration::new(2, 0));

    // MV再生ボタンを押す
    click_position(mouse, &mv_start_button);

    // 待機
    thread::sleep(Duration::new(1, 0));

    // 画面中央をクリック
    click_position(mouse, &center_resume_button);
    thread::sleep(Duration::new(1, 0));
    click_position(mouse, &center_resume_button);

    // 待機
    thread::sleep(Duration::new(1, 0));
}
