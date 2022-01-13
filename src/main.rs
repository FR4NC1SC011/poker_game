use std::fmt;
use rand::random;
use std::io::{self, BufRead};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Card {
    value: Values,
    suit: Suits,
}

//NOTE: This structure is necessary?
#[derive(Debug, Clone)]
struct Deck {
    hand: Vec<Card>
}


#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Suits {
    Clubs,
    Hearts,
    Spades,
    Diamonds,
}


#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Values {
    Ace = 1,
    Two= 2,
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
}


enum PokerHands {
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    FourofAKind,
}

fn main() {

    /*
    let mut deck: Deck = Deck::new();
    let mut hand: Deck = Deck::new_hand(&mut deck);

    for x in &hand.hand {
        x.print_card();
    }

    hand = Deck::change_cards(hand, &mut deck);

    for x in &hand.hand {
        x.print_card();
    }

    let mut points = PokerHands::n_of_a_kind(&hand);
    println!("Total Points: {}", points);

    let mut points = PokerHands::is_straight(&mut hand);
    */

    let card1: Card = Card::new(Values::King, Suits::Clubs);
    let card2: Card = Card::new(Values::Jack, Suits::Clubs);
    let card3: Card = Card::new(Values::Queen, Suits::Clubs);
    let card4: Card = Card::new(Values::Ten, Suits::Clubs);
    let card5: Card = Card::new(Values::Ace, Suits::Clubs);

    let mut x: Vec<Card> = Vec::new();
    x.push(card1);
    x.push(card2);
    x.push(card3);
    x.push(card4);
    x.push(card5);

    let mut hand: Deck = Deck { hand: x };

    let points = PokerHands::is_straight(&mut hand);
    println!("{:?}", hand);
    println!("{}", points);

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
    fn new() -> Self {                          // Create a new Deck
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

    fn new_hand(deck: &mut Deck) -> Self {      // Create a new hand for the player
        let hand: Vec<Card> = Vec::new();
        let mut player_hand: Deck = Deck { hand };
        for _ in 0..5 {
            let index = (random::<f32>() * deck.hand.len() as f32).floor() as usize;
            player_hand.hand.push(deck.hand[index]);
            deck.hand.remove(index);
        }


        player_hand
    }

    fn change_cards(mut hand: Deck, deck: &mut Deck) -> Self {      // If the user wants to change the cards of the hand
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


impl PokerHands {
    fn n_of_a_kind(hand: &Deck) -> u32 {
        //NOTE: Optimize this code
        //let (mut a, mut b, mut c_c, mut d, mut e, mut f, mut g, mut h, mut i, mut j, mut k, mut l, mut m) = (0, 0, 0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0, 0, 0);
        let mut v = vec![0; 13];
        for c in &hand.hand {
            match c.value {
                Values::Ace => {
                    v[0] += 1;
                },
                Values::Two => {
                    v[1] += 1;
                },
                Values::Three => {
                    v[2] += 1;
                },
                Values::Four => {
                    v[3] += 1;
                },
                Values::Five => {
                    v[4] += 1;
                },
                Values::Six => {
                    v[5] += 1;
                },
                Values::Seven => {
                    v[6] += 1;
                },
                Values::Eight => {
                    v[7] += 1;
                },
                Values::Nine => {
                    v[8] += 1;
                },
                Values::Ten => {
                    v[9] += 1;
                },
                Values::Jack => {
                    v[10] += 1;
                },
                Values::Queen => {
                    v[11] += 1;
                },
                Values::King => {
                    v[12] += 1;
                },
            }
        }

        let mut two_pairs: u8 = 0;
        let mut points: u32 = 0;

        for j in v {
            match j {
                2 => {
                    two_pairs += 1;
                    if two_pairs == 1 {
                        println!("Pair");
                        points += 1;
                    } else if two_pairs == 2 {
                        println!("Two Pairs");
                        points += 2;
                    }
                },
                3 => {
                    println!("Three of a kind");
                    points += 5;
                },
                4 => {
                    println!("Poker");
                    points += 20;

                },
                _ => (),
            }
        }

        points
    }

    fn is_straight(hand: &mut Deck) -> u32 {
        let mut points: u32 = 0;
        let mut straight = true;

        hand.hand.sort();

        if hand.hand[4].value == Values::King && hand.hand[0].value == Values::Ace {
            for i in 2..4 {
                hand.hand[i].print_card();
                if hand.hand[i].value as i32 - hand.hand[i -1].value as i32 != 1 {
                    straight = false;
                }
            }
        } else {
            for i in 1..5 {
                if hand.hand[i].value as i32 - hand.hand[i -1].value as i32 != 1 {
                    straight = false;
                }
            }
        }

        if straight {
            points = 10;
        }

        points
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




#[cfg(test)]
mod tests {
    use crate::Card;
    use crate::Values;
    use crate::Suits;
    use crate::Deck;
    use crate::PokerHands;

    #[test]
    fn is_pair() {
        let card1: Card = Card::new(Values::Ace, Suits::Clubs);
        let card2: Card = Card::new(Values::Ace, Suits::Clubs);
        let card3: Card = Card::new(Values::Two, Suits::Clubs);
        let card4: Card = Card::new(Values::Three, Suits::Clubs);
        let card5: Card = Card::new(Values::Four, Suits::Clubs);

        let mut x: Vec<Card> = Vec::new();
        x.push(card1);
        x.push(card2);
        x.push(card3);
        x.push(card4);
        x.push(card5);

        let hand: Deck = Deck { hand: x };
        println!("{:?}", hand);

        let points = PokerHands::n_of_a_kind(&hand);
        assert_eq!(points, 1);
    }

    #[test]
    fn is_two_pair() {
        let card1: Card = Card::new(Values::Ace, Suits::Clubs);
        let card2: Card = Card::new(Values::Ace, Suits::Clubs);
        let card3: Card = Card::new(Values::Two, Suits::Clubs);
        let card4: Card = Card::new(Values::Two, Suits::Clubs);
        let card5: Card = Card::new(Values::Four, Suits::Clubs);

        let mut x: Vec<Card> = Vec::new();
        x.push(card1);
        x.push(card2);
        x.push(card3);
        x.push(card4);
        x.push(card5);

        let hand: Deck = Deck { hand: x };
        println!("{:?}", hand);

        let points = PokerHands::n_of_a_kind(&hand);
        assert_eq!(points, 3);
    }

    #[test]
    fn is_three_of_a_kind() {
        let card1: Card = Card::new(Values::Ace, Suits::Clubs);
        let card2: Card = Card::new(Values::Ace, Suits::Clubs);
        let card3: Card = Card::new(Values::Ace, Suits::Clubs);
        let card4: Card = Card::new(Values::Two, Suits::Clubs);
        let card5: Card = Card::new(Values::Four, Suits::Clubs);

        let mut x: Vec<Card> = Vec::new();
        x.push(card1);
        x.push(card2);
        x.push(card3);
        x.push(card4);
        x.push(card5);

        let hand: Deck = Deck { hand: x };
        println!("{:?}", hand);

        let points = PokerHands::n_of_a_kind(&hand);
        assert_eq!(points, 5);
    }

    #[test]
    fn is_poker() {
        let card1: Card = Card::new(Values::Ace, Suits::Clubs);
        let card2: Card = Card::new(Values::Ace, Suits::Clubs);
        let card3: Card = Card::new(Values::Ace, Suits::Clubs);
        let card4: Card = Card::new(Values::Ace, Suits::Clubs);
        let card5: Card = Card::new(Values::Four, Suits::Clubs);

        let mut x: Vec<Card> = Vec::new();
        x.push(card1);
        x.push(card2);
        x.push(card3);
        x.push(card4);
        x.push(card5);

        let hand: Deck = Deck { hand: x };
        println!("{:?}", hand);

        let points = PokerHands::n_of_a_kind(&hand);
        assert_eq!(points, 20);
    }

    #[test]
    fn is_straight() {
        let card1: Card = Card::new(Values::Ace, Suits::Clubs);
        let card2: Card = Card::new(Values::Two, Suits::Clubs);
        let card3: Card = Card::new(Values::Three, Suits::Clubs);
        let card4: Card = Card::new(Values::Four, Suits::Clubs);
        let card5: Card = Card::new(Values::Five, Suits::Clubs);

        let mut x: Vec<Card> = Vec::new();
        x.push(card1);
        x.push(card2);
        x.push(card3);
        x.push(card4);
        x.push(card5);

        let mut hand: Deck = Deck { hand: x };
        println!("{:?}", hand);

        let points = PokerHands::is_straight(&mut hand);
        assert_eq!(points, 10);
    }

    #[test]
    fn is_straight_v2() {
        let card1: Card = Card::new(Values::King, Suits::Clubs);
        let card2: Card = Card::new(Values::Jack, Suits::Clubs);
        let card3: Card = Card::new(Values::Queen, Suits::Clubs);
        let card4: Card = Card::new(Values::Ten, Suits::Clubs);
        let card5: Card = Card::new(Values::Ace, Suits::Clubs);

        let mut x: Vec<Card> = Vec::new();
        x.push(card1);
        x.push(card2);
        x.push(card3);
        x.push(card4);
        x.push(card5);

        let mut hand: Deck = Deck { hand: x };
        println!("{:?}", hand);

        let points = PokerHands::is_straight(&mut hand);
        assert_eq!(points, 10);
    }



}
