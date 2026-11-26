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
        Action::PlayCard(index) => play_card(&mut next, index),
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
    let cost = state.players[cur].hand.len();
    if(state.players[cur].mana < cost){

    }
}
fn end_turn(state: &mut next){

}