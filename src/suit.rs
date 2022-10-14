#[derive(Debug, Eq, PartialEq)]
pub enum Suit {
    Diamonds,
    Hearts,
    Spades,
    Clubs,
}

impl ToString for Suit {
    fn to_string(&self) -> String {
        match self {
            Self::Diamonds => "♦".to_string(),
            Self::Hearts => "♥".to_string(),
            Self::Spades => "♠".to_string(),
            Self::Clubs => "♣".to_string(),
        }
    }
}
