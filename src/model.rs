#[derive(Debug, Clone)] //
pub enum Effect {
    Damage(i32),
    Heal(i32),
    Shield(i32),
    Draw(u32),
    Burn {damage: i32, turns: u32}, // i32 is signed and u32 is unsigned
}
#[derive(Debug, Clone)]
pub struct Card {
    pub name: String,
    pub cost: u32,
    pub effects: Effect,
}
impl Card {
    pub fn new(name: String, cost: u32, effects: Effect) -> Card {
        Card{
            name: name.to_string(),
            cost,
            effects,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub hp: i32,
    pub shield: i32,
    pub mana: u32,
    pub max_mana: u32,
    pub deck: Vec<Card>,
    pub hand: Vec<Card>,
}

impl Player {
    pub fn new(name: &str) -> Player{
        Player{
            name: name.to_string(),
            hp: 2,
            shield: 0,
            mana: 0,
            max_mana: 0,
            deck: Vec::new(),
            hand: Vec::new(),
        }
    }
}