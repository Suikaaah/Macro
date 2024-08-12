use std::time::{Duration, Instant};

pub trait State {
    fn advance(&mut self);

    fn delay(&self) -> Option<Duration>;

    fn is_expired(&self, origin: Instant, now: Instant) -> Option<bool> {
        self
            .delay()
            .map(|d| origin + d <= now)
    }

    fn action(&self);

    fn process(&mut self, origin: Instant) {
        let now = Instant::now();

        if let Some(expired) = self.is_expired(origin, now) {
            if expired {
                self.action();
                self.advance();
            }
        }
    }
}