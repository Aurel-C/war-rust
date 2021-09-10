mod game;
use game::*;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::cmp::Ordering;

fn round(j1 : &mut Deck,j2: &mut Deck){
    let mut winner = 0;
    let mut stack = Vec::new();
    let mut c1 = Card{color:Color::Coeur,value:Value::Deux};
    let mut c2 = Card{color:Color::Coeur,value:Value::Deux};
    while winner == 0 {
        println!("{} - {}",j1.size(),j2.size());
        if !j1.empty(){
            stack.push(j1.give());
            c1 = stack[stack.len()-1];
        }
        if !j2.empty(){
            stack.push(j2.give());
            c2 = stack[stack.len()-1];
        }
        println!("Joueur1: {} - Joueur2: {}",c1,c2);
        match c1.cmp(&c2){
            Ordering::Less=> {
                println!("Joueur2 gagne!");
                winner = 2;
            },
            Ordering::Equal=>{
                println!("Égalité !");
                // Add hidden Cards
                if !j1.empty(){
                    stack.push(j1.give());
                    c1 = stack[stack.len()-1];
                }
                if !j2.empty(){
                    stack.push(j2.give());
                    c2 = stack[stack.len()-1];
                }
            },
            Ordering::Greater=>{
                println!("Joueur1 gagne!");
                winner = 1;
            }
        }
    }
    stack.shuffle(&mut thread_rng());
    if winner == 1{
        while let Some(card) =  stack.pop(){
            j1.take(card);
        }
    }else if winner == 2 {
        while let Some(card) =  stack.pop(){
            j2.take(card);
        }
    }else{
        println!("Erreur: Pas de vainqueur.")
    }
}


fn main() {
    let mut d = Deck::build();
    d.shuffle();
    let mut j1 = Deck{deck:Vec::new()};
    let mut j2 = Deck{deck:Vec::new()};
    // distribute card
    while d.size() > 0 {
        j1.take(d.give());
        j2.take(d.give());
    }

    // Play a round until one player has no card remaining
    while !j1.empty() && !j2.empty(){
        round(&mut j1,&mut j2);
    }

    if j1.empty(){
        println!("Joueur2 à gagné la partie!")
    }else if j2.empty(){
        println!("Joueur1 à gagné la partie!")
    }
}
