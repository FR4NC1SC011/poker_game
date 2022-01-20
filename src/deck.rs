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

    pub fn change_cards(mut hand: Deck, deck: &mut Deck, cards_to_change: Vec<i32>) -> Self {      // If the user wants to change the cards of the hand
        for x in &cards_to_change {
            if x > &4 || x < &0 {
                unimplemented!();
            }
            let index = (random::<f32>() * deck.hand.len() as f32).floor() as usize;
            let _ = std::mem::replace(&mut hand.hand[*x as usize], deck.hand[index]);
            deck.hand.remove(index);
        }

        /*
            let i = x - z;
            std::mem::replace(&mut hand.hand[*x as usize], );
            hand.hand.remove(i.try_into().unwrap());
            z += 1;
        }

        for _ in 0..cards_to_change.len() {
            let index = (random::<f32>() * deck.hand.len() as f32).floor() as usize;
            hand.hand.push(deck.hand[index]);
            deck.hand.remove(index);
        }
        */


        hand
    }

    pub fn print_hand(hand: &Deck) {
        let mut i = 0;
        for x in &hand.hand {
            print!("{}. ", i);
            x.print_card();
            i += 1;
        }
    }
}
