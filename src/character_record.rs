use crate::{
    improvements::{EffectType, Improvement},
    CommonError,
};
use rand::Rng;

pub struct Character<'a> {
    turns_left: u32,
    agents_left: u32,
    food: u32,
    cash: u32,

    improvements: Vec<&'a Improvement>,
}

pub enum ResourceType {
    Food,
    Income,
}

impl<'a> Character<'a> {
    pub fn new() -> Character<'a> {
        Character {
            turns_left: 100,
            agents_left: 100,
            food: 100,
            cash: 100,
            improvements: Vec::new(),
        }
    }
    pub fn print_hud(&self) {
        println!(
            "Turns left: {}    Agents: {}    Food:{}    Cash: {}",
            self.turns_left, self.agents_left, self.food, self.cash
        )
    }

    pub fn gather_resouce(&mut self, resource_type: ResourceType) -> u32 {
        let mut rng = rand::thread_rng();
        let resouce_gathered = rng.gen_range(0..10);

        let action: Box<dyn FnOnce()> = match resource_type {
            ResourceType::Food => Box::new(|| -> () {
                self.food += resouce_gathered;
            }),
            ResourceType::Income => Box::new(|| -> () {
                self.cash += resouce_gathered;
            }),
        };

        action();

        return resouce_gathered;
    }

    pub fn purchase(&mut self, impr: &'a Improvement) -> Option<CommonError> {
        let (result, action): (i32, Box<dyn FnOnce() -> ()>) = match impr.cost.0 {
            EffectType::Cash => {
                let inner_result: i32 = (self.cash as i32) - (impr.cost.1 as i32);

                let action = Box::new(|| -> () {
                    self.cash -= impr.cost.1;
                });

                (inner_result, action)
            }
            EffectType::Food => {
                let inner_result: i32 = (self.food as i32) - (impr.cost.1 as i32);

                let action = Box::new(|| -> () {
                    self.food -= impr.cost.1;
                });

                (inner_result, action)
            }
        };

        if result >= 0 {
            action();

            self.improvements.push(impr);
            None
        } else {
            let err = CommonError::new("You do not have enough to make this purchase".to_string());
            Some(err)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn purchase_should_subtract_cost_add_improvement() {
        let mut character = Character::new();

        assert_eq!(character.cash, 100);

        let improvement = Improvement {
            name: String::from("Test improvement"),
            description: String::from("This describes the test improvement"),
            cost: (EffectType::Cash, 1),
            benefit: (EffectType::Food, 1),
        };

        match character.purchase(&improvement) {
            None => assert!(true),
            Some(e) => panic!("Purchase should not return an error: {}", e),
        }

        assert_eq!(character.cash, 99);

        assert_eq!(character.improvements[0].name, "Test improvement");
    }

    #[test]
    fn purchase_should_return_error_if_cost_too_high() {
        let mut character = Character::new();

        assert_eq!(character.cash, 100);

        let improvement = Improvement {
            name: String::from("Test improvement"),
            description: String::from("This describes the test improvement"),
            cost: (EffectType::Food, 101),
            benefit: (EffectType::Cash, 1),
        };

        match character.purchase(&improvement) {
            None => panic!("Should have thrown an error"),
            Some(e) => assert_eq!(
                e.message,
                "You do not have enough to make this purchase".to_string()
            ),
        }

        assert_eq!(character.cash, 100);
        assert_eq!(character.improvements.len(), 0);
    }
}
