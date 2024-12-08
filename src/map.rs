use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationExit {
    pub direction: String,
    pub description: String,
    pub destination: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub name: String,
    pub description: String,
    pub exits: Vec<LocationExit>,
}

pub fn all_locations() -> HashMap<String, Location> {
    let contents = fs::read_to_string("./data/locations.json").unwrap();
    let results: HashMap<String, Location> = serde_json::from_str(&contents).unwrap();
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_locations_initializes_data() {
        let location_map = all_locations();
        println!("col: {:?}", location_map);

        let cameriel = &location_map["1"];
        let darkrend = &location_map["2"];
        let tubeliam = &location_map["3"];
        let the_hub = &location_map["4"];
        let nether = &location_map["5"];

        assert_eq!(cameriel.name, "Cameriel");

        // let tavern = &col[0];
        // let granary = &col[1];

        // assert_eq!(tavern.name, "Tavern");
        // assert_eq!(granary.name, "Granary");
    }
}
