use windows::Win32::UI::Input::KeyboardAndMouse::VK_SPACE;

use super::input;

pub struct Alternator {
    f_false: fn(),
    f_true: fn(),
    state: bool,
    active: bool,
}

impl Alternator {
    pub fn new(f_false: fn(), f_true: fn()) -> Self {
        Self {
            f_false,
            f_true,
            state: false,
            active: false,
        }
    }

    pub fn toggle(&mut self) {
        self.active ^= true;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn execute(&mut self) {
        if !self.active {
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
