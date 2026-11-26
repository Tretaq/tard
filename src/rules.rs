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
        Action::PlayCard(index) => play_card(),
        Action::EndTurn => end_turn(),
    }
    Ok(next)
}


