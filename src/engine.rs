mod alternator;
mod input;
mod sdl_handler;
mod timer;

use alternator::Alternators;
use input::Keys;
use sdl2::{event::Event, pixels::Color, rect::Rect, ttf::Sdl2TtfContext};
use sdl_handler::SDLHandler;
use timer::Timers;

pub struct Engine {
    ttf: Sdl2TtfContext,
    timers: Timers,
    alt: Alternators,
    keys: Keys,
    draw: bool,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            ttf: sdl2::ttf::init().unwrap(),
            timers: Timers::new(),
            alt: Alternators::new(),
            keys: Keys::new(),
            draw: true,
        }
    }

    pub fn run(&mut self) {
        const RESOLUTION: (u32, u32) = (400, 300);

        let mut handler = SDLHandler::new(RESOLUTION, &self.ttf);

        'main_loop: loop {
            // input
            if self.keys.z.is_down() {
                self.draw = true;
                self.alt.ls.toggle();
            }
            if self.keys.x.is_down() {
                self.draw = true;
                self.alt.rs.toggle();
            }
            if self.keys.c.is_down() {
                self.draw = true;
                self.alt.ss.toggle();
            }

            // draw
            if self.timers.draw.expired() {
                self.timers.draw.update();

                for event in handler.event_pump().poll_iter() {
                    match event {
                        Event::Quit { .. } => break 'main_loop,
                        _ => {}
                    }
                }

                if self.draw {
                    self.draw = false;

                    handler.clear();

                    if self.alt.ls.is_activated() {
                        handler.rect(Rect::new(10, 10, 10, 10), Color::RGB(0xFF, 0, 0));
                    }
                    if self.alt.rs.is_activated() {
                        handler.rect(Rect::new(60, 10, 10, 10), Color::RGB(0, 0xFF, 0));
                    }
                    if self.alt.ss.is_activated() {
                        handler.rect(Rect::new(110, 10, 10, 10), Color::RGB(0, 0, 0xFF));
                    }
                    handler.text("ls", (10, 10));
                    handler.text("rs", (60, 10));
                    handler.text("ss", (110, 10));

                    handler.render();
                }
            }

            // spam
            if self.timers.lr.expired() {
                self.timers.lr.update();

                self.alt.ls.execute();
                self.alt.rs.execute();
            }
            if self.timers.s.expired() {
                self.timers.s.update();

                self.alt.ss.execute();
            }

            // sleep
            self.timers.poll.sleep();
        }
    }
}
