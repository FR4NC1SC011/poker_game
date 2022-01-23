use crate::card::*;
use strum::IntoEnumIterator;
use rand::random;


#[derive(Debug, Clone)]
pub struct Deck {
    pub cards: Vec<Card>
}


impl Deck {

    // Create a new deck of 52 cards
    pub fn new() -> Self {
        let cards: Vec<Card> = Vec::new();
        let mut deck: Deck = Deck { cards };

        // Iterate over the 4 tipes of suits (Clubs, Hearts, Diamonds, Spades)
        for s in Suits::iter() {
            // Iterate over the card values (Ace, Two ... King)
            for v in Values::iter() {
                // Create the card (Value, Suit)
                let card = Card::new(v, s);

                // Push the card to the Deck vector
                deck.cards.push(card);
            }
        }

        // Return the Deck
        deck
    }


    // Create the user hand with 5 cards
    pub fn new_hand(deck: &mut Deck) -> Self {
        // Create a empty vector of type Card
        let cards: Vec<Card> = Vec::new();

        // Create the player hand of type Deck
        let mut player_hand: Deck = Deck { cards };

        // Push a random card to the player hand
        // We do this 5 times because the hand needs 5 cards
        for _ in 0..5 {
            // Get a random card from the deck
            let index = (random::<f32>() * deck.cards.len() as f32).floor() as usize;

            // Push the random card that we got before to the player hand
            player_hand.cards.push(deck.cards[index]);

            // Remove the random card from the deck
            deck.cards.remove(index);
        }


        // Return the player hand
        player_hand
    }


    // If the user wants to change cards of the hand
    pub fn change_cards(mut hand: Deck, deck: &mut Deck, mut cards_to_change: Vec<i32>) -> Self {

        // The game limits the user to a maximun of 3 changed cards
        // Truncate the cards_to_change vector to a len of 3
        cards_to_change.truncate(3);

        // Iterate over the vector that contains the index
        // Of the cards that the user wants to change
        for x in &cards_to_change {
            // The index needs to be between 0 - 4
            if x > &4 || x < &0 {
                println!("Ignoring Invalid Index {}", x);
                println!("Penalized with 50$");
                continue;
            }

            // Get a random card from the deck
            let index = (random::<f32>() * deck.cards.len() as f32).floor() as usize;

            // Replace the card in the player hand
            let _ = std::mem::replace(&mut hand.cards[*x as usize], deck.cards[index]);

            // Remove the card from the deck
            deck.cards.remove(index);
        }

        // Return the new hand
        hand
    }

    // Print the player hand
    pub fn print_hand(hand: &Deck) {
        // index variable
        let mut i = 0;

        // Iterate over the cards of the hand
        for x in &hand.cards {
            // Print the index
            print!("{}. ", i);
            // Print the card
            x.print_card();
            // Update index
            i += 1;
        }
    }
}

