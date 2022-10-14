#[derive(Debug, Eq, PartialEq)]
pub enum Face {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Knight,
    Queen,
    King,
}

impl ToString for Face {
    fn to_string(&self) -> String {
        match self {
            Self::Ace => "A".to_string(),
            Self::Two => "2".to_string(),
            Self::Three => "3".to_string(),
            Self::Four => "4".to_string(),
            Self::Five => "5".to_string(),
            Self::Six => "6".to_string(),
            Self::Seven => "7".to_string(),
            Self::Eight => "8".to_string(),
            Self::Nine => "9".to_string(),
            Self::Ten => "10".to_string(),
            Self::Jack => "J".to_string(),
            Self::Knight => "C".to_string(),
            Self::Queen => "Q".to_string(),
            Self::King => "K".to_string(),
        }
    }
}