use std::mem;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    self as kam, INPUT, INPUT_KEYBOARD, INPUT_MOUSE, KEYEVENTF_KEYUP, MAPVK_VK_TO_VSC,
    MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MOVE,
    MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, VIRTUAL_KEY, VK_C, VK_DOWN, VK_LCONTROL, VK_LSHIFT,
    VK_R, VK_RBUTTON, VK_TAB, VK_X, VK_Z,
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

pub fn mouse_l_down() {
    let mut input = INPUT::default();

    input.r#type = INPUT_MOUSE;
    input.Anonymous.mi.dwFlags = MOUSEEVENTF_LEFTDOWN;

    send_input(input);
}

pub fn mouse_l_up() {
    let mut input = INPUT::default();

    input.r#type = INPUT_MOUSE;
    input.Anonymous.mi.dwFlags = MOUSEEVENTF_LEFTUP;

    send_input(input);
}

pub fn mouse_r_down() {
    let mut input = INPUT::default();

    input.r#type = INPUT_MOUSE;
    input.Anonymous.mi.dwFlags = MOUSEEVENTF_RIGHTDOWN;

    send_input(input);
}

pub fn mouse_r_up() {
    let mut input = INPUT::default();

    input.r#type = INPUT_MOUSE;
    input.Anonymous.mi.dwFlags = MOUSEEVENTF_RIGHTUP;

    send_input(input);
}

pub fn key_down(vk: VIRTUAL_KEY) {
    let mut input = INPUT::default();

    input.r#type = INPUT_KEYBOARD;
    input.Anonymous.ki.wScan = unsafe { kam::MapVirtualKeyA(vk.0 as u32, MAPVK_VK_TO_VSC) }
        .try_into()
        .unwrap();
    input.Anonymous.ki.wVk = vk;

    send_input(input);
}

pub fn key_up(vk: VIRTUAL_KEY) {
    let mut input = INPUT::default();

    input.r#type = INPUT_KEYBOARD;
    input.Anonymous.ki.dwFlags = KEYEVENTF_KEYUP;
    input.Anonymous.ki.wScan = unsafe { kam::MapVirtualKeyA(vk.0 as u32, MAPVK_VK_TO_VSC) }
        .try_into()
        .unwrap();
    input.Anonymous.ki.wVk = vk;

    send_input(input);
}

pub fn is_key_down(vk: VIRTUAL_KEY) -> bool {
    unsafe { kam::GetAsyncKeyState(vk.0 as i32) as u16 & 0x8000 == 0x8000 }
}

pub struct Key {
    vk: VIRTUAL_KEY,
    current: bool,
    previous: bool,
}

impl Key {
    pub fn new(vk: VIRTUAL_KEY) -> Self {
        Self {
            vk,
            current: false,
            previous: false,
        }
    }

    #[must_use]
    pub fn update(&mut self) -> bool {
        self.previous = mem::replace(&mut self.current, is_key_down(self.vk));
        self.previous ^ self.current
    }

    pub fn is_down(&self) -> bool {
        self.current
    }

    pub fn is_down_first(&self) -> bool {
        self.current && !self.previous
    }

    pub fn combination(a: &Self, b: &Self) -> bool {
        (a.is_down() && b.is_down_first()) || (a.is_down_first() && b.is_down())
    }
}

pub struct Keys {
    pub z: Key,
    pub x: Key,
    pub c: Key,
    pub r: Key,
    pub tab: Key,
    pub shift: Key,
    pub ctrl: Key,
    pub down: Key,
    pub r_button: Key,
}

impl Keys {
    pub fn new() -> Self {
        Self {
            z: Key::new(VK_Z),
            x: Key::new(VK_X),
            c: Key::new(VK_C),
            r: Key::new(VK_R),
            tab: Key::new(VK_TAB),
            shift: Key::new(VK_LSHIFT),
            ctrl: Key::new(VK_LCONTROL),
            down: Key::new(VK_DOWN),
            r_button: Key::new(VK_RBUTTON),
        }
    }
}
