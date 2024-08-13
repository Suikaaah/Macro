use super::input;
use super::sequencial::instruction::Instruction;
use super::sequencial::Sequencial;

enum SeqType {
    A,
    B(i32, i32),
}

fn sequence(seq_type: SeqType) -> Vec<Instruction> {
    use Instruction::*;

    let left = Move(905, 473);
    let right = Move(1079, 476);

    match seq_type {
        SeqType::A => vec![Down, Up, left, Down, Up, right],
        SeqType::B(x, y) => vec![Down, Up, left, Down, Up, Move(x, y)],
    }
}

pub struct Trade {
    seq_a: Sequencial,
    seq_b: Sequencial,
    active: bool,
    home: (i32, i32),
}

impl Trade {
    pub fn new() -> Self {
        Self {
            seq_a: Sequencial::new(),
            seq_b: Sequencial::new(),
            active: false,
            home: Default::default(),
        }
    }

    pub fn request(&mut self) {
        self.active = true;
        self.home = input::mouse_pos();
        self.seq_a.set(sequence(SeqType::A));
        self.seq_a.request();
    }

    pub fn resume(&mut self) {
        if self.active {
            self.active = false;
            self.seq_b
                .set(sequence(SeqType::B(self.home.0, self.home.1)));
            self.seq_b.request();
        }
    }

    pub fn execute(&mut self) {
        if self.active {
            self.seq_a.execute();
        } else {
            self.seq_b.execute();
        }
    }
}
