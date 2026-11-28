mod model;
mod rules;

use model::{Card, Effect, GameState};
use rules::{apply, Action};
fn main() {
     let mut state = GameState::new();
    state.players[0].hand.push(Card::new("Fireball",4,Effect::Damage(6)));
    let last = state.players[0].hand.len() - 1;
    println!("mana: {}", state.players[0].mana);
    println!("{:?}", apply(&state, Action::PlayCard(99)).err());
    println!("{:?}",apply(&state, Action::PlayCard(last)).err());
    state.players[0].mana = 4;
    let next = apply(&state, Action::PlayCard(last)).unwrap();
    println!("enemy hp: {}", next.players[1].hp);
    println!("mana: {}", next.players[0].mana);
    println!(
        "old hand: {}, new hand: {}",
        state.players[0].hand.len(),
        next.players[0].hand.len()
    );
    let fresh = GameState::new();
    let t1 = apply(&fresh, Action::EndTurn).unwrap();
    println!(
        "after 1st EndTurn: current={}, turn={}, enemy mana={}",
        t1.current, t1.turn, t1.players[1].mana
    );
    let t2 = apply(&t1,Action::EndTurn).unwrap();
    println!(
        "after 2nd EndTurn: current={}, turn={}, enemy mana={}",
        t2.current, t2.turn, t2.players[0].mana
    );
}