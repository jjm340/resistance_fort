use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self};
use std::iter::Filter;
use std::option::Iter;

use crate::utils::{log_and_exit, wrap_error};
use crate::CommonError;

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

static mut IS_INITIALIZED: bool = false;

pub struct ImprovementCollection {
    improvements: Vec<Improvement>,
}

impl ImprovementCollection {
    fn load_improvements() -> Result<Vec<Improvement>, io::Error> {
        let contents = fs::read_to_string("./data/improvements.json")?;

        let results: Vec<Improvement> = serde_json::from_str(&contents)?;

        return Ok(results);
    }

    pub fn filter_improvements(&self, search_input: &str) -> &[Improvement] {
        let normalized_search_input = search_input.trim().to_lowercase();

        let results: Vec<Improvement> = self
            .improvements
            .iter()
            .filter(|probe| {
                (**probe)
                    .name
                    .to_lowercase()
                    .contains(&normalized_search_input)
                    || (**probe)
                        .description
                        .to_lowercase()
                        .contains(&normalized_search_input)
            })
            .collect();

        &results[..]
    }

    pub fn all_improvements(&self) -> &[Improvement] {
        &self.improvements[..]
    }

    pub fn new() -> ImprovementCollection {
        unsafe {
            if IS_INITIALIZED {
                log_and_exit(Some(Box::new(CommonError::new(
                    "Do not try and initialize data twice".to_string(),
                ))))
            }
        }

        let loaded_improvements = ImprovementCollection::load_improvements();

        match loaded_improvements {
            Ok(improvements) => {
                unsafe {
                    IS_INITIALIZED = true;
                }

                return ImprovementCollection { improvements };
            }
            Err(e) => log_and_exit(wrap_error(e)),
        }
    }
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

                let granary = &results[0];
                let tavern = &results[1];

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
