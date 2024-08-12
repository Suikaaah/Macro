use std::time::{Duration, Instant};

use super::input;

#[derive(PartialEq)]
enum State {
    Idle,
    UpReq,
    DownBeginReq,
    DownEndReq,
}

pub struct DoubleClick {
    active: bool,
    state: State,
    temporarily_disabled: bool,
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
            state: State::Idle,
            temporarily_disabled: false,
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
        if self.is_active() && self.state != State::DownEndReq {
            self.state = State::UpReq;
            self.origin = Instant::now();
        }
    }

    pub fn toggle(&mut self) {
        self.active ^= true;
    }

    pub fn execute(&mut self) {
        self.temporarily_disabled = false;

        let now = Instant::now();
        let expired_init = self.origin + self.delay_init <= now;
        let expired_on = self.origin + self.delay_on <= now;
        let expired_off = self.origin + self.delay_on + self.delay_off <= now;

        if expired_init && self.state == State::UpReq {
            self.state = State::DownBeginReq;
            input::mouse_r_up();
        }
        if expired_on && self.state == State::DownBeginReq {
            self.state = State::DownEndReq;
            input::mouse_r_down();
        }
        if expired_off && self.state == State::DownEndReq {
            self.state = State::Idle;
            input::mouse_r_up();
        }
    }
}
