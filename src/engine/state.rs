use std::time::{Duration, Instant};

pub trait State {
    fn advance(&mut self);

    fn delay(&self) -> Option<Duration>;

    fn is_expired(&self, origin: &Instant) -> Option<bool> {
        self.delay().map(|d| *origin + d <= Instant::now())
    }

    fn action(&self);

    fn process(&mut self, origin: &Instant) {
        if let Some(expired) = self.is_expired(origin) {
            if expired {
                self.action();
                self.advance();
            }
        }
    }
}
