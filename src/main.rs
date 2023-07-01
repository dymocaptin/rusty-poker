mod card;
mod deck;
mod player;
mod rank;
mod role;
mod suit;

extern crate rand;

use deck::Deck;
use player::{new, Player};
use rand::Rng;
use role::Role;

fn main() {
    // let's play a game of cards

    // create a deck of cards
    let mut d = Deck::new();
    for card in &d.cards {
        println!("card rank: {:?} card suit: {:?}", card.rank, card.suit);
    }
    // shuffle the deck
    d.shuffle();
    println!("The deck is shuffled! \n");
    for card in &d.cards {
        println!("card rank: {:?} card suit: {:?}", card.rank, card.suit);
    }

    // generate players at the table
    let mut players: Vec<Player> = Vec::new();
    players.push(new("bob"));
    players.push(new("sally"));
    players.push(new("dave"));
    players.push(new("paul"));

    // select a dealer
    pick_the_dealer(&mut players);

    // deal 5 cards from the deck at random to players

    // select a winner
}

fn pick_the_dealer(players: &mut Vec<Player>) {
    // pick a player at random to be the dealer
    let dealer = rand::thread_rng().gen_range(0..=players.len());
    let dealer = players.get_mut(dealer);
    // change the players role to Role::Dealer
    match dealer {
        None => (),
        Some(p) => p.role = Role::Dealer,
    };
}
