use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self};

use crate::utils::log_and_exit;

#[derive(Serialize, Deserialize, Debug)]
pub enum EffectCostType {
    Food,
    Cash,
}

// TODO: Rename to EffectType
type Effect = (EffectCostType, u32);

#[derive(Serialize, Deserialize, Debug)]
pub struct Improvement {
    pub name: String,
    pub description: String,
    pub cost: Effect,
    pub benefit: Effect,
}

lazy_static! {
    pub static ref IMPROVEMENTS: HashMap<String, Improvement> = {
        let results = load_improvements();

        match results {
            Ok(results) => results,
            Err(e) => log_and_exit(Some(Box::new(e))),
        }
    };
}

fn load_improvements() -> Result<HashMap<String, Improvement>, io::Error> {
    let contents = fs::read_to_string("./data/improvements.json")?;

    let results: HashMap<String, Improvement> = serde_json::from_str(&contents)?;

    return Ok(results);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_improvements_parses_json_file_for_data() {
        let results = load_improvements();

        match results {
            Ok(results) => {
                println!("results: {:?}", results);

                let granary = results.get("granary").unwrap();
                let tavern = results.get("tavern").unwrap();

                assert_eq!(granary.name, "Granary");
                assert_eq!(tavern.name, "Tavern");
            }
            _ => {
                println!("{:?}", results);
                panic!("Should return results")
            }
        }
    }
}
