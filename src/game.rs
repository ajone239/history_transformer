use crate::outcome::Outcome;

#[derive(Debug, Default)]
pub struct Game {
    outcome: Outcome,
    white_elo: Option<usize>,
    black_elo: Option<usize>,
    moves: Vec<String>,
    event: Option<String>,
}

impl Game {
    pub fn new() -> Self {
        unimplemented!()
    }
    pub fn new_from_str_vec(data: &[String]) -> Self {
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
