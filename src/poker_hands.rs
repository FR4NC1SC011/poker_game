// HANDS

use crate::deck::*;
use crate::card::*;

pub enum PokerHands {
    Pair = 1,
    TwoPair = 3,
    ThreeOfAKind = 5,
    Straight = 10,
    Poker = 20,
}


impl PokerHands {
    pub fn n_of_a_kind(hand: &Deck) -> u32 {
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

        let mut n_pairs: u8 = 0;
        let mut points: u32 = 0;

        for j in v {
            match j {
                2 => {
                    n_pairs += 1;
                 },
                3 => {
                    println!("Three of a kind");
                    points += PokerHands::ThreeOfAKind as u32;
                },
                4 => {
                    println!("Poker");
                    points += PokerHands::Poker as u32;

                },
                _ => (),
            }
        }

        if n_pairs == 1 {
            println!("Pair");
            points += PokerHands::Pair as u32;
        } else if n_pairs == 2 {
            println!("Two Pairs");
            points += PokerHands::TwoPair as u32;
        }


        points
    }

    pub fn is_straight(hand: &mut Deck) -> u32 {
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
            points += PokerHands::Straight as u32;
            println!("Straight");
        }

        points
    }

}
