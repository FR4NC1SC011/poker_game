use std::fmt;
use rand::random;
use std::io::{self, BufRead};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;


#[derive(Debug, Clone, Copy)]
struct Card {
    value: Values,
    suit: Suits,
}

//NOTE: This structure is necessary?
#[derive(Debug, Clone)]
struct Deck {
    hand: Vec<Card>
}


#[derive(EnumIter, Debug, Clone, Copy)]
enum Suits {
    Clubs,
    Hearts,
    Spades,
    Diamonds,
}


#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq)]
enum Values {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

fn main() {
    //let mut deck: Vec<Card> = Vec::new();
    //let mut hand: Vec<Card> = Vec::new();

    /*
    for s in Suits::iter() {
        for v in Values::iter() {
            let card = Card::new(v, s);
            deck.push(card);
        }
    }
    */

    let mut deck: Deck = Deck::new();
    let mut hand: Deck = Deck::new_hand(&mut deck);


    /*
    for _ in 0..5 {
        let index = (random::<f32>() * deck.hand.len() as f32).floor() as usize;
        hand.push(deck.hand[index]);
        deck.hand.remove(index);
    }
    */

    for x in &hand.hand {
        x.print_card();
    }

    //NOTE: Upgrade this piece of code
    /*
    println!("Cards to change: ");
    let reader = io::stdin();
    let mut cards_to_change: Vec<i32> =
        reader.lock()
              .lines().next().unwrap().unwrap()
              .split(' ').map(|s| s.trim())
              .filter(|s| !s.is_empty())
              .map(|s| s.parse().unwrap())
              .collect();

    cards_to_change.truncate(2);


    let mut z = 0;
    for x in &cards_to_change {

        if x > &4 {
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
    */

    hand = Deck::change_cards(hand, &mut deck);

    for x in &hand.hand {
        x.print_card();
    }

}





impl Card {
    fn new(value: Values, suit: Suits) -> Self {
        let card = Self {value, suit};

        card
    }

    fn print_card(self) {
        println!("{} of {}", self.value, self.suit);
    }

}

impl Deck {
    fn new() -> Self {
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

    fn new_hand(deck: &mut Deck) -> Self {
        let hand: Vec<Card> = Vec::new();
        let mut player_hand: Deck = Deck { hand };
        for _ in 0..5 {
            let index = (random::<f32>() * deck.hand.len() as f32).floor() as usize;
            player_hand.hand.push(deck.hand[index]);
            deck.hand.remove(index);
        }


        player_hand
    }

    fn change_cards(mut hand: Deck, deck: &mut Deck) -> Self {
        println!("Cards to change: ");
        let reader = io::stdin();
        let mut cards_to_change: Vec<i32> =
            reader.lock()
                .lines().next().unwrap().unwrap()
                .split(' ').map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();

        cards_to_change.truncate(2);            // Mantains the len of the vector in 3   ---  0,1,2


        let mut z = 0;
        for x in &cards_to_change {

            if x > &4 {
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







impl fmt::Display for Suits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Suits::Clubs => write!(f, "♣"),
           Suits::Hearts => write!(f, "♥"),
           Suits::Spades => write!(f, "♠"),
           Suits::Diamonds => write!(f, "♦"),
       }
    }
}

impl fmt::Display for Values {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Values::Ace => write!(f, "Ace"),
            Values::Two => write!(f, "Two"),
            Values::Three => write!(f, "Three"),
            Values::Four => write!(f, "Four"),
            Values::Five => write!(f, "Five"),
            Values::Six => write!(f, "Six"),
            Values::Seven => write!(f, "Seven"),
            Values::Eight => write!(f, "Eight"),
            Values::Nine => write!(f, "Nine"),
            Values::Ten => write!(f, "Ten"),
            Values::Jack => write!(f, "Jack"),
            Values::Queen => write!(f, "Queen"),
            Values::King => write!(f, "King"),
       }
    }
}
