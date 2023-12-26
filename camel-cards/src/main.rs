use std::{str::FromStr, cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Ord)]
enum Card {
    Number(u8),
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_input(byte: u8) -> Option<Self> {
        match byte {
            b'A' => Some(Self::Ace),
            b'K' => Some(Self::King),
            b'Q' => Some(Self::Queen),
            b'J' => Some(Self::Jack),
            b'T' => Some(Self::Number(10)),
            digit @ b'0'..=b'9' => Some(Self::Number(digit - b'0')),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Ord)]
enum Outcome {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand([Card; 5]);

#[derive(Debug)]
struct ParseError;

impl FromStr for Hand {
    type Err = ParseError;
    
    // TODO: how does one even leverage Err here
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [first, second, third, fourth, fifth] = s.as_bytes() else {
            panic!("Invalid hand: {:?} (expected 5 cards)", s);
        };

        Ok(Self([
            Card::from_input(*first).unwrap(),
            Card::from_input(*second).unwrap(),
            Card::from_input(*third).unwrap(),
            Card::from_input(*fourth).unwrap(),
            Card::from_input(*fifth).unwrap(),
        ]))
    }
}

#[derive(Debug, Eq)]
struct Game {
    pub hand: Hand,
    pub bid: u64
}

impl Game {
    pub fn get_outcome(&self) -> Outcome {
        let mut card_count = HashMap::new();
        for card in &self.hand.0 {
            *card_count.entry(card).or_insert(0) += 1;
        }
        match card_count.len() {
            1 => Outcome::FiveOfAKind,
            2 => {
                if card_count.values().any(|&count| count == 4) {
                    Outcome::FourOfAKind
                } else {
                    Outcome::FullHouse
                }
            }
            3 => {
                if card_count.values().any(|&count| count == 3) {
                    Outcome::ThreeOfAKind
                } else {
                    Outcome::TwoPair
                }
            }
            4 => Outcome::Pair,
            5 => Outcome::HighCard,
            _ => {
                panic!("Invalid hand: {:?}", self.hand);
            }
        }
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.get_outcome().cmp(&other.get_outcome()) {
            Ordering::Equal => {
                for (card, other_card) in self.hand.0.iter().zip(other.hand.0.iter()) {
                    match card.cmp(other_card) {
                        Ordering::Equal => continue,
                        ordering => return ordering,
                    }
                }
                return Ordering::Equal;
            },
            ordering => ordering,
        }
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}


impl PartialEq for Game {
    /// Two weighted hands are equal if the type of their hands are equal.
    fn eq(&self, other: &Self) -> bool {
        match self.get_outcome() == other.get_outcome() {
            true => {
                for (a, b) in self.hand.0.iter().zip(other.hand.0.iter()) {
                    if a != b {return false}
                }
                return true
            }
            _ => false
        }
    }
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s
            .split_once(' ')
            .unwrap();

        Ok(Self {
            hand: Hand::from_str(hand)?,
            bid: bid
                .parse::<u64>()
                .unwrap()
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Ord)]
enum JokerHandCard {
    Jack,
    Number(u8),
    Queen,
    King,
    Ace,
}

impl JokerHandCard {
    fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            b'A' => Some(Self::Ace),
            b'K' => Some(Self::King),
            b'Q' => Some(Self::Queen),
            b'J' => Some(Self::Jack),
            b'T' => Some(Self::Number(10)),
            digit @ b'0'..=b'9' => Some(Self::Number(digit - b'0')),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct JokerHand([JokerHandCard; 5]);

#[derive(Debug, Eq)]
struct JokerGame {
    pub hand: JokerHand,
    pub bid: u64
}

impl FromStr for JokerGame {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s
            .split_once(' ')
            .unwrap();

        Ok(Self {
            hand: JokerHand::from_str(hand)?,
            bid: bid
                .parse::<u64>()
                .unwrap()
        })
    }
}

impl PartialEq for JokerGame {
    /// Two weighted hands are equal if the type of their hands are equal.
    fn eq(&self, other: &Self) -> bool {
        match self.get_outcome() == other.get_outcome() {
            true => {
                for (a, b) in self.hand.0.iter().zip(other.hand.0.iter()) {
                    if a != b {return false}
                }
                return true
            }
            _ => false
        }
    }
}

impl Ord for JokerGame {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.get_outcome().cmp(&other.get_outcome()) {
            Ordering::Equal => {
                for (card, other_card) in self.hand.0.iter().zip(other.hand.0.iter()) {
                    match card.cmp(other_card) {
                        Ordering::Equal => continue,
                        ordering => return ordering,
                    }
                }
                return Ordering::Equal;
            },
            ordering => ordering,
        }
    }
}

impl PartialOrd for JokerGame {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl JokerGame {
    pub fn get_outcome(&self) -> Outcome {
        let mut card_count = HashMap::new();
        for card in &self.hand.0 {
            *card_count.entry(card).or_insert(0) += 1;
        }
        match card_count.len() {
            1 => Outcome::FiveOfAKind,
            2 => {
                if card_count.values().any(|&count| count == 4) {
                    if card_count.contains_key(&JokerHandCard::Jack) {
                        Outcome::FiveOfAKind
                    } else {
                        Outcome::FourOfAKind
                    }
                } else if card_count.contains_key(&JokerHandCard::Jack) {
                    Outcome::FiveOfAKind
                } else {
                    Outcome::FullHouse
                }
            }
            3 => {
                if card_count.values().any(|&count| count == 3) {
                    if card_count.contains_key(&JokerHandCard::Jack) {
                        Outcome::FourOfAKind
                    } else {
                        Outcome::ThreeOfAKind
                    }
                } else {
                    card_count
                        .get(&JokerHandCard::Jack)
                        .map_or(Outcome::TwoPair, |&count| {
                            if count == 2 {
                                Outcome::FourOfAKind
                            } else {
                                Outcome::FullHouse
                            }
                        })
                }
            }
            4 => {
                if card_count.contains_key(&JokerHandCard::Jack) {
                    Outcome::ThreeOfAKind
                } else {
                    Outcome::Pair
                }
            }
            5 => {
                if card_count.contains_key(&JokerHandCard::Jack) {
                    Outcome::Pair
                } else {
                    Outcome::HighCard
                }
            }
            _ => {
                panic!("Yikes")
            }
        }
    }
}

impl FromStr for JokerHand {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [first, second, third, fourth, fifth] = s.as_bytes() else {
            panic!("Invalid hand: {:?} (expected 5 cards)", s);
        };

        Ok(Self([
            JokerHandCard::from_byte(*first).unwrap(),
            JokerHandCard::from_byte(*second).unwrap(),
            JokerHandCard::from_byte(*third).unwrap(),
            JokerHandCard::from_byte(*fourth).unwrap(),
            JokerHandCard::from_byte(*fifth).unwrap(),
        ]))
    }
}

fn main() {
    let mut input = include_str!("../input.txt").lines().map(|x| x.parse::<Game>().unwrap()).collect::<Vec<Game>>();
    input.sort_unstable();
    let result: usize = input.iter().enumerate()
        .map(|(rank, hand)| {
            (rank + 1) * hand.bid as usize
        })
        .sum();

    println!("P1: {}", result);

    let mut input = include_str!("../input.txt").lines().map(|x| x.parse::<JokerGame>().unwrap()).collect::<Vec<JokerGame>>();
    input.sort_unstable();
    let result: usize = input.iter().enumerate()
        .map(|(rank, hand)| {
            (rank + 1) * hand.bid as usize
        })
        .sum();

    println!("P2: {}", result);
}
