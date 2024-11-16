use serde::{Deserialize, Serialize};
use std::io::{self};
use std::{fs, process};

pub struct Character<'a> {
    turns_left: u32,
    agents_left: u32,
    food: u32,
    cash: u32,

    improvements: Vec<&'a Improvement>,
}

impl<'a> Character<'a> {
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

    pub fn purchase(&mut self, impr: &'a Improvement) {
        self.cash -= impr.cost.1;
        self.improvements.push(impr);
    }
}

type Effect = (String, u32);

// Create a wrapper for the sequence part of this
#[derive(Serialize, Deserialize, Debug)]
pub struct Improvement {
    pub name: String,
    pub description: String,
    pub cost: Effect,
    pub benefit: Effect,
}

pub fn load_improvements() -> Result<Vec<Improvement>, io::Error> {
    let contents = fs::read_to_string("./data/improvements.json")?;

    let results: Vec<Improvement> = serde_json::from_str(&contents)?;

    return Ok(results);
}
