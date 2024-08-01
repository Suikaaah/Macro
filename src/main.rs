use std::time::Duration;

use windows::Win32::UI::Input::KeyboardAndMouse::{
    self as kam, INPUT, INPUT_MOUSE, MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
    MOUSEEVENTF_MOVE,
};

fn send_input(input: INPUT) {
    let array = [input];

    unsafe {
        kam::SendInput(&array, std::mem::size_of_val(&array).try_into().unwrap());
    }
}

fn move_mouse(x: i32, y: i32) {
    let mut input = INPUT::default();

    input.r#type               = INPUT_MOUSE;
    input.Anonymous.mi.dx      = (x as f32 * 65535.0 / 1920.0) as i32;
    input.Anonymous.mi.dy      = (y as f32 * 65535.0 / 1080.0) as i32;
    input.Anonymous.mi.dwFlags = MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_MOVE;

    send_input(input);
}

fn mouse_down() {
    let mut input = INPUT::default();

    input.r#type               = INPUT_MOUSE;
    input.Anonymous.mi.dwFlags = MOUSEEVENTF_LEFTDOWN;

    send_input(input);
}

fn mouse_up() {
    let mut input = INPUT::default();

    input.r#type               = INPUT_MOUSE;
    input.Anonymous.mi.dwFlags = MOUSEEVENTF_LEFTUP;

    send_input(input);
}

fn sleep(ms: u64) {
    std::thread::sleep(Duration::from_millis(ms));
}

#[derive(Clone, Copy)]
enum Item {
    SetSleep(u64),
    Point(i32, i32),
}

fn main() {
    use Item::*;

    let rr   = Point(1049, 311);
    let r    = Point( 906, 379);
    let u    = Point( 763, 311);
    let l    = Point( 628, 380);
    let d    = Point( 766, 458);
    let make = Point(1288, 504);
    let s_0  = Point( 931, 562);
    let s_1  = Point( 970, 562);
    let s_2  = Point(1009, 562);
    let s_3  = Point(1050, 562);
    let s_4  = Point(1091, 562);
    let x    = Point(1309, 341);

    let sequence = [
        SetSleep(750),
        rr,
        SetSleep(75),
        s_0, make, make,
        s_1, make, make,
        s_2, make, make,
        s_3, make, make,
        s_4, make, make,
        SetSleep(750),
        x,
        SetSleep(75),
        rr, r, u, l, d
    ];

    let mut ms: u64 = 0;
    for _ in 0..10 {
        for item in sequence {
            match item {
                SetSleep(x) => {
                    ms = x;
                },
                Point(x, y) => {
                    move_mouse(x, y);
                    sleep(10);
                    mouse_down();
                    sleep(10);
                    mouse_up();
                    sleep(ms);
                }
            }
        }

        sleep(3500);
    }
}
