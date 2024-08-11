use std::time::{Duration, Instant};

use super::input;

pub struct DoubleClick {
    active: bool,
    requested: bool,
    up_requested: bool,
    temporarily_disabled: bool,
    down: bool,
    origin: Instant,
    delay_init: Duration,
    delay_on: Duration,
    delay_off: Duration,
}

impl DoubleClick {
    pub fn new() -> Self {
        const DELAY_INIT: Duration = Duration::from_millis(25);
        const DELAY_ON: Duration = Duration::from_millis(50);
        const DELAY_OFF: Duration = Duration::from_millis(25);

        Self {
            active: false,
            requested: false,
            up_requested: false,
            temporarily_disabled: false,
            down: false,
            origin: Instant::now(),
            delay_init: DELAY_INIT,
            delay_on: DELAY_ON,
            delay_off: DELAY_OFF,
        }
    }

    pub fn is_active(&self) -> bool {
        self.active && !self.temporarily_disabled
    }

    pub fn temporarily_disable(&mut self) {
        self.temporarily_disabled = true
    }

    pub fn request(&mut self) {
        if self.is_active() && !self.down {
            self.requested = true;
            self.up_requested = true;
            self.origin = Instant::now();
        }
    }

    pub fn toggle(&mut self) {
        self.active ^= true;
    }

    pub fn update(&mut self) {
        self.temporarily_disabled = false;
        let now = Instant::now();
        let expired_init = self.origin + self.delay_init <= now;
        let expired_on = self.origin + self.delay_on <= now;
        let expired_off = self.origin + self.delay_on + self.delay_off <= now;

        if expired_init && self.up_requested {
            self.up_requested = false;
            input::mouse_r_up();
        }
        if expired_on && self.requested {
            self.requested = false;
            self.down = true;
            input::mouse_r_down();
        }
        if expired_off && self.down {
            self.down = false;
            input::mouse_r_up();
        }
    }
}
