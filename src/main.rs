use std::fmt;
use rand::random;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;


#[derive(Debug, Clone, Copy)]
struct Card {
    value: Values,
    suit: Suits,
}

#[derive(EnumIter, Debug, Clone, Copy)]
enum Suits {
    Clubs,
    Hearts,
    Spades,
    Diamonds,
}


#[derive(EnumIter, Debug, Clone, Copy)]
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
    let mut deck: Vec<Card> = Vec::new();
    let mut hand: Vec<Card> = Vec::new();

    for s in Suits::iter() {
        for v in Values::iter() {
            let card = Card::new(v, s);
            deck.push(card);
        }
    }


    for _ in 0..5 {
        let index = (random::<f32>() * deck.len() as f32).floor() as usize;
        hand.push(deck[index]);
        deck.remove(index);
    }

    for x in hand {
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
