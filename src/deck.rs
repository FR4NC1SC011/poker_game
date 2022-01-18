// DECK
use crate::card::*;
use strum::IntoEnumIterator;

use rand::random;
use std::io::{self, BufRead};


//NOTE: This structure is necessary?
#[derive(Debug, Clone)]
pub struct Deck {
    pub hand: Vec<Card>
}



impl Deck {
    pub fn new() -> Self {                          // Create a new Deck
        let hand: Vec<Card> = Vec::new();
        let mut deck: Deck = Deck { hand };

        for s in Suits::iter() {
            for v in Values::iter() {
                let card = Card::new(v, s);
                deck.hand.push(card);
            }
        }

        deck
    }

    pub fn new_hand(deck: &mut Deck) -> Self {      // Create a new hand for the player
        let hand: Vec<Card> = Vec::new();
        let mut player_hand: Deck = Deck { hand };
        for _ in 0..5 {
            let index = (random::<f32>() * deck.hand.len() as f32).floor() as usize;
            player_hand.hand.push(deck.hand[index]);
            deck.hand.remove(index);
        }


        player_hand
    }

    pub fn change_cards(mut hand: Deck, deck: &mut Deck) -> Self {      // If the user wants to change the cards of the hand
        println!("Cards to change: ");
        let reader = io::stdin();

        // Read user input to vector in the form "%d %d %d ..." -> eg: 1 2 3
        let mut cards_to_change: Vec<i32> =
            reader.lock()
                .lines().next().unwrap().unwrap()
                .split(' ').map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();

        cards_to_change.truncate(3);            // Mantains the len of the vector in 3
        cards_to_change.sort();


        let mut z = 0;
        for x in &cards_to_change {
            if x > &4 || x < &0 {
                unimplemented!();
            }

            let i = x - z;
            hand.hand.remove(i.try_into().unwrap());
            z += 1;
        }

        for _ in 0..cards_to_change.len() {
            let index = (random::<f32>() * deck.hand.len() as f32).floor() as usize;
            hand.hand.push(deck.hand[index]);
            deck.hand.remove(index);
        }


        hand
    }
}
