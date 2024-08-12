use std::time::{Duration, Instant};

use super::input;
use super::state::State;

#[derive(PartialEq)]
enum LocalState {
    Idle,
    UpReq,
    DownBeginReq,
    DownEndReq,
}

impl State for LocalState {
    fn advance(&mut self) {
        use LocalState::*;

        *self = match *self {
            Idle => UpReq,
            UpReq => DownBeginReq,
            DownBeginReq => DownEndReq,
            DownEndReq => Idle,
        };
    }

    fn delay(&self) -> Option<Duration> {
        use LocalState::*;

        match *self {
            Idle => None,
            UpReq => Some(Duration::from_millis(25)),
            DownBeginReq => Some(Duration::from_millis(50)),
            DownEndReq => Some(Duration::from_millis(75)),
        }
    }

    fn action(&self) {
        use LocalState::*;

        match *self {
            Idle => {},
            UpReq => input::mouse_r_up(),
            DownBeginReq => input::mouse_r_down(),
            DownEndReq => input::mouse_r_up(),
        }
    }
}

pub struct DoubleClick {
    active: bool,
    state: LocalState,
    temporarily_disabled: bool,
    origin: Instant,
}

impl DoubleClick {
    pub fn new() -> Self {
        Self {
            active: false,
            state: LocalState::Idle,
            temporarily_disabled: false,
            origin: Instant::now(),
        }
    }

    pub fn is_active(&self) -> bool {
        self.active && !self.temporarily_disabled
    }

    pub fn temporarily_disable(&mut self) {
        self.temporarily_disabled = true
    }

    pub fn request(&mut self) {
        if self.is_active() && self.state != LocalState::DownEndReq {
            self.state = LocalState::UpReq;
            self.origin = Instant::now();
        }
    }

    pub fn toggle(&mut self) {
        self.active ^= true;
    }

    pub fn execute(&mut self) {
        self.temporarily_disabled = false;
        self.state.process(self.origin);
    }
}
