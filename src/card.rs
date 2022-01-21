// CARD
use std::fmt;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub value: Values,
    pub suit: Suits,
}

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suits {
    Clubs,
    Hearts,
    Spades,
    Diamonds,
}


#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Values {
    Ace = 1,
    Two = 2,
    Three = 3 ,
    Four = 4,
    Five = 5,
    Six = 6 ,
    Seven = 7,
    Eight = 8,
    Nine = 9 ,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    // NO JOKER
}


impl Card {
    // Create a new Card
    pub fn new(value: Values, suit: Suits) -> Self {
        let card = Self {value, suit};

        card
    }

    // Print the Card
    pub fn print_card(self) {
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
