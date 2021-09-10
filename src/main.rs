mod game;
use game::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ordering;

fn round(j1: &mut Deck, j2: &mut Deck) {
    let mut winner = 0;
    let mut stack = Vec::new();
    let mut c1 = Card {
        color: Color::Hearts,
        value: Value::Two,
    };
    let mut c2 = Card {
        color: Color::Hearts,
        value: Value::Two,
    };
    while winner == 0 {
        println!("{} - {}", j1.size(), j2.size());
        if !j1.empty() {
            stack.push(j1.give());
            c1 = stack[stack.len() - 1];
        }
        if !j2.empty() {
            stack.push(j2.give());
            c2 = stack[stack.len() - 1];
        }
        println!("Player 1: {} - Player 2: {}", c1, c2);
        match c1.cmp(&c2) {
            Ordering::Less => {
                println!("Player 2 won!");
                winner = 2;
            }
            Ordering::Equal => {
                println!("War !");
                // Add hidden Cards
                if !j1.empty() {
                    stack.push(j1.give());
                    c1 = stack[stack.len() - 1];
                }
                if !j2.empty() {
                    stack.push(j2.give());
                    c2 = stack[stack.len() - 1];
                }
            }
            Ordering::Greater => {
                println!("Player 1 won!");
                winner = 1;
            }
        }
    }
    stack.shuffle(&mut thread_rng());
    if winner == 1 {
        while let Some(card) = stack.pop() {
            j1.take(card);
        }
    } else if winner == 2 {
        while let Some(card) = stack.pop() {
            j2.take(card);
        }
    } else {
        println!("Error: No winner.")
    }
}

fn main() {
    let mut d = Deck::build();
    d.shuffle();
    let mut j1 = Deck { deck: Vec::new() };
    let mut j2 = Deck { deck: Vec::new() };
    // distribute card
    while d.size() > 0 {
        j1.take(d.give());
        j2.take(d.give());
    }

    // Play a round until one player has no card remaining
    while !j1.empty() && !j2.empty() {
        round(&mut j1, &mut j2);
    }

    if j1.empty() {
        println!("Player 2 won the game!")
    } else if j2.empty() {
        println!("Player 1 won the game!")
    }
}
