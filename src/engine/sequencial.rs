pub mod instruction;

use super::input;
use super::state::State;
use instruction::Instruction;
use std::time::{Duration, Instant};

struct LocalState {
    sequence: Vec<Instruction>,
    index: Option<usize>,
}

impl LocalState {
    fn new() -> Self {
        Self {
            sequence: Vec::default(),
            index: None,
        }
    }

    fn set(&mut self, sequence: Vec<Instruction>) {
        self.sequence = sequence;
    }

    fn advanceable(&self) -> bool {
        self.index.map_or(false, |i| i + 1 < self.sequence.len())
    }

    fn begin(&mut self) {
        if !self.sequence.is_empty() {
            self.index = Some(0);
        }
    }
}

impl State for LocalState {
    fn advance(&mut self) {
        self.index = match self.index {
            Some(index) if self.advanceable() => Some(index + 1),
            _ => None,
        }
    }

    fn delay(&self) -> Option<Duration> {
        const DELAY: usize = 5;

        self.index
            .map(|i| Duration::from_millis((i * DELAY).try_into().unwrap()))
    }

    fn action(&self) {
        if let Some(index) = self.index {
            self.sequence[index].action();
        }
    }
}

pub struct Sequencial {
    state: LocalState,
    origin: Instant,
}

impl Sequencial {
    pub fn new() -> Self {
        Self {
            state: LocalState::new(),
            origin: Instant::now(),
        }
    }

    pub fn set(&mut self, sequence: Vec<Instruction>) {
        self.state.set(sequence);
    }

    pub fn request(&mut self) {
        self.state.begin();
        self.origin = Instant::now();
    }

    pub fn execute(&mut self) {
        self.state.process(&self.origin);
    }
}
