use super::input;

#[derive(Clone, Copy)]
pub enum Instruction {
    Move(i32, i32),
    Down,
    Up,
}

impl Instruction {
    pub fn action(&self) {
        use input::*;
        use Instruction::*;

        match *self {
            Move(x, y) => mouse_move(x, y),
            Down => mouse_l_down(),
            Up => mouse_l_up(),
        }
    }
}
