use std::{thread, time::{Duration, Instant}};

pub struct Timer {
    duration: Duration,
    sleep_until: Instant
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            sleep_until: Instant::now() + duration
        }
    }

    pub fn sleep(&mut self) {
        let now = Instant::now();

        if now < self.sleep_until {
            thread::sleep(self.sleep_until - now);
        }

        self.sleep_until += self.duration;
    }
}