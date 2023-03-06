use std::io::prelude::*;

use std::sync::{Arc, Mutex};
use std::{error::Error, fs::File, io::BufReader, sync::mpsc, thread};

use history_transformer::{game::Game, states::States};

// use chess::{Board, ChessMove};

const WORKER_COUNT: usize = 1;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("data/for_austin.txt")?;
    let lines = BufReader::new(f).lines();

    let (tx, rx) = mpsc::channel::<Vec<String>>();

    let rx = Arc::new(Mutex::new(rx));

    let mut handles = vec![];

    for i in 0..WORKER_COUNT {
        let rx = rx.clone();
        let handle = thread::spawn(move || game_data_handler(i, rx));

        handles.push(handle);
    }

    let mut game_data = vec![];
    let mut state = States::Event;

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

    drop(tx);

    for h in handles {
        h.join().unwrap();
    }

    Ok(())
}

type Data = Vec<String>;
type LockedChannel = Arc<Mutex<mpsc::Receiver<Data>>>;

fn game_data_handler(id: usize, rx: LockedChannel) {
    loop {
        let game_data = match rx.lock().unwrap().recv() {
            Ok(data) => data,
            Err(_) => break,
        };
        let game = Game::new_from_str_vec(&game_data[..]);

        println!("{id}: {game:?}");
    }
}
