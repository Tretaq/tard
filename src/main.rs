mod model;

use model::{Card,Effect, GameState};
fn main() {
    let mut state = GameState::new();
    let fireball = Card::new("Fireball", 4, Effect::Damage(6));
    state.players[0].hand.push(fireball.clone());
    state.players[1].hand.push(fireball);
    println!("{:#?}", state);
}