use rank::Rank;
use suit::Suit;

#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub const fn new(suit: Suit, rank: Rank) -> Card {
        Card { suit, rank }
    }
}
