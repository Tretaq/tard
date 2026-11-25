use rand::seq::SliceRandom;
#[derive(Debug, Clone)]
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
#[derive(Debug,Clone,PartialEq)]
pub enum DrawResult {
    Drawn,
    Burned,
    DeckEmpty,
}
pub const MAX_HAND: usize = 5;
impl Card {
    pub fn new(name: &str, cost: u32, effects: Effect) -> Card {
        Card{
            name: name.to_string(),
            cost,
            effects,
        }
    }
}

#[derive(Debug, Clone)]// derive poies and pastes functions
// debug tels rust how to pringt it (using {:?}) and Clone copies a variable and not just changes ownership
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
            hp: 20,
            shield: 0,
            mana: 0,
            max_mana: 0,
            deck: Vec::new(),
            hand: Vec::new(),
        }
    }
    pub fn shuffle_deck(&mut self) {// mut self lets you read and modify without taking ownership || also thats how u can tell a func what can it do so like were useing mut so we can change it
        let mut rng = rand::rng();
        self.deck.shuffle(&mut rng);
    }

    pub fn draw_card(&mut self) -> DrawResult{
        match  self.deck.pop() {
            None => DrawResult::DeckEmpty,
            Some(card) => {
                if self.hand.len() >= MAX_HAND {
                    DrawResult::Burned
                } else {
                    self.hand.push(card);
                    DrawResult::Drawn
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub players: Vec<Player>,
    pub current: usize,
    pub turn: u32,
}
impl GameState {
    pub fn new() -> GameState {
        GameState{
            players: vec![// makes it a macro so it runs at compile time and can allocate any num  WE USE MACRO SO THAT THE MAKES A FUNC WITH THAT MANY ARGUMENTSSSS IN COMPILER TIME
                Player::new("You"), Player::new("Enemy"),
            ],
            current: 0,
            turn:1,
        }
    }
}

pub fn card_library() -> Vec<Card> {
    vec![
        Card::new("Goblin Stab",1, Effect::Damage(2)),
        Card::new("Fireball", 4, Effect::Damage(6)),
        Card::new("Healing Spring",3,Effect::Heal(5)),
        Card::new("Stone Shield",2,Effect::Shield(4)),
        Card::new("Ember Curse", 3, Effect::Burn {damage: 2, turns:3}),
        Card::new("Scholar's Insight",2,Effect::Draw(2))
    ]
}
pub fn build_deck() -> Vec<Card>{
    let library = card_library();
    let mut deck = Vec::new();
    for i in 0..20 {
        deck.push(library[i % library.len()].clone());
    }
    deck
}

