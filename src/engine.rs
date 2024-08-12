mod alternator;
mod double_click;
mod input;
mod sdl_handler;
mod timer;

use alternator::Alternators;
use double_click::DoubleClick;
use input::Keys;
use sdl2::{event::Event, pixels::Color, ttf::Sdl2TtfContext};
use sdl_handler::SDLHandler;
use timer::Timers;

pub struct Engine {
    ttf: Sdl2TtfContext,
    timers: Timers,
    alts: Alternators,
    keys: Keys,
    draw: bool,
    double_click: DoubleClick,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            ttf: sdl2::ttf::init().unwrap(),
            timers: Timers::new(),
            alts: Alternators::new(),
            keys: Keys::new(),
            draw: true,
            double_click: DoubleClick::new(),
        }
    }

    pub fn run(&mut self) {
        const RESOLUTION: (u32, u32) = (300, 100);

        let mut handler = SDLHandler::new(RESOLUTION, &self.ttf);

        'main_loop: loop {
            // input
            let _ = self.keys.r_button.update();
            self.draw |= self.keys.shift.update();
            self.draw |= self.keys.tab.update();
            self.draw |= self.keys.z.update();
            self.draw |= self.keys.x.update();
            self.draw |= self.keys.c.update();

            if self.keys.shift.is_down() {
                self.double_click.temporarily_disable();
            }
            if self.keys.r_button.is_down_first() {
                self.double_click.request();
            }
            if self.keys.tab.is_down_first() {
                self.double_click.toggle();
            }
            if self.keys.z.is_down_first() {
                self.alts.ls.toggle();
            }
            if self.keys.x.is_down_first() {
                self.alts.rs.toggle();
            }
            if self.keys.c.is_down_first() {
                self.alts.ss.toggle();
            }

            // draw and event
            if self.timers.draw.is_expired() {
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

                    let color_gen = |active| {
                        if active {
                            Color::RGB(0, 0xFF, 0)
                        } else {
                            Color::RGB(0x7F, 0x7F, 0x7F)
                        }
                    };

                    handler.text("dc", (10, 10), color_gen(self.double_click.is_active()));
                    handler.text("ls", (60, 10), color_gen(self.alts.ls.is_active()));
                    handler.text("rs", (110, 10), color_gen(self.alts.rs.is_active()));
                    handler.text("ss", (160, 10), color_gen(self.alts.ss.is_active()));

                    handler.render();
                }
            }

            // action
            self.double_click.update();

            if self.timers.lr.is_expired() {
                self.timers.lr.update();

                self.alts.ls.execute();
                self.alts.rs.execute();
            }
            if self.timers.s.is_expired() {
                self.timers.s.update();

                self.alts.ss.execute();
            }

            // sleep
            self.timers.poll.sleep();
        }
    }
}
