use std::fmt;
use crate::model::{Effect, GameState};

#[derive(Debug,Clone)]
pub enum Action{
    PlayCard(usize),
    EndTurn,
}

#[derive(Debug, Clone)]
pub enum GameError {
    InvalidCardIndex,
    NotEnoughMana {needed: u32, have: u32},
    GameOver,
}

pub fn apply(state: &GameState, action: Action) -> Result<GameState, GameError>{
    if state.winner.is_some(){
        return Err(GameError::GameOver);
    }
    let mut next = state.clone();
    match action {
        Action::PlayCard(index) => play_card(&mut next, index)?,
        Action::EndTurn => end_turn(&mut next),
    }
    Ok(next)
}

fn play_card(state: &mut GameState, index: usize) -> Result<(), GameError> {
    let cur = state.current;
    let opp = 1 - cur;
    if(index >= state.players[cur].hand.len()){
        return Err(GameError::InvalidCardIndex)
    }
    let cost = state.players[cur].hand[index].cost;
    if(state.players[cur].mana < cost){
        return Err(GameError::NotEnoughMana {needed: cost, have: state.players[cur].mana})
    }
    let card = state.players[cur].hand.remove(index);
    state.players[cur].mana -= cost;
    match card.effects{
        Effect::Damage(amount) => {state.players[opp].hp -= amount;}, _ => {}
    }
    if(state.players[opp].hp <= 0){
        state.winner = Some(cur);
    }
    Ok(())
}
fn end_turn(state: &mut GameState){
    state.current = 1 - state.current;
    if state.current == 0 {
        state.turn += 1;
    }
    state.start_turn();
}
impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
    }
}