#[derive(Debug, Default)]
pub enum Outcome {
    Black,
    White,
    #[default]
    Draw,
}

impl Outcome {
    pub fn new_from_str(s: &str) -> Self {
        match s {
            "1-0" => Self::White,
            "0-1" => Self::Black,
            _ => Self::Draw,
        }
    }
}
