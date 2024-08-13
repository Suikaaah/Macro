use super::input;
use super::sequencial::instruction::Instruction;

pub fn sequence() -> Vec<Instruction> {
    use Instruction::*;

    let home = input::mouse_pos();
    let left = Move(905, 473);

    vec![
        Down,
        Up,
        left,
        Down,
        Up,
        Move(1079, 476),
        Down,
        Up,
        Down,
        Up,
        Down,
        Up,
        Down,
        Up,
        Move(1175, 674),
        Down,
        Up,
        left,
        Down,
        Up,
        Move(home.0, home.1),
    ]
}
