const BOARD_SIZE: usize = 3;
const BOARD_LENGTH: usize = 9;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Player {
    X,
    O,
}

impl Player {
    fn next(&self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Tile {
    Marked(Player),
    Empty,
}

impl Tile {
    fn is_marked(&self) -> bool {
        match self {
            Self::Empty => false,
            _ => true,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum GameState {
    InProgress(Player),
    Won(Player),
    Tied,
}

impl GameState {
    fn is_finished(&self) -> bool {
        match self {
            Self::InProgress(_) => false,
            _ => true,
        }
    }

    fn current_player(&self) -> Player {
        match self {
            Self::InProgress(player) => player.clone(),
            _ => panic!("tried to get the current player of a finished game"),
        }
    }
}

type Board = [Tile; BOARD_LENGTH];

#[derive(Copy, Clone, Debug)]
pub struct Game {
    state: GameState,
    board: Board,
    moves: u8,
}

#[derive(Debug)]
pub enum MoveError {
    TileTaken,
    WrongTurn,
    InvalidIndex,
    GameFinished,
}

impl Game {
    pub fn new() -> Self {
        Game {
            state: GameState::InProgress(Player::X),
            board: [Tile::Empty; BOARD_LENGTH],
            moves: 0,
        }
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn make_move(&self, player: Option<Player>, board_index: usize) -> Result<Game, MoveError> {
        if board_index >= BOARD_LENGTH {
            return Err(MoveError::InvalidIndex);
        }

        if self.state.is_finished() {
            return Err(MoveError::GameFinished);
        }

        let player = match player {
            Some(received) => {
                let expected = self.state.current_player();
                if received != expected {
                    return Err(MoveError::WrongTurn);
                }
                received
            }
            None => self.state.current_player(),
        };

        if self.board[board_index].is_marked() {
            return Err(MoveError::TileTaken);
        }

        let marked_tile = Tile::Marked(player);
        let (board, state) = self.calculate_new_state(board_index, marked_tile);

        return Ok(Game {
            board,
            moves: self.moves + 1,
            state,
        });
    }

    fn calculate_new_state(&self, board_index: usize, tile: Tile) -> (Board, GameState) {
        let (x, y) = (board_index % BOARD_SIZE, board_index / BOARD_SIZE);
        let check = |c: &Tile| *c == tile;
        let mut won = false;

        let mut board = self.board.clone();
        board[board_index] = tile;

        let s = BOARD_SIZE;
        // primary diagonal
        if x == y {
            won |= board.iter().step_by(s + 1).all(check);
        }
        // secondary diagonal
        if x + y == s - 1 {
            won |= board.iter().skip(s - 1).step_by(s - 1).take(s).all(check);
        }
        // rows and colons
        won |= board.iter().skip(y * s).take(s).all(check)
            || board.iter().skip(x).step_by(s).all(check);

        if won {
            return (board, GameState::Won(self.state.current_player()));
        }
        if self.moves as usize == BOARD_LENGTH {
            return (board, GameState::Tied);
        }
        return (
            board,
            GameState::InProgress(self.state.current_player().next()),
        );
    }
}
