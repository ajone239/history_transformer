use std::io::prelude::*;

use std::{error::Error, fs::File, io::BufReader, sync::mpsc, thread};

use history_transformer::{game::Game, states::States};

// use chess::{Board, ChessMove};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("data/for_austin.txt")?;
    let lines = BufReader::new(f).lines();

    let (tx, rx) = mpsc::channel::<Vec<String>>();

    let handle = thread::spawn(move || {
        while let Ok(game_data) = rx.recv() {
            let game = Game::new_from_str_vec(&game_data[..]);

            println!("{game:#?}");
        }
    });

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

    handle.join().unwrap();

    Ok(())
}
