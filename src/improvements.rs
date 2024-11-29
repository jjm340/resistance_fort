use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self};
use std::sync::Mutex;

use crate::utils::{log_and_exit, wrap_error};
use crate::CommonError;

// TODO: Rename to EffectType
#[derive(Serialize, Deserialize, Debug)]
pub enum EffectCostType {
    Food,
    Cash,
}

type Effect = (EffectCostType, u32);

#[derive(Serialize, Deserialize, Debug)]
pub struct Improvement {
    pub name: String,
    pub description: String,
    pub cost: Effect,
    pub benefit: Effect,
}

static mut IS_INITIALIZED: Mutex<bool> = Mutex::new(false);

pub struct ImprovementCollection {
    improvements: Vec<Improvement>,
}

impl ImprovementCollection {
    fn load_improvements() -> Result<Vec<Improvement>, io::Error> {
        let contents = fs::read_to_string("./data/improvements.json")?;

        let results: Vec<Improvement> = serde_json::from_str(&contents)?;

        return Ok(results);
    }

    pub fn all_improvements(&self) -> Vec<&Improvement> {
        self.improvements.iter().collect()
    }

    pub fn new() -> ImprovementCollection {
        unsafe {
            let mut is_initialized = IS_INITIALIZED.lock().unwrap();
            if *is_initialized {
                log_and_exit(Some(Box::new(CommonError::new(
                    "Do not try and initialize data twice".to_string(),
                ))))
            }

            let loaded_improvements = ImprovementCollection::load_improvements();

            match loaded_improvements {
                Ok(improvements) => {
                    *is_initialized = true;
                    return ImprovementCollection { improvements };
                }
                Err(e) => log_and_exit(wrap_error(e)),
            }
        }
    }
}

pub fn filter_improvements<'a>(
    improvements: &Vec<&'a Improvement>,
    search_input: &str,
) -> Vec<&'a Improvement> {
    let normalized_search_input = search_input.trim().to_lowercase();

    let results = improvements
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
        .map(|item| *item)
        .collect();

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_improvements_parses_json_file_for_data() {
        let col = ImprovementCollection::new();

        let tavern = &col.improvements[0];
        let granary = &col.improvements[1];

        assert_eq!(tavern.name, "Tavern");
        assert_eq!(granary.name, "Granary");
    }

    #[test]
    fn filter_improvements_matches_partial_name() {
        let col = ImprovementCollection::new();

        let tavern = filter_improvements(&col.all_improvements(), "tav");

        assert_eq!(tavern[0].name, "Tavern");
    }
}
