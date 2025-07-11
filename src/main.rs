use crate::morpion::*;
use std::io;
mod morpion;
fn main() {
    let mut b = Bord::new();
    let mut winer = String::new();
    let mut current_player: i32 = 1;
    loop {
        println!(
            "c'est au joueur {} de jouer : entrez le numero de la case :",
            current_player
        );
        let mut guess = String::new();
        if let Win::Plyr(player) = b.check_win() {
            match player {
                Player::Player1(p1) => winer = p1,
                Player::Player2(p2) => winer = p2,
            }
            break;
        }
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let mut guess: usize = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        guess -= 1;
        if play(&current_player, &guess, &mut b).is_err() {
            continue;
        };
        println!("{}", b);
        switch_player(&mut current_player);
    }
    println!("{}", b);
    println!("GG {} You win !!", winer);
}
