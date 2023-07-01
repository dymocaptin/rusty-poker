use card::Card;
use rank::Rank;
use suit::Suit;

use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        cards.push(Card::new(Suit::Diamond, Rank::Ace));
        cards.push(Card::new(Suit::Diamond, Rank::Two));
        cards.push(Card::new(Suit::Diamond, Rank::Three));
        cards.push(Card::new(Suit::Diamond, Rank::Four));
        cards.push(Card::new(Suit::Diamond, Rank::Five));
        cards.push(Card::new(Suit::Diamond, Rank::Six));
        cards.push(Card::new(Suit::Diamond, Rank::Seven));
        cards.push(Card::new(Suit::Diamond, Rank::Eight));
        cards.push(Card::new(Suit::Diamond, Rank::Nine));
        cards.push(Card::new(Suit::Diamond, Rank::Ten));
        cards.push(Card::new(Suit::Diamond, Rank::Jack));
        cards.push(Card::new(Suit::Diamond, Rank::Queen));
        cards.push(Card::new(Suit::Diamond, Rank::King));
        cards.push(Card::new(Suit::Heart, Rank::Ace));
        cards.push(Card::new(Suit::Heart, Rank::Two));
        cards.push(Card::new(Suit::Heart, Rank::Three));
        cards.push(Card::new(Suit::Heart, Rank::Four));
        cards.push(Card::new(Suit::Heart, Rank::Five));
        cards.push(Card::new(Suit::Heart, Rank::Six));
        cards.push(Card::new(Suit::Heart, Rank::Seven));
        cards.push(Card::new(Suit::Heart, Rank::Eight));
        cards.push(Card::new(Suit::Heart, Rank::Nine));
        cards.push(Card::new(Suit::Heart, Rank::Ten));
        cards.push(Card::new(Suit::Heart, Rank::Jack));
        cards.push(Card::new(Suit::Heart, Rank::Queen));
        cards.push(Card::new(Suit::Heart, Rank::King));
        cards.push(Card::new(Suit::Spade, Rank::Ace));
        cards.push(Card::new(Suit::Spade, Rank::Two));
        cards.push(Card::new(Suit::Spade, Rank::Three));
        cards.push(Card::new(Suit::Spade, Rank::Four));
        cards.push(Card::new(Suit::Spade, Rank::Five));
        cards.push(Card::new(Suit::Spade, Rank::Six));
        cards.push(Card::new(Suit::Spade, Rank::Seven));
        cards.push(Card::new(Suit::Spade, Rank::Eight));
        cards.push(Card::new(Suit::Spade, Rank::Nine));
        cards.push(Card::new(Suit::Spade, Rank::Ten));
        cards.push(Card::new(Suit::Spade, Rank::Jack));
        cards.push(Card::new(Suit::Spade, Rank::Queen));
        cards.push(Card::new(Suit::Spade, Rank::King));
        cards.push(Card::new(Suit::Club, Rank::Ace));
        cards.push(Card::new(Suit::Club, Rank::Two));
        cards.push(Card::new(Suit::Club, Rank::Three));
        cards.push(Card::new(Suit::Club, Rank::Four));
        cards.push(Card::new(Suit::Club, Rank::Five));
        cards.push(Card::new(Suit::Club, Rank::Six));
        cards.push(Card::new(Suit::Club, Rank::Seven));
        cards.push(Card::new(Suit::Club, Rank::Eight));
        cards.push(Card::new(Suit::Club, Rank::Nine));
        cards.push(Card::new(Suit::Club, Rank::Ten));
        cards.push(Card::new(Suit::Club, Rank::Jack));
        cards.push(Card::new(Suit::Club, Rank::Queen));
        cards.push(Card::new(Suit::Club, Rank::King));
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }
}
