#![allow(dead_code)]
use std::fmt;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::cmp::Ordering;

#[derive(Copy,Clone)]
pub enum Color {
    Coeur,
    Carreau,
    Pique,
    Trefle,
}

#[derive(Copy,Clone)]
pub enum Value {
    Deux,
    Trois,
    Quatre,
    Cinq,
    Six,
    Sept,
    Huit,
    Neuf,
    Dix,
    Valet,
    Dame,
    Roi,
    As,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color = match *self {
            Color::Coeur => "♥",
            Color::Carreau => "♦",
            Color::Pique => "♠",
            Color::Trefle => "♣",
        };
        write!(f, "{}", color)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color = match *self {
            Value::Deux => "2",
            Value::Trois => "3",
            Value::Quatre => "4",
            Value::Cinq => "5",
            Value::Six => "6",
            Value::Sept => "7",
            Value::Huit => "8",
            Value::Neuf => "9",
            Value::Dix => "10",
            Value::Valet => "Valet",
            Value::Dame => "Dame",
            Value::Roi => "Roi",
            Value::As => "As",
        };
        write!(f, "{}", color)
    }
}

#[derive(Copy,Clone)]
pub struct Card {
    pub color: Color,
    pub value: Value,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.value, self.color)
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

pub struct Deck{
    pub deck :Vec<Card>
}
    
impl Deck{
    pub fn build()-> Deck{
        let mut d = Deck{deck:Vec::new()};
        for col in [Color::Carreau,Color::Coeur,Color::Pique,Color::Trefle].iter(){
            for val in [Value::As,Value::Roi,Value::Dame,Value::Valet,Value::Dix,
            Value::Neuf,Value::Huit,Value::Sept,Value::Six,Value::Cinq,Value::Quatre,
            Value::Trois,Value::Deux].iter(){
                d.deck.push(Card{color:*col,value:*val});
            }
        }
        d
    }
    pub fn shuffle(&mut self){
        self.deck.shuffle(&mut thread_rng());
    }
    pub fn give(&mut self)->Card{
        self.deck.remove(0)
    }
    pub fn take(&mut self,c :Card){
        self.deck.push(c);
    }
    pub fn size(&self)->usize{
        self.deck.len()
    }
    pub fn empty(&self)->bool{
        self.size()==0
    }
}