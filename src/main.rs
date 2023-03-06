use std::collections::HashMap;
use std::io::prelude::*;

use std::sync::{Arc, Mutex};
use std::{error::Error, fs::File, io::BufReader, sync::mpsc, thread};

use history_transformer::{game::Game, outcome::Outcome, states::States};

use chess::{Board, ChessMove};

const WORKER_COUNT: usize = 4;

fn main() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = mpsc::channel::<Vec<String>>();

    let rx = Arc::new(Mutex::new(rx));

    let mut handles = vec![];
    let mut standings = vec![];

    for i in 0..WORKER_COUNT {
        let position_wld = Arc::new(Mutex::new(HashMap::new()));

        let handle = thread::spawn({
            let rx = rx.clone();
            let position_wld = position_wld.clone();
            move || game_data_handler(i, rx, position_wld)
        });

        handles.push(handle);
        standings.push(position_wld);
    }

    let mut game_data = vec![];
    let mut state = States::Event;

    // Read in the data and send it to the workers
    let f = File::open("data/for_austin.txt")?;
    let lines = BufReader::new(f).lines();
    for line in lines {
        let line = line?;

        // check the input code
        if &line[..2] == state.value() {
            game_data.push(line[2..].to_string());
            state = state.next();
        } else {
            game_data.clear();
            state = States::Event;
            continue;
        }

        // We have all the data
        if game_data.len() >= 5 {
            tx.send(game_data.clone())?;
            game_data.clear();
            state = States::Event
        }
    }

    // Drop the sender stub so the recv channel will die
    drop(tx);

    for h in handles {
        h.join().unwrap();
    }

    // Zip the results
    let final_standings = {
        let mut position_wld: HashMap<Board, [i32; 3]> = HashMap::new();

        for s in standings {
            let s = s.lock().unwrap();
            for (k, v) in s.iter() {
                let a = position_wld.entry(*k).or_insert([0; 3]);
                for (i, j) in a.iter_mut().zip(v.iter()) {
                    *i += j;
                }
            }
        }

        position_wld
    };

    // Print
    for (k, v) in final_standings.iter() {
        let key_str = k.to_string();
        let key_str: String = key_str.split(' ').take(1).collect();
        println!("{key_str}: {v:?}");
    }

    Ok(())
}

type Data = Vec<String>;
type LockedChannel = Arc<Mutex<mpsc::Receiver<Data>>>;
type LockedResult = Arc<Mutex<HashMap<Board, [i32; 3]>>>;

fn game_data_handler(_id: usize, rx: LockedChannel, position_wld: LockedResult) {
    let mut position_wld = position_wld.lock().unwrap();
    loop {
        let game_data = match rx.lock().unwrap().recv() {
            Ok(data) => data,
            Err(_) => break,
        };
        let game = Game::new_from_str_vec(&game_data[..]);

        let mut board = Board::default();

        for m in game.moves {
            let player_move = ChessMove::from_san(&board, &m).unwrap();

            let new_board = board.make_move_new(player_move);
            board = new_board;

            let standings = position_wld.entry(board).or_insert([0, 0, 0]);
            match game.outcome {
                Outcome::White => standings[0] += 1,
                Outcome::Black => standings[1] += 1,
                Outcome::Draw => standings[2] += 1,
            }
        }
    }
}
