use mouse_rs::{types::keys::Keys, Mouse};
// mod shiny_mas;

fn main() {
    let mouse = Mouse::new();

    mouse.move_to(0, 0).expect("move_to failed.");
    mouse.press(&Keys::RIGHT).expect("Press failed.");
    mouse.release(&Keys::RIGHT).expect("Release failed.");

    // ALLタブの順序を取得し、それに合わせて自動でマウスを移動させる
    // TODO : 曲ごとに何秒待機させるか？
}
