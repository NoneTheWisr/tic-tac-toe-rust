pub struct GameData {
    state: GameState,
    board: [CellState; 9],
    num_moves: u8,
}

#[derive(Clone, Copy)]
pub enum GameState {
    XsTurn,
    OsTurn,
    XWon,
    OWon,
    Stalemate,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CellState {
    Empty,
    X,
    O,
}

impl GameData {
    pub fn new() -> Self {
        GameData {
            state: GameState::XsTurn,
            board: [CellState::Empty; 9],
            num_moves: 0,
        }
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    pub fn board(&self) -> &[CellState; 9] {
        &self.board
    }

    pub fn turn(&mut self, board_index: usize) -> Option<GameState> {
        let mark;
        match self.state {
            GameState::XsTurn => mark = CellState::X,
            GameState::OsTurn => mark = CellState::O,
            _ => return None,
        };

        let ref mut cell = self.board[board_index];
        match cell {
            CellState::Empty => *cell = mark,
            _ => return None,
        }

        self.num_moves += 1;
        self.check_board(board_index, mark);

        return Some(self.state);
    }

    fn check_board(&mut self, board_index: usize, mark: CellState) {
        let mut won = false;
        let (x, y) = (board_index % 3, board_index / 3);
        let check = |c: &CellState| *c == mark;

        // primary diagonal
        if x == y {
            won |= self.board.iter().step_by(4).all(check);
        }
        // secondary diagonal
        if x + y == 2 {
            won |= self.board.iter().skip(2).step_by(2).take(3).all(check);
        }
        // rows and colons
        won |= self.board.iter().skip(y * 3).take(3).all(check)
            || self.board.iter().skip(x).step_by(3).all(check);

        if won {
            return match self.state {
                GameState::XsTurn => self.state = GameState::XWon,
                GameState::OsTurn => self.state = GameState::OWon,
                _ => (),
            };
        }

        if self.num_moves == 9 {
            return self.state = GameState::Stalemate;
        }

        match self.state {
            GameState::XsTurn => self.state = GameState::OsTurn,
            GameState::OsTurn => self.state = GameState::XsTurn,
            _ => (),
        }
    }
}
