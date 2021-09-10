#![allow(dead_code)]
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ordering;
use std::fmt;

#[derive(Copy, Clone)]
pub enum Color {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

#[derive(Copy, Clone)]
pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color = match *self {
            Color::Hearts => "♥",
            Color::Diamonds => "♦",
            Color::Spades => "♠",
            Color::Clubs => "♣",
        };
        write!(f, "{}", color)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color = match *self {
            Value::Two => "2",
            Value::Three => "3",
            Value::Four => "4",
            Value::Five => "5",
            Value::Six => "6",
            Value::Seven => "7",
            Value::Eight => "8",
            Value::Nine => "9",
            Value::Ten => "10",
            Value::Jack => "Jack",
            Value::Queen => "Queen",
            Value::King => "King",
            Value::Ace => "Ace",
        };
        write!(f, "{}", color)
    }
}

#[derive(Copy, Clone)]
pub struct Card {
    pub color: Color,
    pub value: Value,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.color, self.value)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.value as usize).cmp(&(other.value as usize))
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value as usize == other.value as usize
    }
}

impl Eq for Card {}

pub struct Deck {
    pub deck: Vec<Card>,
}

impl Deck {
    pub fn build() -> Deck {
        let mut d = Deck { deck: Vec::new() };
        for col in [Color::Hearts, Color::Diamonds, Color::Spades, Color::Clubs].iter() {
            for val in [
                Value::Two,
                Value::Three,
                Value::Four,
                Value::Five,
                Value::Six,
                Value::Seven,
                Value::Eight,
                Value::Nine,
                Value::Ten,
                Value::Jack,
                Value::Queen,
                Value::King,
                Value::Ace,
            ]
            .iter()
            {
                d.deck.push(Card {
                    color: *col,
                    value: *val,
                });
            }
        }
        d
    }
    pub fn shuffle(&mut self) {
        self.deck.shuffle(&mut thread_rng());
    }
    pub fn give(&mut self) -> Card {
        self.deck.remove(0)
    }
    pub fn take(&mut self, c: Card) {
        self.deck.push(c);
    }
    pub fn size(&self) -> usize {
        self.deck.len()
    }
    pub fn empty(&self) -> bool {
        self.size() == 0
    }
}
