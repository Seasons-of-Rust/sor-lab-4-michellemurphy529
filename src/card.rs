use std::fmt;

use crate::FightResult;

/// A Card is a card stores a price, health, and damage.
pub struct Card {
    pub price: u32,
    pub health: u32,
    pub damage: u32,
}

impl Card {
    pub fn fight(&self, other: &Card) -> FightResult {

        let card_one = self.damage - other.health;
        let card_two = other.damage - self.health;
    
        let mut fight = -1;

        if card_one <= 0 && card_two <= 0 {
            fight = 0;
        }
        else if card_one <= 0 && card_two > 0 {
            fight = 1;
        }
        else if card_one > 0 && card_two <= 0 {
            fight = 2;
        }
        else if card_one > 0 && card_two > 0 {
            fight = 3;
        }
        
        match fight{
            
            // If both cards deal enough damage to kill one another, return a tie
            0 => FightResult::Tie,
            // If this card deals enough damage to kill the other card, return a win
            1 => FightResult::Win,
            // If the other card deals enough damage to kill this card, return a loss
            2 => FightResult::Loss,
            // If neither card deals enough damage to kill the other, return a draw
            _ => FightResult::Draw,
        }
    }

    /// Give a play by play of the battle
    pub fn print_fight(&self, other: &Card) -> FightResult {
        println!("\n{} vs {}", &self, other);
        println!("🗡️ 🗡️ 🗡️");

        let fight_result = self.fight(other);

        match fight_result {
            FightResult::Win => println!("{} wins!", self),
            FightResult::Loss => println!("{} wins!", other),
            FightResult::Tie => println!("It's a tie!"),
            FightResult::Draw => println!("It's a draw!"),
        }

        println!();

        fight_result
    }
}

/// Implement the Display trait for Card so that it can be printed. It will
/// print in the form:
///
/// |Card: dmg/hp|
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "|Card: {}/{}|", self.damage, self.health)
    }
}
