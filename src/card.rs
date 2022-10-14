use crate::face::Face;
use crate::suit::Suit;

#[derive(Debug, Eq, PartialEq)]
pub struct Card {
    suit: Suit,
    face: Face,
}

impl Card {
    pub fn new(suit: Suit, face: Face) -> Self {
        Self { suit, face }
    }

    pub fn suit(&self) -> &Suit {
        &self.suit
    }

    pub fn face(&self) -> &Face {
        &self.face
    }
}

impl ToString for Card {
    fn to_string(&self) -> String {
        match (&self.suit, &self.face) {
            (Suit::Diamonds, Face::Ace) => "🃁".to_string(),
            (Suit::Diamonds, Face::Two) => "🃂".to_string(),
            (Suit::Diamonds, Face::Three) => "🃃".to_string(),
            (Suit::Diamonds, Face::Four) => "🃄".to_string(),
            (Suit::Diamonds, Face::Five) => "🃅".to_string(),
            (Suit::Diamonds, Face::Six) => "🃆".to_string(),
            (Suit::Diamonds, Face::Seven) => "🃇".to_string(),
            (Suit::Diamonds, Face::Eight) => "🃈".to_string(),
            (Suit::Diamonds, Face::Nine) => "🃉".to_string(),
            (Suit::Diamonds, Face::Ten) => "🃊".to_string(),
            (Suit::Diamonds, Face::Jack) => "🃋".to_string(),
            (Suit::Diamonds, Face::Knight) => "🃌".to_string(),
            (Suit::Diamonds, Face::Queen) => "🃍".to_string(),
            (Suit::Diamonds, Face::King) => "🃎".to_string(),
            (Suit::Hearts, Face::Ace) => "🂱".to_string(),
            (Suit::Hearts, Face::Two) => "🂲".to_string(),
            (Suit::Hearts, Face::Three) => "🂳".to_string(),
            (Suit::Hearts, Face::Four) => "🂴".to_string(),
            (Suit::Hearts, Face::Five) => "🂵".to_string(),
            (Suit::Hearts, Face::Six) => "🂶".to_string(),
            (Suit::Hearts, Face::Seven) => "🂷".to_string(),
            (Suit::Hearts, Face::Eight) => "🂸".to_string(),
            (Suit::Hearts, Face::Nine) => "🂹".to_string(),
            (Suit::Hearts, Face::Ten) => "🂺".to_string(),
            (Suit::Hearts, Face::Jack) => "🂻".to_string(),
            (Suit::Hearts, Face::Knight) => "🂼".to_string(),
            (Suit::Hearts, Face::Queen) => "🂽".to_string(),
            (Suit::Hearts, Face::King) => "🂾".to_string(),
            (Suit::Spades, Face::Ace) => "🂡".to_string(),
            (Suit::Spades, Face::Two) => "🂢".to_string(),
            (Suit::Spades, Face::Three) => "🂣".to_string(),
            (Suit::Spades, Face::Four) => "🂤".to_string(),
            (Suit::Spades, Face::Five) => "🂥".to_string(),
            (Suit::Spades, Face::Six) => "🂦".to_string(),
            (Suit::Spades, Face::Seven) => "🂧".to_string(),
            (Suit::Spades, Face::Eight) => "🂨".to_string(),
            (Suit::Spades, Face::Nine) => "🂩".to_string(),
            (Suit::Spades, Face::Ten) => "🂪".to_string(),
            (Suit::Spades, Face::Jack) => "🂫".to_string(),
            (Suit::Spades, Face::Knight) => "🂬".to_string(),
            (Suit::Spades, Face::Queen) => "🂭".to_string(),
            (Suit::Spades, Face::King) => "🂮".to_string(),
            (Suit::Clubs, Face::Ace) => "🃑".to_string(),
            (Suit::Clubs, Face::Two) => "🃒".to_string(),
            (Suit::Clubs, Face::Three) => "🃓".to_string(),
            (Suit::Clubs, Face::Four) => "🃔".to_string(),
            (Suit::Clubs, Face::Five) => "🃕".to_string(),
            (Suit::Clubs, Face::Six) => "🃖".to_string(),
            (Suit::Clubs, Face::Seven) => "🃗".to_string(),
            (Suit::Clubs, Face::Eight) => "🃘".to_string(),
            (Suit::Clubs, Face::Nine) => "🃙".to_string(),
            (Suit::Clubs, Face::Ten) => "🃚".to_string(),
            (Suit::Clubs, Face::Jack) => "🃛".to_string(),
            (Suit::Clubs, Face::Knight) => "🃜".to_string(),
            (Suit::Clubs, Face::Queen) => "🃝".to_string(),
            (Suit::Clubs, Face::King) => "🃞".to_string(),
        }
    }
}
