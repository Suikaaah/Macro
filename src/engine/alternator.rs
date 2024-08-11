use windows::Win32::UI::Input::KeyboardAndMouse::VK_SPACE;

use super::input;

pub struct Alternator {
    f_false: fn(),
    f_true: fn(),
    state: bool,
    activated: bool,
}

impl Alternator {
    pub fn new(f_false: fn(), f_true: fn()) -> Self {
        Self {
            f_false,
            f_true,
            state: false,
            activated: false,
        }
    }

    pub fn toggle(&mut self) {
        self.activated ^= true;
    }

    pub fn is_activated(&self) -> bool {
        self.activated
    }

    pub fn execute(&mut self) {
        if !self.activated {
            if self.state {
                (self.f_true)();
                self.state = false;
            }
        } else {
            if self.state {
                (self.f_true)();
            } else {
                (self.f_false)();
            }

            self.state ^= true;
        }
    }
}

pub struct Alternators {
    pub ls: Alternator,
    pub rs: Alternator,
    pub ss: Alternator,
}

impl Alternators {
    pub fn new() -> Self {
        use input::*;

        Self {
            ls: Alternator::new(|| mouse_l_down(), || mouse_l_up()),
            rs: Alternator::new(|| mouse_r_down(), || mouse_r_up()),
            ss: Alternator::new(|| key_down(VK_SPACE), || key_up(VK_SPACE)),
        }
    }
}
