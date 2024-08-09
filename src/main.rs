use std::{thread, time::Duration};

use engine::Engine;
use sdl2::event::Event;
use timer::Timer;
use windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY;

mod engine;
mod input;
mod timer;

#[derive(Debug)]
enum Item {
    MouseMove(i32, i32),
    MouseDown,
    MouseUp,
    KeyDown(VIRTUAL_KEY),
    KeyUp(VIRTUAL_KEY),
}

impl Item {
    fn execute(&self) {
        use input::*;
        use Item::*;

        match *self {
            MouseMove(x, y) => mouse_move(x, y),
            MouseDown => mouse_down(),
            MouseUp => mouse_up(),
            KeyDown(vk) => key_down(vk),
            KeyUp(vk) => key_up(vk),
        }
    }
}

fn main() {
    let ttf = sdl2::ttf::init().unwrap();
    let mut engine = Engine::new((400, 300), &ttf);
    let mut timer = Timer::new(Duration::from_millis(4));

    use Item::*;
    let sequence = [
        MouseMove(0, 0),
        MouseMove(10, 0),
        MouseMove(0, 10),
        MouseMove(10, 10),
        MouseMove(0, 0),
        MouseMove(10, 0),
        MouseMove(0, 10),
        MouseMove(10, 10),
    ];

    'main_loop: loop {
        for event in engine.sdl.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit { .. } => break 'main_loop,
                _ => {}
            }
        }

        engine.clear();
        engine.text("HI", (10, 10));
        engine.render();

        timer.sleep();
    }
}
