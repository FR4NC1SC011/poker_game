// ---------------------------------------------------------------------------------------------------------
//                      TESTS
// ---------------------------------------------------------------------------------------------------------
#[cfg(test)]
mod deck_tests {
    use crate::card::*;
    use crate::deck::*;
    use crate::poker_hands::*;

    #[test]
    fn deck_is_52_cards() {
        let deck: Deck = Deck::new();

        assert_eq!(deck.cards.len(), 52);
    }

    #[test]
    fn change_one_card() {
        let mut deck: Deck = Deck::new();

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

        let mut y: Vec<Card> = Vec::new();
        y.push(card1);
        y.push(card2);
        y.push(card3);
        y.push(card4);
        y.push(card5);

        let mut hand: Deck = Deck { cards: x };
        let old_hand: Deck = Deck { cards: y };

        let cards_to_change: Vec<i32> = vec![3];

        hand = Deck::change_cards(hand, &mut deck, cards_to_change);

        assert_eq!(old_hand.cards[0], hand.cards[0]);
        assert_eq!(old_hand.cards[1], hand.cards[1]);
        assert_eq!(old_hand.cards[2], hand.cards[2]);
        assert_ne!(old_hand.cards[3], hand.cards[3]);
        assert_eq!(old_hand.cards[4], hand.cards[4]);
    }

    #[test]
    fn change_two_cards() {
        let mut deck: Deck = Deck::new();

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

        let mut y: Vec<Card> = Vec::new();
        y.push(card1);
        y.push(card2);
        y.push(card3);
        y.push(card4);
        y.push(card5);

        let mut hand: Deck = Deck { cards: x };
        let old_hand: Deck = Deck { cards: y };

        let cards_to_change: Vec<i32> = vec![2, 4];

        hand = Deck::change_cards(hand, &mut deck, cards_to_change);

        assert_eq!(old_hand.cards[0], hand.cards[0]);
        assert_eq!(old_hand.cards[1], hand.cards[1]);
        assert_ne!(old_hand.cards[2], hand.cards[2]);
        assert_eq!(old_hand.cards[3], hand.cards[3]);
        assert_ne!(old_hand.cards[4], hand.cards[4]);
    }

    #[test]
    fn change_three_cards() {
        let mut deck: Deck = Deck::new();

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

        let mut y: Vec<Card> = Vec::new();
        y.push(card1);
        y.push(card2);
        y.push(card3);
        y.push(card4);
        y.push(card5);

        let mut hand: Deck = Deck { cards: x };
        let old_hand: Deck = Deck { cards: y };

        let cards_to_change: Vec<i32> = vec![0, 2, 4];

        hand = Deck::change_cards(hand, &mut deck, cards_to_change);

        assert_ne!(old_hand.cards[0], hand.cards[0]);
        assert_eq!(old_hand.cards[1], hand.cards[1]);
        assert_ne!(old_hand.cards[2], hand.cards[2]);
        assert_eq!(old_hand.cards[3], hand.cards[3]);
        assert_ne!(old_hand.cards[4], hand.cards[4]);
    }

    #[test]
    fn change_three_not_sorted_cards() {
        let mut deck: Deck = Deck::new();

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

        let mut y: Vec<Card> = Vec::new();
        y.push(card1);
        y.push(card2);
        y.push(card3);
        y.push(card4);
        y.push(card5);

        let mut hand: Deck = Deck { cards: x };
        let old_hand: Deck = Deck { cards: y };

        let cards_to_change: Vec<i32> = vec![2, 4, 0];

        hand = Deck::change_cards(hand, &mut deck, cards_to_change);

        assert_ne!(old_hand.cards[0], hand.cards[0]);
        assert_eq!(old_hand.cards[1], hand.cards[1]);
        assert_ne!(old_hand.cards[2], hand.cards[2]);
        assert_eq!(old_hand.cards[3], hand.cards[3]);
        assert_ne!(old_hand.cards[4], hand.cards[4]);
    }

    #[test]
    fn change_cards_with_invalid_index() {
        let mut deck: Deck = Deck::new();

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

        let mut y: Vec<Card> = Vec::new();
        y.push(card1);
        y.push(card2);
        y.push(card3);
        y.push(card4);
        y.push(card5);

        let mut hand: Deck = Deck { cards: x };
        let old_hand: Deck = Deck { cards: y };

        let cards_to_change: Vec<i32> = vec![6, 4, 0];

        hand = Deck::change_cards(hand, &mut deck, cards_to_change);

        assert_ne!(old_hand.cards[0], hand.cards[0]);
        assert_eq!(old_hand.cards[1], hand.cards[1]);
        assert_eq!(old_hand.cards[2], hand.cards[2]);
        assert_eq!(old_hand.cards[3], hand.cards[3]);
        assert_ne!(old_hand.cards[4], hand.cards[4]);
    }

    #[test]
    fn change_cards_with_input_of_more_than_three_cards() {
        let mut deck: Deck = Deck::new();

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

        let mut y: Vec<Card> = Vec::new();
        y.push(card1);
        y.push(card2);
        y.push(card3);
        y.push(card4);
        y.push(card5);

        let mut hand: Deck = Deck { cards: x };
        let old_hand: Deck = Deck { cards: y };

        let cards_to_change: Vec<i32> = vec![3, 4, 0, 1];

        hand = Deck::change_cards(hand, &mut deck, cards_to_change);

        assert_ne!(old_hand.cards[0], hand.cards[0]);
        assert_eq!(old_hand.cards[1], hand.cards[1]);
        assert_eq!(old_hand.cards[2], hand.cards[2]);
        assert_ne!(old_hand.cards[3], hand.cards[3]);
        assert_ne!(old_hand.cards[4], hand.cards[4]);
    }

}
