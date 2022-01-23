use crate::deck::*;
use crate::card::*;

// Declare the Poker Hands and how many points they return
pub enum PokerHands {
    Pair = 1,
    TwoPair = 3,
    ThreeOfAKind = 5,
    Straight = 10,
    FullHouse = 13,
    Flush = 15,
    Poker = 20,
    StraightFlush = 25,
    RoyalFlush = 30,
}


impl PokerHands {
    pub fn check_for_n_of_a_kind_fullHouse(hand: &Deck) -> u16 {
        // Declare a vector of 13 elements
        // each element represent a card
        // v[0] = Ace, v[1] = Two, v[2] = Three ... v[13] = King
        let mut v = vec![0; 13];

        // Count the cards in the Hand
        for c in &hand.cards {
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
        let mut three_kind: bool = false;

        // Check for Pair, Three of a Kind, Poker or Full House
        for j in v {
            match j {
                2 => {
                    // Count how many pairs we got
                    n_pairs += 1;
                 },
                3 => {
                    // Is three of a kind
                    three_kind = true;
                },
                4 => {
                    // Is Poker
                    println!("Poker");

                    // Return the points
                    return PokerHands::Poker as u16;

                },
                _ => (),
            }
        }


        // Return the points of the hand
        if three_kind && n_pairs == 1 {
            println!("Full House");
            return PokerHands::FullHouse as u16;
        } else if three_kind {
            println!("Three Of a Kind");
            return PokerHands::ThreeOfAKind as u16;
        } else if n_pairs == 2 {
            println!("Two Pairs");
            return PokerHands::TwoPair as u16;
        } else if n_pairs == 1 {
            println!("Pair");
            return PokerHands::Pair as u16;
        } else {
            return 0;
        }

    }

    pub fn check_for_straight_flush_royalFlush(hand: &mut Deck) -> u16 {

        let mut flush: bool = true;
        let mut straight = true;
        let mut is_royal = true;

        // Sort the hand cards
        hand.cards.sort();

        // Check for Royal Straight
        if hand.cards[4].value == Values::King && hand.cards[0].value == Values::Ace {
            straight = false;
            // We know the first and last element of the hand
            // We can avoid them in the for loop
            for i in 2..4 {
                // Check if the vector elements are consecutive
                // If the elements are not consecutive then royal straight is false
                if hand.cards[i].value as i32 - hand.cards[i -1].value as i32 != 1 {
                    is_royal = false;
                    break;
                }
            }
        } else {
            is_royal = false;
            // Check for Straight
            for i in 1..5 {
                // Check if the vector elements are consecutive
                // If the elements are not consecutive then straight is false
                if hand.cards[i].value as i32 - hand.cards[i -1].value as i32 != 1 {
                    straight = false;
                    break;
                }
            }
        }

            // Suit of the first cards
            let suit = hand.cards[0].suit;
            // Check for Flush
            for c in 0..5 {
                // If the suit of the card is different to the suit of the first card
                // Then is not Flush
                if hand.cards[c].suit != suit {
                    flush = false;
                }
            }

        // Return the points of the hand
        if is_royal && flush {
            return PokerHands::RoyalFlush as u16;
        } else if straight && flush {
            return PokerHands::StraightFlush as u16;
        } else if straight || is_royal {
            return PokerHands::Straight as u16;
        } else if flush {
            return PokerHands::Flush as u16;
        } else {
            0
        }

    }

}
