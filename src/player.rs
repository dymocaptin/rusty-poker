use role::Role;
use card::Card;

pub struct Player<'a> {
    pub name: &'a str,
    pub role: Role,
    pub hand: Vec<Card>
}

pub fn new(name: &str) -> Player {
    Player {
        name,
        role: Role::Participant,
        hand: Vec::new()
    }
}