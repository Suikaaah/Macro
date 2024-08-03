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

fn process(sequence: &[Item]) {
    use Item::*;

    let mut ms = 0;

    for item in sequence {
        match *item {
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
}

fn main() {
    use Item::*;

    let rr   = Point(1049, 331);
    let r    = Point( 906, 399);
    let u    = Point( 763, 331);
    let l    = Point( 628, 400);
    let d    = Point( 766, 478);
    let make = Point(1288, 524);
    let s_0  = Point( 931, 582);
    let s_1  = Point( 970, 582);
    let s_2  = Point(1009, 582);
    let s_3  = Point(1050, 582);
    let s_4  = Point(1091, 582);
    let x    = Point(1309, 361);

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
        rr, r, u, l, d,
    ];

    for _ in 0..20 {
        process(&sequence);
        sleep(3500);
    }
}
