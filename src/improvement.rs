use crate::character_record::Character;

pub struct Improvement {
    cost: u32,
    description: String,
    take_turn: fn(&mut Character) -> (),
}

impl Improvement {
    pub fn new(cost: u32, description: String, take_turn: fn(&mut Character) -> ()) -> Improvement {
        Improvement {
            cost,
            description,
            take_turn,
        }
    }
}

// fn tavern_take_turn(character: &mut Character) {
//     return ();
// }

pub fn get_improvements() -> [Improvement; 1] {
    [Improvement::new(
        123,
        String::from("Tavern: Resistance owned tavern that generates income"),
        |character| -> () {},
    )]
}
