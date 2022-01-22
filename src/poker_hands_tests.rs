// TESTS FILE
#[cfg(test)]
mod poker_hands_tests {
    use crate::card::*;
    use crate::deck::*;
    use crate::poker_hands::*;

    #[test]
    fn is_pair_of_ace() {
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

        let cards: Deck = Deck { cards: x };
        println!("{:?}", cards);

        let points = PokerHands::check_for_n_of_a_kind_fullHouse(&cards);
        assert_eq!(points, 1);
    }

    #[test]
    fn is_pair_of_five() {
        let card1: Card = Card::new(Values::King, Suits::Clubs);
        let card2: Card = Card::new(Values::Jack, Suits::Hearts);
        let card3: Card = Card::new(Values::Five, Suits::Clubs);
        let card4: Card = Card::new(Values::Two, Suits::Spades);
        let card5: Card = Card::new(Values::Five, Suits::Clubs);

        let mut x: Vec<Card> = Vec::new();
        x.push(card1);
        x.push(card2);
        x.push(card3);
        x.push(card4);
        x.push(card5);

        let cards: Deck = Deck { cards: x };
        println!("{:?}", cards);

        let points = PokerHands::check_for_n_of_a_kind_fullHouse(&cards);
        assert_eq!(points, 1);
    }

    #[test]
    fn is_pair_of_king() {
        let card1: Card = Card::new(Values::King, Suits::Clubs);
        let card2: Card = Card::new(Values::Ace, Suits::Clubs);
        let card3: Card = Card::new(Values::Two, Suits::Clubs);
        let card4: Card = Card::new(Values::King, Suits::Clubs);
        let card5: Card = Card::new(Values::Four, Suits::Clubs);

        let mut x: Vec<Card> = Vec::new();
        x.push(card1);
        x.push(card2);
        x.push(card3);
        x.push(card4);
        x.push(card5);

        let cards: Deck = Deck { cards: x };
        println!("{:?}", cards);

        let points = PokerHands::check_for_n_of_a_kind_fullHouse(&cards);
        assert_eq!(points, 1);
    }



    #[test]
    fn is_two_pair() {
        let card1: Card = Card::new(Values::Ace, Suits::Spades);
        let card2: Card = Card::new(Values::Ace, Suits::Clubs);
        let card3: Card = Card::new(Values::Two, Suits::Diamonds);
        let card4: Card = Card::new(Values::Two, Suits::Clubs);
        let card5: Card = Card::new(Values::Four, Suits::Clubs);

        let mut x: Vec<Card> = Vec::new();
        x.push(card1);
        x.push(card2);
        x.push(card3);
        x.push(card4);
        x.push(card5);

        let cards: Deck = Deck { cards: x };
        println!("{:?}", cards);

        let points = PokerHands::check_for_n_of_a_kind_fullHouse(&cards);
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

        let cards: Deck = Deck { cards: x };
        println!("{:?}", cards);

        let points = PokerHands::check_for_n_of_a_kind_fullHouse(&cards);
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

        let cards: Deck = Deck { cards: x };
        println!("{:?}", cards);

        let points = PokerHands::check_for_n_of_a_kind_fullHouse(&cards);
        assert_eq!(points, 20);
    }

    #[test]
    fn is_full_house() {
        let card1: Card = Card::new(Values::King, Suits::Clubs);
        let card2: Card = Card::new(Values::King, Suits::Diamonds);
        let card3: Card = Card::new(Values::King, Suits::Clubs);
        let card4: Card = Card::new(Values::Queen, Suits::Spades);
        let card5: Card = Card::new(Values::Queen, Suits::Clubs);

        let mut x: Vec<Card> = Vec::new();
        x.push(card1);
        x.push(card2);
        x.push(card3);
        x.push(card4);
        x.push(card5);

        let cards: Deck = Deck { cards: x };
        println!("{:?}", cards);

        let points = PokerHands::check_for_n_of_a_kind_fullHouse(&cards);
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

        let mut cards: Deck = Deck { cards: x };
        println!("{:?}", cards);

        let points = PokerHands::check_for_straight_flush_royalFlush(&mut cards);
        assert_eq!(points, 10);
    }

    #[test]
    fn is_straight_v2() {
        let card1: Card = Card::new(Values::Six, Suits::Clubs);
        let card2: Card = Card::new(Values::Seven, Suits::Diamonds);
        let card3: Card = Card::new(Values::Eight, Suits::Clubs);
        let card4: Card = Card::new(Values::Nine, Suits::Diamonds);
        let card5: Card = Card::new(Values::Ten, Suits::Clubs);

        let mut x: Vec<Card> = Vec::new();
        x.push(card1);
        x.push(card2);
        x.push(card3);
        x.push(card4);
        x.push(card5);

        let mut cards: Deck = Deck { cards: x };
        println!("{:?}", cards);

        let points = PokerHands::check_for_straight_flush_royalFlush(&mut cards);
        assert_eq!(points, 10);
    }

    #[test]
    fn is_straight_v3() {
        let card1: Card = Card::new(Values::Six, Suits::Clubs);
        let card2: Card = Card::new(Values::Eight, Suits::Diamonds);
        let card3: Card = Card::new(Values::Ten, Suits::Clubs);
        let card4: Card = Card::new(Values::Nine, Suits::Diamonds);
        let card5: Card = Card::new(Values::Seven, Suits::Clubs);

        let mut x: Vec<Card> = Vec::new();
        x.push(card1);
        x.push(card2);
        x.push(card3);
        x.push(card4);
        x.push(card5);

        let mut cards: Deck = Deck { cards: x };
        println!("{:?}", cards);

        let points = PokerHands::check_for_straight_flush_royalFlush(&mut cards);
        assert_eq!(points, 10);
    }



    #[test]
    fn is_straight_royal_no_flush() {
        let card1: Card = Card::new(Values::King, Suits::Clubs);
        let card2: Card = Card::new(Values::Jack, Suits::Diamonds);
        let card3: Card = Card::new(Values::Queen, Suits::Clubs);
        let card4: Card = Card::new(Values::Ten, Suits::Diamonds);
        let card5: Card = Card::new(Values::Ace, Suits::Clubs);

        let mut x: Vec<Card> = Vec::new();
        x.push(card1);
        x.push(card2);
        x.push(card3);
        x.push(card4);
        x.push(card5);

        let mut cards: Deck = Deck { cards: x };
        println!("{:?}", cards);

        let points = PokerHands::check_for_straight_flush_royalFlush(&mut cards);
        assert_eq!(points, 10);
    }


    #[test]
    fn is_straight_flush_clubs() {
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

        let mut cards: Deck = Deck { cards: x };
        println!("{:?}", cards);

        let points = PokerHands::check_for_straight_flush_royalFlush(&mut cards);
        assert_eq!(points, 25);
    }

    #[test]
    fn is_straight_flush_diamonds() {
        let card1: Card = Card::new(Values::Ace, Suits::Diamonds);
        let card2: Card = Card::new(Values::Two, Suits::Diamonds);
        let card3: Card = Card::new(Values::Three, Suits::Diamonds);
        let card4: Card = Card::new(Values::Four, Suits::Diamonds);
        let card5: Card = Card::new(Values::Five, Suits::Diamonds);

        let mut x: Vec<Card> = Vec::new();
        x.push(card1);
        x.push(card2);
        x.push(card3);
        x.push(card4);
        x.push(card5);

        let mut cards: Deck = Deck { cards: x };
        println!("{:?}", cards);

        let points = PokerHands::check_for_straight_flush_royalFlush(&mut cards);
        assert_eq!(points, 25);
    }

    #[test]
    fn is_straight_flush_spades() {
        let card1: Card = Card::new(Values::Ace, Suits::Spades);
        let card2: Card = Card::new(Values::Two, Suits::Spades);
        let card3: Card = Card::new(Values::Three, Suits::Spades);
        let card4: Card = Card::new(Values::Four, Suits::Spades);
        let card5: Card = Card::new(Values::Five, Suits::Spades);

        let mut x: Vec<Card> = Vec::new();
        x.push(card1);
        x.push(card2);
        x.push(card3);
        x.push(card4);
        x.push(card5);

        let mut cards: Deck = Deck { cards: x };
        println!("{:?}", cards);

        let points = PokerHands::check_for_straight_flush_royalFlush(&mut cards);
        assert_eq!(points, 25);
    }

    #[test]
    fn is_straight_flush_hearts() {
        let card1: Card = Card::new(Values::Ace, Suits::Hearts);
        let card2: Card = Card::new(Values::Two, Suits::Hearts);
        let card3: Card = Card::new(Values::Three, Suits::Hearts);
        let card4: Card = Card::new(Values::Four, Suits::Hearts);
        let card5: Card = Card::new(Values::Five, Suits::Hearts);

        let mut x: Vec<Card> = Vec::new();
        x.push(card1);
        x.push(card2);
        x.push(card3);
        x.push(card4);
        x.push(card5);

        let mut cards: Deck = Deck { cards: x };
        println!("{:?}", cards);

        let points = PokerHands::check_for_straight_flush_royalFlush(&mut cards);
        assert_eq!(points, 25);
    }




    #[test]
    fn is_royal_flush_clubs() {
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

        let mut cards: Deck = Deck { cards: x };
        println!("{:?}", cards);

        let points = PokerHands::check_for_straight_flush_royalFlush(&mut cards);
        assert_eq!(points, 30);
    }

    #[test]
    fn is_royal_flush_diamonds() {
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

        let mut cards: Deck = Deck { cards: x };
        println!("{:?}", cards);

        let points = PokerHands::check_for_straight_flush_royalFlush(&mut cards);
        assert_eq!(points, 30);
    }

    #[test]
    fn is_royal_flush_spades() {
        let card1: Card = Card::new(Values::King, Suits::Spades);
        let card2: Card = Card::new(Values::Jack, Suits::Spades);
        let card3: Card = Card::new(Values::Queen, Suits::Spades);
        let card4: Card = Card::new(Values::Ten, Suits::Spades);
        let card5: Card = Card::new(Values::Ace, Suits::Spades);

        let mut x: Vec<Card> = Vec::new();
        x.push(card1);
        x.push(card2);
        x.push(card3);
        x.push(card4);
        x.push(card5);

        let mut cards: Deck = Deck { cards: x };
        println!("{:?}", cards);

        let points = PokerHands::check_for_straight_flush_royalFlush(&mut cards);
        assert_eq!(points, 30);
    }

    #[test]
    fn is_royal_flush_hearts() {
        let card1: Card = Card::new(Values::King, Suits::Hearts);
        let card2: Card = Card::new(Values::Jack, Suits::Hearts);
        let card3: Card = Card::new(Values::Queen, Suits::Hearts);
        let card4: Card = Card::new(Values::Ten, Suits::Hearts);
        let card5: Card = Card::new(Values::Ace, Suits::Hearts);

        let mut x: Vec<Card> = Vec::new();
        x.push(card1);
        x.push(card2);
        x.push(card3);
        x.push(card4);
        x.push(card5);

        let mut cards: Deck = Deck { cards: x };
        println!("{:?}", cards);

        let points = PokerHands::check_for_straight_flush_royalFlush(&mut cards);
        assert_eq!(points, 30);
    }

    #[test]
    fn is_flush_hearts() {
        let card1: Card = Card::new(Values::Ace, Suits::Hearts);
        let card2: Card = Card::new(Values::Jack, Suits::Hearts);
        let card3: Card = Card::new(Values::Two, Suits::Hearts);
        let card4: Card = Card::new(Values::Five, Suits::Hearts);
        let card5: Card = Card::new(Values::Seven, Suits::Hearts);

        let mut x: Vec<Card> = Vec::new();
        x.push(card1);
        x.push(card2);
        x.push(card3);
        x.push(card4);
        x.push(card5);

        let mut cards: Deck = Deck { cards: x };
        println!("{:?}", cards);

        let points = PokerHands::check_for_straight_flush_royalFlush(&mut cards);
        assert_eq!(points, 15);
    }

    #[test]
    fn is_flush_diamonds() {
        let card1: Card = Card::new(Values::Ace, Suits::Diamonds);
        let card2: Card = Card::new(Values::Jack, Suits::Diamonds);
        let card3: Card = Card::new(Values::Two, Suits::Diamonds);
        let card4: Card = Card::new(Values::Five, Suits::Diamonds);
        let card5: Card = Card::new(Values::Seven, Suits::Diamonds);

        let mut x: Vec<Card> = Vec::new();
        x.push(card1);
        x.push(card2);
        x.push(card3);
        x.push(card4);
        x.push(card5);

        let mut cards: Deck = Deck { cards: x };
        println!("{:?}", cards);

        let points = PokerHands::check_for_straight_flush_royalFlush(&mut cards);
        assert_eq!(points, 15);
    }

    #[test]
    fn is_flush_spades() {
        let card1: Card = Card::new(Values::Ace, Suits::Spades);
        let card2: Card = Card::new(Values::Jack, Suits::Spades);
        let card3: Card = Card::new(Values::Two, Suits::Spades);
        let card4: Card = Card::new(Values::Five, Suits::Spades);
        let card5: Card = Card::new(Values::Seven, Suits::Spades);

        let mut x: Vec<Card> = Vec::new();
        x.push(card1);
        x.push(card2);
        x.push(card3);
        x.push(card4);
        x.push(card5);

        let mut cards: Deck = Deck { cards: x };
        println!("{:?}", cards);

        let points = PokerHands::check_for_straight_flush_royalFlush(&mut cards);
        assert_eq!(points, 15);
    }

    #[test]
    fn is_flush_clubs() {
        let card1: Card = Card::new(Values::Ace, Suits::Clubs);
        let card2: Card = Card::new(Values::Jack, Suits::Clubs);
        let card3: Card = Card::new(Values::Two, Suits::Clubs);
        let card4: Card = Card::new(Values::Five, Suits::Clubs);
        let card5: Card = Card::new(Values::Seven, Suits::Clubs);

        let mut x: Vec<Card> = Vec::new();
        x.push(card1);
        x.push(card2);
        x.push(card3);
        x.push(card4);
        x.push(card5);

        let mut cards: Deck = Deck { cards: x };
        println!("{:?}", cards);

        let points = PokerHands::check_for_straight_flush_royalFlush(&mut cards);
        assert_eq!(points, 15);
    }
}
