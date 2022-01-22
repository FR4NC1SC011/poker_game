use crate::deck::*;
use crate::poker_hands::*;
//use std::io;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use std::io::{self, BufRead};
use std::process;

mod card;
mod deck;
mod poker_hands;
mod poker_hands_tests;


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

    let mut score: u16 = 0;

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
            // Create a new deck
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

        println!("Cards to change: ");

        // Ask the user which cards he wants to change
        let cards_to_change: Vec<i32> = get_vec_input();

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

        let percentage_points: u32 = (points * 10) as u32;
        // Calculate how much money the user the user earned
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


pub fn load_or_new_db(db_name: &str) -> PickleDb {
    let load = PickleDb::load(
                            db_name,
                            PickleDbDumpPolicy::AutoDump,
                            SerializationMethod::Json);
    return match load {
        Ok(load) => load,
        Err(_) => {
            create_db(db_name)
        }
    };


}

fn create_db(db_name: &str) -> PickleDb {
    let mut new_db = PickleDb::new(
        db_name,
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    );

    new_db.set("HighScore", &0).unwrap();

    new_db
}


fn money_bet(money: u32) -> u32 {
    if money < 100 {
        println!("No More Money");
        process::exit(1);
    }

    println!("Input Bet (min 100):");
    let bet: u32 = get_input("Input...");


    if bet < 100 || bet > money {
        println!("Invalid quantity The Bet is 100");
        return 100;
    } else {
        println!("Bet is {}", bet);
        return bet;
    }

}

// Get user input
pub fn get_input<U: std::str::FromStr>(prompt: &str) -> U {

    loop {
        let mut input = String::new();

        // Reads the input from STDIN and places it in the String named input.
        println!("{}", prompt);
        io::stdin().read_line(&mut input)
            .expect("Failed to read input.");

        // Convert to another type.
        // If successful, bind to a new variable named input.
        // If failed, restart the loop.
        let input = match input.trim().parse::<U>() {
            Ok(parsed_input) => parsed_input,
            Err(_) => continue,
        };
        return input;
    }
}

// Get user input in the form of a vector
pub fn get_vec_input() -> Vec<i32> {
    let reader = io::stdin();

    let v: Vec<i32> =
        reader.lock()
            .lines().next().unwrap().unwrap()
            .split(' ').map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();


    v

}
