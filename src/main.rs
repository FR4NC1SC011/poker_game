use crate::deck::*;
use crate::poker_hands::*;
use std::io;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

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

    // create a new DB with AutoDum, meaning every change is written to the file,
    // and with Json serialization
    let mut db = load_or_new();

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

/*
pub fn load_or_new<P: AsRef<Path> + Copy>(
    db_path: P,
    dump_policy: PickleDbDumpPolicy,
    serialization_method: SerializationMethod,
) -> PickleDb {
    let load = PickleDb::load(db_path, dump_policy, serialization_method);
    return match load {
        Ok(load) => load,
        Err(_) => PickleDb::new(db_path, dump_policy, serialization_method),
    };
}
*/

pub fn load_or_new() -> PickleDb {
    let mut load = PickleDb::load(
                            "example.db",
                            PickleDbDumpPolicy::AutoDump,
                            SerializationMethod::Json);
    return match load {
        Ok(load) => load,
        Err(_) => {
            create_db()
        }
    };


}

fn create_db() -> PickleDb {
    let mut new_db = PickleDb::new(
        "example.db",
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    );

    new_db.set("HighScore", &0).unwrap();

    new_db
}
