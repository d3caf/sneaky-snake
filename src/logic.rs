use crate::GameState;

pub enum Move {
    Left,
    Right,
    Up,
    Down,
}

pub fn get_move(_state: &GameState) -> Move {
    println!("Move!");
    return Move::Left;
}
