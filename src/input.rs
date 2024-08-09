use windows::Win32::UI::Input::KeyboardAndMouse::{
    self as kam, INPUT, INPUT_KEYBOARD, INPUT_MOUSE, KEYEVENTF_KEYUP, MOUSEEVENTF_ABSOLUTE,
    MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MOVE, VIRTUAL_KEY, VK_6,
};

pub fn send_input(input: INPUT) {
    let array = [input];

    unsafe {
        kam::SendInput(&array, std::mem::size_of_val(&array).try_into().unwrap());
    }
}

pub fn mouse_move(x: i32, y: i32) {
    let mut input = INPUT::default();

    input.r#type = INPUT_MOUSE;
    input.Anonymous.mi.dx = (x as f32 * 65535.0 / 1920.0) as i32;
    input.Anonymous.mi.dy = (y as f32 * 65535.0 / 1080.0) as i32;
    input.Anonymous.mi.dwFlags = MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_MOVE;

    send_input(input);
}

pub fn mouse_down() {
    let mut input = INPUT::default();

    input.r#type = INPUT_MOUSE;
    input.Anonymous.mi.dwFlags = MOUSEEVENTF_LEFTDOWN;

    send_input(input);
}

pub fn mouse_up() {
    let mut input = INPUT::default();

    input.r#type = INPUT_MOUSE;
    input.Anonymous.mi.dwFlags = MOUSEEVENTF_LEFTUP;

    send_input(input);
}

pub fn key_down(vk: VIRTUAL_KEY) {
    let mut input = INPUT::default();

    input.r#type = INPUT_KEYBOARD;
    input.Anonymous.ki.wVk = vk;

    send_input(input);
}

pub fn key_up(vk: VIRTUAL_KEY) {
    let mut input = INPUT::default();

    input.r#type = INPUT_KEYBOARD;
    input.Anonymous.ki.dwFlags = KEYEVENTF_KEYUP;
    input.Anonymous.ki.wVk = vk;

    send_input(input);
}
