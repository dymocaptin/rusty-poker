use card::Card;
use role::Role;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub role: Role,
    pub hand: Vec<Card>,
}

pub fn new(name: String) -> Player {
    Player {
        name,
        role: Role::Participant,
        hand: Vec::new(),
    }
}
