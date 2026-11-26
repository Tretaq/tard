mod model;
mod rules;

use model::GameState;
fn main() {
    let mut state = GameState::new();
    println!(
        "Hand: {:?}",
        state.players[0].hand.iter().map(|c| &c.name).collect::<Vec<_>>()
    );
    println!(
        "Deck size: {}",state.players[0].deck.len()
    );
    for _ in 0..20{
        let result = state.players[0].draw_card();
        println!("{:?}", result);
    }
}