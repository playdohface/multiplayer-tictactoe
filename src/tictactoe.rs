/// Module to store Types and logic related to the Game
use rand::seq::SliceRandom;
use serde::Serialize;
use std::fmt::Display;

pub fn best_next_move(b: &Board, lvl: &Difficulty) -> usize {
    let winconditions = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8], // horizontal
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8], // vertical
        [0, 4, 8],
        [2, 4, 6],
    ]; // diagonal

    let me: Field;
    let other: Field;
    let mut block: usize = 10;
    match b.next_turn {
        Player::X => {
            me = Field::X;
            other = Field::O
        }
        Player::O => {
            me = Field::O;
            other = Field::X
        }
    }

    // first check whether we can win this turn
    for condition in winconditions {
        let mut curr: Vec<Field> = Vec::new();
        for index in condition {
            curr.push(*b.fields.get(index).unwrap());
        }
        if curr.contains(&other) && curr.contains(&me) {
            continue;
        }
        let mut i = 0;
        let mut count = 0;
        let mut othercount = 0;
        let mut free = 10;
        while i <= 2 {
            if curr[i] == me {
                count += 1;
            } else if curr[i] == Field::Empty {
                free = i;
            } else if curr[i] == other {
                othercount += 1;
            }
            i += 1;
        }
        if count == 2 && lvl.take_win {
            return condition[free];
        } else if othercount == 2 {
            block = condition[free];
        }
    }
    // if the other player could win next turn, we block
    if block != 10 && lvl.block {
        return block;
    }

    // otherwise, we prefer the center if we can have it
    if b.fields[4] == Field::Empty && lvl.prefer_center {
        return 4;
    }
    // if we can't have the center, we prefer a corner if we can have it
    let mut rng = rand::thread_rng();
    if lvl.prefer_corners {
        let mut pref = [0, 2, 6, 8];
        pref.shuffle(&mut rng);
        for index in pref {
            if b.fields[index] == Field::Empty {
                return index;
            }
        }
    }
    // finally, we pick at random
    let mut pref = [1, 2, 3, 4, 5, 6, 7, 8];
    pref.shuffle(&mut rng);
    for index in pref {
        if b.fields[index] == Field::Empty {
            return index;
        }
    }
    11 // if we return >8  something is wrong, we should have exhausted all possibilities by now
}

pub struct Difficulty {
    block: bool,
    prefer_center: bool,
    prefer_corners: bool,
    take_win: bool,
}

impl Difficulty {
    fn hardest() -> Self {
        Difficulty {
            block: true,
            prefer_center: true,
            prefer_corners: true,
            take_win: true,
        }
    }

    fn hard() -> Self {
        Difficulty {
            block: true,
            prefer_center: false,
            prefer_corners: false,
            take_win: true,
        }
    }

    fn medium() -> Self {
        Difficulty {
            block: false,
            prefer_center: true,
            prefer_corners: true,
            take_win: true,
        }
    }

    fn beatable() -> Self {
        Difficulty {
            block: false,
            prefer_center: false,
            prefer_corners: true,
            take_win: true,
        }
    }

    fn easy() -> Self {
        Difficulty {
            block: false,
            prefer_center: false,
            prefer_corners: false,
            take_win: true,
        }
    }

    fn easiest() -> Self {
        Difficulty {
            block: false,
            prefer_center: false,
            prefer_corners: false,
            take_win: false,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Serialize)]
pub enum Field {
    X,
    O,
    Empty,
}
impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X => write!(f, "X"),
            Self::O => write!(f, "O"),
            Self::Empty => write!(f, " "),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Serialize)]
pub enum Player {
    X,
    O,
}
impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X => write!(f, "Player X"),
            Self::O => write!(f, "Player O"),
        }
    }
}
impl std::ops::Not for Player {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}
impl std::convert::TryFrom<Field> for Player {
    type Error = ();
    fn try_from(value: Field) -> Result<Self, ()> {
        match value {
            Field::X => Ok(Self::X),
            Field::O => Ok(Self::O),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Board {
    fields: [Field; 9],
    pub next_turn: Player,
}

impl Board {
    pub fn new() -> Board {
        Board {
            fields: [Field::Empty; 9],
            next_turn: Player::X,
        }
    }

    pub fn add_turn(&mut self, position: usize) -> bool {
        if position > 8 || self.fields[position] != Field::Empty || self.get_winner() != None {
            false
        } else {
            match self.next_turn {
                Player::X => self.fields[position] = Field::X,
                Player::O => self.fields[position] = Field::O,
            }
            self.next_turn = !self.next_turn;
            true
        }
    }

    pub fn show(&self) -> [Field; 9] {
        self.fields
        /*  format!(
            "{}|{}|{}\n{}|{}|{}\n{}|{}|{}\n",
            self.fields[0],
            self.fields[1],
            self.fields[2],
            self.fields[3],
            self.fields[4],
            self.fields[5],
            self.fields[6],
            self.fields[7],
            self.fields[8]
        ) */
    }
    /// If theres a winner, returns Some(Winnerfield, wincondition), Some(Field::Empty, 10) for draw
    /// None if the game is undecided
    pub fn get_winner(&self) -> Option<(Field, usize)> {
        let winconditions = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8], // horizontal
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8], // vertical
            [0, 4, 8],
            [2, 4, 6],
        ]; // diagonal
        let mut blocked: usize = 0;
        for (i, condition) in winconditions.iter().enumerate() {
            let mut curr: Vec<Field> = Vec::new();
            for index in *condition {
                curr.push(*self.fields.get(index).unwrap());
            }

            if curr.contains(&Field::X) && curr.contains(&Field::O) {
                blocked += 1;
                continue;
            } else if curr.contains(&Field::Empty) {
                continue;
            } else {
                match curr[0] {
                    Field::X => {
                        return Some((Field::X, i));
                    }
                    Field::O => {
                        return Some((Field::O, i));
                    }
                    Field::Empty => panic!("Winner can not be empty field"),
                }
            }
        }
        if blocked == winconditions.len() {
            // it's a draw
            Some((Field::Empty, 10))
        } else {
            // the outcome is not yet determined
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x_is_first() {
        let game = Board::new();
        assert!(game.next_turn == Player::X);
    }

    #[test]
    fn o_is_second() {
        let mut game = Board::new();
        game.add_turn(0);
        assert_eq!(game.next_turn, Player::O);
    }

    #[test]
    fn occupied_fields_are_not_overwritten() {
        let mut game = Board::new();
        game.add_turn(0);
        assert!(!game.add_turn(0));
        assert_eq!(game.fields[0], Field::X);
    }
}
