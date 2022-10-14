use crate::face::Face;
use crate::suit::Suit;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Card {
    suit: Suit,
    face: Face,
}

impl Card {
    pub const fn new(suit: Suit, face: Face) -> Self {
        Self { suit, face }
    }

    pub fn suit(&self) -> &Suit {
        &self.suit
    }

    pub fn face(&self) -> &Face {
        &self.face
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match (&self.suit, &self.face) {
            (Suit::Diamonds, Face::Ace) => write!(f, "🃁"),
            (Suit::Diamonds, Face::Two) => write!(f, "🃂"),
            (Suit::Diamonds, Face::Three) => write!(f, "🃃"),
            (Suit::Diamonds, Face::Four) => write!(f, "🃄"),
            (Suit::Diamonds, Face::Five) => write!(f, "🃅"),
            (Suit::Diamonds, Face::Six) => write!(f, "🃆"),
            (Suit::Diamonds, Face::Seven) => write!(f, "🃇"),
            (Suit::Diamonds, Face::Eight) => write!(f, "🃈"),
            (Suit::Diamonds, Face::Nine) => write!(f, "🃉"),
            (Suit::Diamonds, Face::Ten) => write!(f, "🃊"),
            (Suit::Diamonds, Face::Jack) => write!(f, "🃋"),
            (Suit::Diamonds, Face::Knight) => write!(f, "🃌"),
            (Suit::Diamonds, Face::Queen) => write!(f, "🃍"),
            (Suit::Diamonds, Face::King) => write!(f, "🃎"),
            (Suit::Hearts, Face::Ace) => write!(f, "🂱"),
            (Suit::Hearts, Face::Two) => write!(f, "🂲"),
            (Suit::Hearts, Face::Three) => write!(f, "🂳"),
            (Suit::Hearts, Face::Four) => write!(f, "🂴"),
            (Suit::Hearts, Face::Five) => write!(f, "🂵"),
            (Suit::Hearts, Face::Six) => write!(f, "🂶"),
            (Suit::Hearts, Face::Seven) => write!(f, "🂷"),
            (Suit::Hearts, Face::Eight) => write!(f, "🂸"),
            (Suit::Hearts, Face::Nine) => write!(f, "🂹"),
            (Suit::Hearts, Face::Ten) => write!(f, "🂺"),
            (Suit::Hearts, Face::Jack) => write!(f, "🂻"),
            (Suit::Hearts, Face::Knight) => write!(f, "🂼"),
            (Suit::Hearts, Face::Queen) => write!(f, "🂽"),
            (Suit::Hearts, Face::King) => write!(f, "🂾"),
            (Suit::Spades, Face::Ace) => write!(f, "🂡"),
            (Suit::Spades, Face::Two) => write!(f, "🂢"),
            (Suit::Spades, Face::Three) => write!(f, "🂣"),
            (Suit::Spades, Face::Four) => write!(f, "🂤"),
            (Suit::Spades, Face::Five) => write!(f, "🂥"),
            (Suit::Spades, Face::Six) => write!(f, "🂦"),
            (Suit::Spades, Face::Seven) => write!(f, "🂧"),
            (Suit::Spades, Face::Eight) => write!(f, "🂨"),
            (Suit::Spades, Face::Nine) => write!(f, "🂩"),
            (Suit::Spades, Face::Ten) => write!(f, "🂪"),
            (Suit::Spades, Face::Jack) => write!(f, "🂫"),
            (Suit::Spades, Face::Knight) => write!(f, "🂬"),
            (Suit::Spades, Face::Queen) => write!(f, "🂭"),
            (Suit::Spades, Face::King) => write!(f, "🂮"),
            (Suit::Clubs, Face::Ace) => write!(f, "🃑"),
            (Suit::Clubs, Face::Two) => write!(f, "🃒"),
            (Suit::Clubs, Face::Three) => write!(f, "🃓"),
            (Suit::Clubs, Face::Four) => write!(f, "🃔"),
            (Suit::Clubs, Face::Five) => write!(f, "🃕"),
            (Suit::Clubs, Face::Six) => write!(f, "🃖"),
            (Suit::Clubs, Face::Seven) => write!(f, "🃗"),
            (Suit::Clubs, Face::Eight) => write!(f, "🃘"),
            (Suit::Clubs, Face::Nine) => write!(f, "🃙"),
            (Suit::Clubs, Face::Ten) => write!(f, "🃚"),
            (Suit::Clubs, Face::Jack) => write!(f, "🃛"),
            (Suit::Clubs, Face::Knight) => write!(f, "🃜"),
            (Suit::Clubs, Face::Queen) => write!(f, "🃝"),
            (Suit::Clubs, Face::King) => write!(f, "🃞"),
        }
    }
}
