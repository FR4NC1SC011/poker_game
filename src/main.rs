use crate::deck::*;
use crate::poker_hands::*;
use std::io;

mod card;
mod deck;
mod poker_hands;
mod tests;


#[macro_use]
extern crate clap;


fn main() {

    let _matches = clap_app!(poke_game =>
        (version: "1.0")
        (author: "Francisco O. <fcootzg@gmail.com>")
        (about: "Simple Poker Game")
        ).get_matches();

    let mut deck: Deck = Deck::new();

    loop {
        print!("{esc}c", esc = 27 as char);     // Clean Screen

        // We need min. 8 cards in the deck to keep playing.
        // 5 cards in the hand and 3 to change
        if deck.hand.len() < 8 {
            println!("No more cards");
        }


        let mut hand: Deck = Deck::new_hand(&mut deck);

        let mut i = 0;
        for x in &hand.hand {
            print!("{}. ", i);
            x.print_card();
            i += 1;
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
        println!("Keep Playing? yes/no");
        let mut play = String::new();
        io::stdin()
            .read_line(&mut play)
            .expect("Failed to read input");

        if play == "no\n" {
            break;
        }
    }
}
