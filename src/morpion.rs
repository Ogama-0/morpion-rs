use core::panic;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::io;

#[derive(PartialEq)]
pub enum Player {
    Player1(String),
    Player2(String),
}
impl Player {
    fn from(n: i32) -> Player {
        match n {
            1 => Player::Player1(String::from("Player 1")),
            2 => Player::Player2(String::from("Player 2")),
            _ => panic!("Il n'y a que 2 player differents, il n'y a pas {}", n),
        }
    }
}
#[derive(PartialEq)]
pub enum Win {
    Plyr(Player),
    False,
}
pub struct Bord {
    core: Vec<i32>,
}
impl Bord {
    pub fn new() -> Self {
        Bord {
            core: vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        }
    }
    pub fn check_win(&self) -> Win {
        let ligne = self.check_ligne();
        if ligne == Win::False {
            let col = self.check_coll();
            if col == Win::False {
                return self.check_diag();
            }
            return col;
        }
        ligne
    }
    fn check_ligne(&self) -> Win {
        for i in (0..9).step_by(3) {
            // [0, 3, 6]
            if self.core[i] != 0
                && self.core[i] == self.core[i + 1]
                && self.core[i] == self.core[i + 2]
            {
                return Win::Plyr(Player::from(self.core[i]));
            }
        }
        Win::False
    }
    fn check_coll(&self) -> Win {
        for i in 0..3 {
            if self.core[i] != 0
                && self.core[i] == self.core[i + 3]
                && self.core[i] == self.core[i + 6]
            {
                return Win::Plyr(Player::from(self.core[i]));
            }
        }
        Win::False
    }
    fn check_diag(&self) -> Win {
        if (self.core[0] != 0 && self.core[0] == self.core[4] && self.core[0] == self.core[8])
            || (self.core[2] != 0 && self.core[2] == self.core[4] && self.core[2] == self.core[6])
        {
            return Win::Plyr(Player::from(self.core[4]));
        }
        Win::False
    }
}
impl Display for Bord {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let c = &self.core;
        write!(
            f,
            "┌───┬───┬───┐\n│ {} │ {} │ {} │\n├───┼───┼───┤\n│ {} │ {} │ {} │\n├───┼───┼───┤\n│ {} │ {} │ {} │\n└───┴───┴───┘",
            better_display(c[6]),
            better_display(c[7]),
            better_display(c[8]),
            better_display(c[3]),
            better_display(c[4]),
            better_display(c[5]),
            better_display(c[0]),
            better_display(c[1]),
            better_display(c[2])
        )
    }
}
fn better_display(n: i32) -> char {
    match n {
        0 => ' ',
        1 => '╳',
        2 => '○',
        _ => panic!("il n'y a que deux player"),
    }
}
impl Debug for Bord {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let c = &self.core;
        write!(
            f,
            "─────────────\n| 7 | 8 | 9 |\n| 4 | 5 | 6 |\n| 1 | 2 | 3 |\n─────────────",
        )
    }
}
pub struct Morpion {
    bord: Bord,
}
impl Morpion {
    pub fn new() -> Self {
        Self { bord: Bord::new() }
    }
    pub fn new_game(&mut self) {
        let mut winer = String::new();
        let mut current_player: i32 = 1;
        loop {
            println!(
                "c'est au joueur {} de jouer : entrez le numero de la case :",
                current_player
            );
            let mut guess = String::new();
            if let Win::Plyr(player) = self.bord.check_win() {
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
            if self.play(&current_player, &guess).is_err() {
                continue;
            };
            println!("{}", self.bord);
            switch_player(&mut current_player);
        }
        println!("{}", self.bord);
        println!("GG {} You win !!", winer);
    }
    pub fn play(&mut self, current_player: &i32, case: &usize) -> Result {
        let inner_case: i32 = match self.bord.core.get(*case) {
            Some(c) => *c,
            None => return Err(std::fmt::Error {}),
        };
        if inner_case != 0 {
            return Err(std::fmt::Error {});
        }

        self.bord.core[*case] = *current_player;
        Ok(())
    }
}
pub fn switch_player(n: &mut i32) {
    *n = 3 - *n
}
