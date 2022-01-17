// TESTS FILE
#[cfg(test)]
mod tests {
    use crate::card::*;
    use crate::deck::*;
    use crate::poker_hands::*;

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
    fn is_full_house() {
        let card1: Card = Card::new(Values::King, Suits::Clubs);
        let card2: Card = Card::new(Values::King, Suits::Clubs);
        let card3: Card = Card::new(Values::King, Suits::Clubs);
        let card4: Card = Card::new(Values::Queen, Suits::Clubs);
        let card5: Card = Card::new(Values::Queen, Suits::Clubs);

        let mut x: Vec<Card> = Vec::new();
        x.push(card1);
        x.push(card2);
        x.push(card3);
        x.push(card4);
        x.push(card5);

        let hand: Deck = Deck { hand: x };
        println!("{:?}", hand);

        let points = PokerHands::n_of_a_kind(&hand);
        assert_eq!(points, 13);
    }

    #[test]
    fn is_straight() {
        let card1: Card = Card::new(Values::Ace, Suits::Clubs);
        let card2: Card = Card::new(Values::Two, Suits::Diamonds);
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
    fn is_straight_flush() {
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
        assert_eq!(points, 25);
    }


    #[test]
    fn is_straight_royal_no_flush() {
        let card1: Card = Card::new(Values::King, Suits::Clubs);
        let card2: Card = Card::new(Values::Jack, Suits::Diamonds);
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

    #[test]
    fn is_royal_flush() {
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
        assert_eq!(points, 30);
    }

    #[test]
    fn is_royal_flush_v2() {
        let card1: Card = Card::new(Values::King, Suits::Diamonds);
        let card2: Card = Card::new(Values::Jack, Suits::Diamonds);
        let card3: Card = Card::new(Values::Queen, Suits::Diamonds);
        let card4: Card = Card::new(Values::Ten, Suits::Diamonds);
        let card5: Card = Card::new(Values::Ace, Suits::Diamonds);

        let mut x: Vec<Card> = Vec::new();
        x.push(card1);
        x.push(card2);
        x.push(card3);
        x.push(card4);
        x.push(card5);

        let mut hand: Deck = Deck { hand: x };
        println!("{:?}", hand);

        let points = PokerHands::is_straight(&mut hand);
        assert_eq!(points, 30);
    }


}
