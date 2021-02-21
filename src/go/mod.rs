use board::{GoStone, GoCoordinates, GoBoard};

pub mod board;
pub mod ai;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GoGame {
    board: Vec<GoBoard>,
    turn: u64,
    black_captures: u64,
    white_captures: u64,
}

impl GoGame {
    pub fn new() -> Self {
        let board = GoBoard::new();
        Self { board: vec![board], turn: 0, black_captures: 0, white_captures: 0 }
    }

    pub fn from_board(board: GoBoard, current_turn: GoStone) -> Self {
        assert!(!current_turn.is_none());

        Self {
            board: vec![board],
            turn: if current_turn == GoStone::BLACK {0} else {1},
            black_captures: 0,
            white_captures: 0,
        }
    }

    pub fn play_stone(&mut self, coordinates: GoCoordinates) -> Result<(), ()> {
        let current_board = self.current_board();
        if current_board.get_stone(coordinates) != GoStone::NONE {
            return Err(());
        }

        let mut new_board = current_board.clone();
        new_board.set_stone(coordinates, self.current_turn());
        let killed_stones = new_board.kill_stones(coordinates);

        if new_board.group_has_liberties(coordinates) {
            if self.board.contains(&new_board) {
                return Err(())
            }

            if self.current_turn() == GoStone::BLACK {
                self.black_captures += killed_stones;
            } else {
                self.white_captures += killed_stones;
            }
            self.board.push(new_board);
            self.turn += 1;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn current_turn(&self) -> GoStone {
        if self.turn % 2 == 0 {GoStone::BLACK} else {GoStone::WHITE}
    }

    pub fn current_board(&self) -> &GoBoard {
        self.board.last().unwrap()
    }

    pub fn undo(&mut self) -> Result<(), ()> {
        if self.board.len() > 1 {
            self.board.pop();
            self.turn -= 1;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn black_captures(&self) -> u64 {
        self.black_captures
    }

    pub fn white_captures(&self) -> u64 {
        self.white_captures
    }
}
