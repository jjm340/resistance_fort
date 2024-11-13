use crate::improvement;

pub struct Character {
    turns_left: u32,
    agents_left: u32,
    food: u32,
    cash: u32,

    improvements: Vec<improvement::Improvement>,
}

impl Character {
    pub fn new() -> Character {
        Character {
            turns_left: 100,
            agents_left: 100,
            food: 100,
            cash: 100,
            improvements: Vec::new(),
        }
    }
    pub fn take_turn(&mut self) {}
    pub fn print_hud(&self) {
        println!(
            "Turns left: {}    Agents: {}    Food:{}    Cash: {}",
            self.turns_left, self.agents_left, self.food, self.cash
        )
    }
}
