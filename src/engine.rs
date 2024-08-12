mod alternator;
mod double_click;
mod input;
mod sdl_handler;
mod timer;

use alternator::Alternators;
use double_click::DoubleClick;
use input::{Key, Keys};
use sdl2::{event::Event, pixels::Color, ttf::Sdl2TtfContext};
use sdl_handler::SDLHandler;
use timer::Timers;

pub struct Engine {
    ttf: Sdl2TtfContext,
    timers: Timers,
    alts: Alternators,
    keys: Keys,
    draw: bool,
    locked: bool,
    double_click: DoubleClick,
}

enum TextColor {
    Green,
    Red,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            ttf: sdl2::ttf::init().unwrap(),
            timers: Timers::new(),
            alts: Alternators::new(),
            keys: Keys::new(),
            draw: true,
            locked: false,
            double_click: DoubleClick::new(),
        }
    }

    pub fn run(&mut self) {
        const RESOLUTION: (u32, u32) = (300, 100);

        let mut handler = SDLHandler::new(RESOLUTION, &self.ttf);

        'main_loop: loop {
            macro_rules! handle_idf_nodraw {
                ($key: ident, $something: tt) => {{
                    let _ = self.keys.$key.update();

                    if self.keys.$key.is_down_first() {
                        $something
                    }
                }};
            }
            macro_rules! handle_idf {
                ($key: ident, $something: tt) => {{
                    let _ = self.keys.$key.update();

                    if self.keys.$key.is_down_first() {
                        self.draw = true;
                        $something
                    }
                }};
            }
            macro_rules! handle_id {
                ($key: ident, $something: tt) => {{
                    self.draw |= self.keys.$key.update();

                    if self.keys.$key.is_down() {
                        $something
                    }
                }};
            }
            macro_rules! handle_com {
                ($key_a: ident, $key_b: ident, $something: tt) => {{
                    let _ = self.keys.$key_a.update();
                    let _ = self.keys.$key_b.update();

                    if Key::combination(&self.keys.$key_a, &self.keys.$key_b) {
                        self.draw = true;
                        $something
                    }
                }};
            }
            macro_rules! handle_timer {
                ($timer: ident, $something: tt) => {{
                    if self.timers.$timer.is_expired() {
                        self.timers.$timer.update();

                        $something
                    }
                }};
            }

            handle_id!(shift, { self.double_click.temporarily_disable(); });
            handle_idf_nodraw!(r_button, { self.double_click.request(); });
            if !self.locked {
                handle_idf!(tab, { self.double_click.toggle(); });
                handle_idf!(z, { self.alts.ls.toggle(); });
                handle_idf!(x, { self.alts.rs.toggle(); });
                handle_idf!(c, { self.alts.ss.toggle(); });
            }
            handle_com!(ctrl, down, { self.locked ^= true; });

            // draw and event
            handle_timer!(draw, {

                for event in handler.event_pump().poll_iter() {
                    match event {
                        Event::Quit { .. } => break 'main_loop,
                        _ => {}
                    }
                }

                if self.draw {
                    self.draw = false;

                    handler.clear();

                    use TextColor::*;
                    const DC: (i32, i32) = (10, 10);
                    const LS: (i32, i32) = (60, 10);
                    const RS: (i32, i32) = (110, 10);
                    const SS: (i32, i32) = (160, 10);
                    const LK: (i32, i32) = (10, 30);

                    handler.text("dc", DC, Self::color(Green, self.double_click.is_active()));
                    handler.text("ls", LS, Self::color(Green, self.alts.ls.is_active()));
                    handler.text("rs", RS, Self::color(Green, self.alts.rs.is_active()));
                    handler.text("ss", SS, Self::color(Green, self.alts.ss.is_active()));
                    handler.text("lk", LK, Self::color(Red, self.locked));

                    handler.render();
                }
            });

            // action
            self.double_click.execute();

            handle_timer!(lr, {
                self.alts.ls.execute();
                self.alts.rs.execute();
            });
            handle_timer!(s, { self.alts.ss.execute(); });

            // sleep
            self.timers.poll.sleep();
        }
    }

    fn color(tc: TextColor, active: bool) -> Color {
        match tc {
            TextColor::Green if active => Color::RGB(0, 0xFF, 0),
            TextColor::Red if active => Color::RGB(0xFF, 0, 0),
            _ => Color::RGB(0x7F, 0x7F, 0x7F),
        }
    }
}
