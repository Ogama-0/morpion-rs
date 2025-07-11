use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

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
            "{} {} {}\n{} {} {}\n{} {} {}",
            c[6], c[7], c[8], c[3], c[4], c[5], c[0], c[1], c[2]
        )
    }
}
pub fn switch_player(n: &mut i32) {
    *n = 3 - *n
}
pub fn play(current_player: &i32, case: &usize, bord: &mut Bord) -> Result {
    let inner_case: i32 = match bord.core.get(*case) {
        Some(c) => *c,
        None => return Err(std::fmt::Error {}),
    };
    if inner_case != 0 {
        return Err(std::fmt::Error {});
    }

    bord.core[*case] = *current_player;
    Ok(())
}
