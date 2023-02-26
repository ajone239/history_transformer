use std::io::prelude::*;
use std::{
    error::Error,
    fs::File,
    io::{BufReader, Lines},
};

// use chess::{Board, ChessMove};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("data/for_austin.txt")?;
    let mut lines = BufReader::new(f).lines();

    let data = read_count_lines::<5>(&mut lines)?;

    let game = Game::new_from_str_vec(data);

    println!("{game:#?}");

    Ok(())
}

fn read_count_lines<const N: usize>(
    lines: &mut Lines<BufReader<File>>,
) -> Result<[String; N], std::io::Error> {
    const EMPTY_STRING: String = String::new();

    let mut data = [EMPTY_STRING; N];

    for d in data.iter_mut() {
        // TODO(austin): Handle this unwrap
        let line = lines.next().unwrap()?;
        *d = line;
    }
    Ok(data)
}

#[derive(Debug, Default)]
enum Outcome {
    Black,
    White,
    #[default]
    Draw,
}

impl Outcome {
    fn new_from_str(s: &str) -> Self {
        match &s[2..] {
            "1-0" => Self::White,
            "0-1" => Self::Black,
            _ => Self::Draw,
        }
    }
}

#[derive(Debug, Default)]
struct Game {
    outcome: Outcome,
    white_elo: usize,
    black_elo: usize,
    moves: Vec<String>,
    event: String,
}

impl Game {
    fn new_from_str_vec(data: [String; 5]) -> Self {
        let outcome = Outcome::new_from_str(&data[0]);

        // TODO(austin): Take this out if slow
        let white_elo = data[1][2..].parse::<usize>().unwrap();
        let black_elo = data[2][2..].parse::<usize>().unwrap();

        // TODO(austin): Make this faster
        let moves: Vec<String> = data[3][2..].split(' ').map(|s| s.to_string()).collect();

        // TODO(austin): Take this out if slow
        let event = data[4][2..].to_string();

        Self {
            outcome,
            white_elo,
            black_elo,
            moves,
            event,
        }
    }
}
