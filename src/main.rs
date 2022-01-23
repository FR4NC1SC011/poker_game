mod card;
mod deck;
mod poker_hands;

mod utils;

mod poker_hands_tests;
mod deck_tests;

use crate::deck::*;
use crate::poker_hands::*;
use crate::utils::*;

#[macro_use]
extern crate clap;

fn main() {
    let _matches = clap_app!(poke_game =>
        (version: "1.0")
        (author: "Francisco O. <fcootzg@gmail.com>")
        (about: "Simple Poker Game")
        ).get_matches();

    // create a new DB with AutoDum, meaning every change is written to the file,
    // and with Json serialization
    let mut db = load_or_new_db("local.db");

    // Declare the Score
    let mut score: u16 = 0;

    // Give the money to the user
    let mut money: u32 = 1000;

    // Create a new deck of 52 cards (no joker)
    let mut deck: Deck = Deck::new();


//---------------------------------------------------------------------------------------------
//          GAME LOOP
//---------------------------------------------------------------------------------------------
    loop {
        // Clear the Terminal Screen
        print!("{esc}c", esc = 27 as char);

        // We need min. 8 cards in the deck to keep playing.
        // 5 cards in the hand and 3 to change
        if deck.cards.len() < 8 {
            println!("No more cards");
            println!("Creating new Deck...");
            deck = Deck::new();
        }

        println!("HighScore: {}", db.get::<u16>("HighScore").unwrap());
        println!("Score: {}", score);
        println!("Money: {}", money);

        // Get Bet from the User
        let mut money_bet = money_bet(money);

        // Subtract the bet from the money
        money = money - money_bet;

        println!("Money: {}", money);

        // Create the user hand (5 cards)
        let mut hand: Deck = Deck::new_hand(&mut deck);

        Deck::print_hand(&hand);

        // Ask the user which cards he wants to change
        let cards_to_change: Vec<i32> = get_vec_input("Index of the Cards to Change: (Separated by Space eg: 1 2)");

        // How many cards the user wants to change
        let n_cards_to_change: u32 = cards_to_change.len() as u32;

        // Change the cards of the user hand
        hand = Deck::change_cards(hand, &mut deck, cards_to_change);

        // The user needs to bet 50 extra for each changed card
        // Subtract the additional bet from the money
        money = money - (n_cards_to_change * 50);

        // Add to the money bet the additional bet (The bet from the changed cards)
        money_bet = money_bet + (n_cards_to_change * 50);

        println!("Total Bet is {}", money_bet);
        println!("Money: {}", money);

        Deck::print_hand(&hand);

        // Check for Straight, Flush or Royal FLush
        let mut points = PokerHands::check_for_straight_flush_royalFlush(&mut hand);

        // If it's not Straight, Flush or Royal FLush Then
        // Check for N of a kind (Pair, Three of a kind, Poker) or Full House
        if points == 0 {
            points = PokerHands::check_for_n_of_a_kind_fullHouse(&hand);
        }

        // Calculate how much money the user the user earned
        let percentage_points: u32 = (points * 10) as u32;
        if percentage_points > 0 {
            money = money + (money_bet/10) * percentage_points;
        }

        // Update Score
        score += points;

        println!("Points of the Hand: {}", points);

        // Update Highscore
        if score > db.get::<u16>("HighScore").unwrap() {
            db.set("HighScore", &score).unwrap();
        }

        // Ask the user if wants to keep playing
        //let play: String = read!("{}\n");
        let play: String = get_input("Keep Playing? [Yes]/no");
        if play == "no" {
            println!("Thanks for playing");
            break;
        }
//---------------------------------------------------------------------------------------------
//          END GAME LOOP
//---------------------------------------------------------------------------------------------
    }

}


