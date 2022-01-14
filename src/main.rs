use crate::deck::*;
use crate::poker_hands::*;

mod card;
mod deck;
mod poker_hands;
mod tests;


fn main() {

    let mut deck: Deck = Deck::new();
    let mut hand: Deck = Deck::new_hand(&mut deck);

    for x in &hand.hand {
        x.print_card();
    }

    hand = Deck::change_cards(hand, &mut deck);

    for x in &hand.hand {
        x.print_card();
    }

    let mut points = PokerHands::n_of_a_kind(&hand);
    if points == 0 {
        points = PokerHands::is_straight(&mut hand);
    }

    println!("Total Points: {}", points);



}
