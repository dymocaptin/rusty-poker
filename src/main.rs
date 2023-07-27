mod card;
mod deck;
mod player;
mod rank;
mod role;
mod suit;

extern crate clap;
extern crate names;
extern crate rand;

use clap::Parser;
use deck::Deck;
use names::Generator;
use player::{new, Player};
use rand::Rng;
use role::Role;

#[derive(Parser)]
struct Cli {
    number_of_players: usize,
}

fn main() {
    // let's play a game of cards!
    let args = Cli::parse();
    start_game_of_poker(args);
}

fn start_game_of_poker(arguments: Cli) {
    let number_of_players = arguments.number_of_players;
    let players = generate_players(number_of_players.clone());
    // select a dealer
    pick_the_dealer(players);

    // create a deck of cards
    let mut d = Deck::new_standard_deck();

    // shuffle the deck
    d.shuffle();
    println!("\nThe deck is shuffled! \n");
    for card in &d.cards {
        println!("card rank: {:?} card suit: {:?}", card.rank, card.suit);
    }

    // deal 5 cards from the deck at random to players

    // select a winner
}

fn generate_players(number_of_players: usize) -> Vec<Player> {
    // generate players at the table
    let mut players: Vec<Player> = Vec::new();
    let mut generator = Generator::default();
    for _n in 1..number_of_players {
        let name = match generator.next() {
            None => String::from("bob"),
            Some(p) => String::from(p),
        };
        players.push(new(name));
    }
    players
}

// Select a dealer at random, and then return a ref to that Player
fn pick_the_dealer(mut players: Vec<Player>) {
    // pick a player at random to be the dealer
    let dealer_index = rand::thread_rng().gen_range(0..=players.len());
    let dealer = players.get_mut(dealer_index);
    // change the players role to Role::Dealer
    match dealer {
        None => (),
        Some(p) => {
            p.role = Role::Dealer;
            println!("{:?} is dealer!\n", p.name)
        }
    };
}
