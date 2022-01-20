use crate::deck::*;
use crate::poker_hands::*;
//use std::io;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use std::io::{self, BufRead};
use text_io::read;

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
    let mut db = load_or_new_db("local.db");

    let mut deck: Deck = Deck::new();

    let mut score: u16 = 0;

    let mut money: u32 = 1000;


    loop {
        print!("{esc}c", esc = 27 as char);     // Clean Screen
        println!("HighScore: {}", db.get::<u32>("HighScore").unwrap());
        println!("Score: {}", score);
        println!("Money: {}", money);

        let mut money_bet = money_bet(money);
        money = money - money_bet;


        // We need min. 8 cards in the deck to keep playing.
        // 5 cards in the hand and 3 to change
        if deck.hand.len() < 8 {
            println!("No more cards");
            break;
        }


        let mut hand: Deck = Deck::new_hand(&mut deck);

        Deck::print_hand(&hand);

        // Read user input to vector in the form "%d %d %d ..." -> eg: 1 2 3
        let mut cards_to_change: Vec<i32> =
            reader.lock()
                .lines().next().unwrap().unwrap()
                .split(' ').map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();
        println!("Cards to change: ");

        cards_to_change.truncate(3);            // Mantains the len of the vector in 3
        cards_to_change.sort();

        let n_cards_to_change: u32 = cards_to_change.len() as u32;

        hand = Deck::change_cards(hand, &mut deck, cards_to_change);

        money = (money - (n_cards_to_change * 50));
        money_bet = money_bet + (n_cards_to_change * 50);

        Deck::print_hand(&hand);

        let mut points = PokerHands::is_straight(&mut hand);
        if points == 0 {
            points = PokerHands::n_of_a_kind(&hand);
        }

        let percentage: u32 = (points * 10) as u32;
        if percentage > 0 {
            money = money + (money_bet/percentage) * 10;
        }

        score += points;

        println!("Points of the Hand: {}", points);

        if score > db.get::<u16>("HighScore").unwrap() {
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

pub fn load_or_new_db(db_name: &str) -> PickleDb {
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


fn money_bet(money: u32) -> u32 {
    if money < 100 {
        println!("No More Money");
        // TODO: Exit Program
    }

    println!("Bet (min 100):");
    let bet: u32 = read!();
    if bet < 100 || bet > money {
        println!("Invalid quantity The Bet is 100");
        return 100;
    } else {
        println!("Bet is {}", bet);
        return bet;
    }


    /*
    let mut money_bet = String::new();
    io::stdin()
        .read_line(&mut money_bet)
        .expect("Failed to read input");

    if let Ok(mut result) = money_bet.parse::<u32>() {
        println!("U32 RESULT: {}", result);
        if result < 100 || result > money {
            println!("Invalid quantity");
            result = 100;
        }
        println!("Bet is: {}", result);
        return result;
    } else {
        println!("Invalid quantity the Bet is 100");
        return 100;
    }
    */

}
