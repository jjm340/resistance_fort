use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::OnceLock;

#[derive(Serialize, Deserialize, Debug)]
pub enum EffectType {
    Food,
    Cash,
}

type Effect = (EffectType, u32);

#[derive(Serialize, Deserialize, Debug)]
pub struct Improvement {
    pub name: String,
    pub description: String,
    pub cost: Effect,
    pub benefit: Effect,
}

static IMPROVEMENTS: OnceLock<Vec<Improvement>> = OnceLock::new();

pub fn all_improvements<'a>() -> &'a Vec<Improvement> {
    IMPROVEMENTS.get_or_init(|| {
        let contents = fs::read_to_string("./data/improvements.json").unwrap();
        let results: Vec<Improvement> = serde_json::from_str(&contents).unwrap();

        results
    })
}

pub fn filter_improvements<'a>(search_input: &str) -> Vec<&'a Improvement> {
    let normalized_search_input = search_input.trim().to_lowercase();

    let results: Vec<&Improvement> = all_improvements()
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
    fn all_improvements_initializes_data() {
        let col = all_improvements();

        let tavern = &col[0];
        let granary = &col[1];

        assert_eq!(tavern.name, "Tavern");
        assert_eq!(granary.name, "Granary");
    }

    #[test]
    fn filter_improvements_matches_partial_name() {
        let tavern = filter_improvements("tav");

        assert_eq!(tavern[0].name, "Tavern");
    }
}
