# Getting Started With Poker Game

This is a simple Poker Game command line program for a single player.
The game is developed using Rust.

## Poker Hand Sorter

A poker hand consists of a combination of five playing cards, ranked in the following ascending order (lowest to
highest):


| Points |  Combination  |                Description                |
|:------:|:-------------:|:-----------------------------------------:|
|    1   |   Pair    |Two cards of same value|
|    3   |   Two pairs   |Two different pairs|
|    5   |   Three of a kind    |Three cards of the same value|
|    10  |   Straight    |All five cards in consecutive value order|
|    13  |   Flush    |All five cards having the same suit|
|    15  |   Full house    |Three of a kind and a Pair|
|    20  |   Four of a kind    |Four cards of the same value|
|    25  |   Straight flush    |All five cards in consecutive value order, with the same suit|
|    30  |   Royal Flush    |Ten, Jack, Queen, King and Ace in the same suit|

Suits are: Diamonds(D), Hearts(H), Spades(S), Clubs(C)

### Bet

The minimun bet to play is: 100
If the user cannot bet that quantity the game ends

The player earns money using the next formula:

(MONEY_BET/10) * (POINTS * 10)

### For Example

| Hand |Player|BET|EARNINGS|
|:----:|:-------------:|-----|--------|
|  1   |  4H 4C 6S 7S KD <br/>Pair of Fours|BET: 100|(100/10) * (1 * 10) = 100 <br/> (Player recovers the money of the bet)
|  2   |  2D 9C AS AH AC <br/>Three Aces|BET: 200|(200/10) * (5 * 10) = 1000
|  3   |  4C 6C 9C KC QC <br/>Flush|BET: 150|(150/10) * (13 * 10) =  1950





