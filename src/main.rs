use crate::deck::*;
use crate::poker_hands::*;
//use std::io;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use std::io::{self, BufRead};

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


    let reader = io::stdin();

    // create a new DB with AutoDum, meaning every change is written to the file,
    // and with Json serialization
    let mut db = load_or_new("local.db");

    let mut deck: Deck = Deck::new();

    let mut score: u32 = 0;

    loop {
        print!("{esc}c", esc = 27 as char);     // Clean Screen
        println!("HighScore: {}", db.get::<u32>("HighScore").unwrap());
        println!("Score: {}", score);

        // We need min. 8 cards in the deck to keep playing.
        // 5 cards in the hand and 3 to change
        if deck.hand.len() < 8 {
            println!("No more cards");
            break;
        }


        let mut hand: Deck = Deck::new_hand(&mut deck);

        Deck::print_hand(&hand);

        // Read user input to vector in the form "%d %d %d ..." -> eg: 1 2 3
        let cards_to_change: Vec<i32> =
            reader.lock()
                .lines().next().unwrap().unwrap()
                .split(' ').map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();

        hand = Deck::change_cards(hand, &mut deck, cards_to_change);

        Deck::print_hand(&hand);

        let mut points = PokerHands::is_straight(&mut hand);
        if points == 0 {
            points = PokerHands::n_of_a_kind(&hand);
        }

        score += points;

        println!("Points of the Hand: {}", points);

        if score > db.get::<u32>("HighScore").unwrap() {
            db.set("HighScore", &score).unwrap();
        }

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

pub fn load_or_new(db_name: &str) -> PickleDb {
    let mut load = PickleDb::load(
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
