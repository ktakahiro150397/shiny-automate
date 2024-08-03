use mouse_rs::{types::keys::Keys, Mouse};

fn main() {
    let mouse = Mouse::new();

    mouse.move_to(0, 0).expect("move_to failed.");
    mouse.press(&Keys::RIGHT).expect("Press failed.");
    mouse.release(&Keys::RIGHT).expect("Release failed.");
}
