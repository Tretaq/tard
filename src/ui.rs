use std::io::{self, Write};
use crate::model::{Effect, GameState};
use crate::rules::Action;

fn describe_effect(effect: &Effect) -> String {
    match effect {
        Effect::Damage(n) => format!("deal {} damage", n),
        Effect::Heal(n) => format!("heal {} health", n),
        Effect::Draw(n) => format!("draw {} cards",n),
        Effect::Shield(n) => format!("block the next {} damage", n),
        Effect::Burn { damage, turns} => { format!("burn for {} damage each turn, for {} turns",damage, turns)}
    }
}
pub fn render(state: &GameState){
    let me = &state.players[state.current];
    let opp = &state.players[1 - state.current];
    println!("\n === Turn {} === {}: {} HP (shield{}) | {}: {} HP (shield {})", state.turn, me.name , me.hp , me.shield, opp.name,opp.hp, opp.shield);
    println!("Mana: {}/{}", me.mana,me.max_mana);
    for(i,card) in me.hand.iter().enumerate() {
        println!(" {}) {} ({}) - {}", i+1, card.name, card.cost, describe_effect(&card.effects));
    }
}