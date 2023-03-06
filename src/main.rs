use std::io::prelude::*;

use std::{error::Error, fs::File, io::BufReader, sync::mpsc, thread};

// use chess::{Board, ChessMove};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("data/for_austin.txt")?;
    let lines = BufReader::new(f).lines();

    let (tx, rx) = mpsc::channel::<Vec<String>>();

    let handle = thread::spawn(move || {
        while let Ok(data) = rx.recv() {
            let game = Game::new_from_str_vec(&data[..]);

            println!("{game:#?}");
        }
    });

    let mut data = vec![];
    let mut state = States::Event;

    for line in lines {
        let line = line?;
        if &line[..2] == state.value() {
            data.push(line[2..].to_string());
            state = state.next();
        } else {
            data.clear();
            state = States::Event;
            continue;
        }
        if data.len() >= 5 {
            tx.send(data.clone())?;
            data.clear();
            state = States::Event
        }
    }

    drop(tx);

    handle.join().unwrap();

    Ok(())
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

// order of data
//
// event
// outcome
// White
// Black
// moves
enum States {
    Event,
    Outcome,
    WhiteElo,
    BlackElo,
    LeMoves,
}

impl States {
    fn value(&self) -> &str {
        match self {
            Self::Event => "E,",
            Self::Outcome => "O,",
            Self::WhiteElo => "W,",
            Self::BlackElo => "B,",
            Self::LeMoves => "L,",
        }
    }
    fn next(&self) -> Self {
        match self {
            Self::Event => Self::Outcome,
            Self::Outcome => Self::WhiteElo,
            Self::WhiteElo => Self::BlackElo,
            Self::BlackElo => Self::LeMoves,
            Self::LeMoves => Self::Event,
        }
    }
}

#[derive(Debug, Default)]
struct Game {
    outcome: Outcome,
    white_elo: Option<usize>,
    black_elo: Option<usize>,
    moves: Vec<String>,
    event: Option<String>,
}

impl Game {
    fn new() -> Self {
        unimplemented!()
    }
    fn new_from_str_vec(data: &[String]) -> Self {
        // TODO(austin): Take this out if slow
        let event = data[0].to_string();

        let outcome = Outcome::new_from_str(&data[1]);

        // TODO(austin): Take this out if slow
        let white_elo = data[2].parse::<usize>().unwrap();
        let black_elo = data[3].parse::<usize>().unwrap();

        // TODO(austin): Make this faster
        let moves: Vec<String> = data[4].split(' ').map(|s| s.to_string()).collect();

        Self {
            outcome,
            white_elo: Some(white_elo),
            black_elo: Some(black_elo),
            moves,
            event: Some(event),
        }
    }
}
