use std::collections::HashMap;

mod logic;

type Position = (u8, u8);

#[derive(Debug)]
enum GameSource {
    Tournament,
    League,
    Arena,
    Challenge,
    Custom,
}

#[derive(Debug)]
pub struct Game {
    id: String,
    map: String,
    timeout: u16,
    source: GameSource,
    ruleset: Option<HashMap<String, String>>,
}
impl Game {
    fn new() -> Self {
        return Game {
            id: String::from("test"),
            map: String::from("standard"),
            timeout: 500,
            source: GameSource::Custom,
            ruleset: Option::None,
        };
    }
}

#[derive(Debug)]
pub struct Snake {
    id: String,
    name: String,
    health: u8,
    body: Vec<Position>,
    latency: u64,
    head: Position,
    length: u16,
    shout: String,
    squad: String,
}

#[derive(Debug)]
pub struct Board {
    height: u8,
    width: u8,
    food: Vec<Position>,
    hazards: Vec<Position>,
    snakes: Vec<Snake>,
}

#[derive(Debug)]
struct GameState {
    game: Game,
    board: Board,
    turn: u32,
    you: Snake,
}

fn main() {
    let game = Game::new();
    dbg!(game);
    println!("Hello, world!");
}
