use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self};
use std::sync::Mutex;

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

    pub fn all_improvements(&self) -> &[Improvement] {
        &self.improvements[..]
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
    improvements: &'a [Improvement],
    search_input: &str,
) -> Vec<&'a Improvement> {
    let normalized_search_input = search_input.trim().to_lowercase();

    let results: Vec<&Improvement> = improvements
        .iter()
        .filter(|probe| {
            (*probe)
                .name
                .to_lowercase()
                .contains(&normalized_search_input)
                || (*probe)
                    .description
                    .to_lowercase()
                    .contains(&normalized_search_input)
        })
        .collect();

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_improvements_parses_json_file_for_data() {
        let col = ImprovementCollection::new();

        let granary = &col.improvements[0];
        let tavern = &col.improvements[1];

        assert_eq!(granary.name, "Granary");
        assert_eq!(tavern.name, "Tavern");
    }
}
