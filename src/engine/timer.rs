use std::{
    thread,
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct Timer {
    duration: Duration,
    last_update: Instant,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            last_update: Instant::now(),
        }
    }

    pub fn end(&self) -> Instant {
        self.last_update + self.duration
    }

    pub fn update(&mut self) {
        if self.is_expired() {
            let now = Instant::now();
            let a = (now - self.last_update).as_nanos() % self.duration.as_nanos();
            self.last_update = now - Duration::from_nanos(a as u64);
        }
    }

    pub fn is_expired(&self) -> bool {
        self.end() <= Instant::now()
    }

    pub fn sleep(&mut self) {
        if !self.is_expired() {
            thread::sleep(self.end() - Instant::now());
        }

        self.update();
    }
}

pub struct Timers {
    pub poll: Timer,
    pub draw: Timer,
    pub lr: Timer,
    pub s: Timer,
}

impl Timers {
    pub fn new() -> Self {
        const DELAY_POLL: Duration = Duration::from_millis(1);
        const DELAY_DRAW: Duration = Duration::from_millis(4);
        const DELAY_LR: Duration = Duration::from_millis(20);
        const DELAY_S: Duration = Duration::from_millis(50);

        Self {
            poll: Timer::new(DELAY_POLL),
            draw: Timer::new(DELAY_DRAW),
            lr: Timer::new(DELAY_LR),
            s: Timer::new(DELAY_S),
        }
    }
}
