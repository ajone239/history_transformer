// order of data
//
// event
// outcome
// White
// Black
// moves
pub enum States {
    Event,
    Outcome,
    WhiteElo,
    BlackElo,
    LeMoves,
}

impl States {
    pub fn value(&self) -> &str {
        match self {
            Self::Event => "E,",
            Self::Outcome => "O,",
            Self::WhiteElo => "W,",
            Self::BlackElo => "B,",
            Self::LeMoves => "L,",
        }
    }
    pub fn next(&self) -> Self {
        match self {
            Self::Event => Self::Outcome,
            Self::Outcome => Self::WhiteElo,
            Self::WhiteElo => Self::BlackElo,
            Self::BlackElo => Self::LeMoves,
            Self::LeMoves => Self::Event,
        }
    }
}
