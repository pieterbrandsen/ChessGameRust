use lazy_static::lazy_static;

use std::fmt;
use std::sync::Mutex;

#[derive(Clone)]
pub struct Piece {
    position: Position,
    piece_type: PieceType,
    side: Side,
    has_moved: bool,
}
pub type Pieces = Vec<Piece>;

#[derive(Clone)]
pub struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}
impl PartialEq for PieceType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PieceType::Pawn, PieceType::Pawn) => true,
            (PieceType::Knight, PieceType::Knight) => true,
            (PieceType::Bishop, PieceType::Bishop) => true,
            (PieceType::Rook, PieceType::Rook) => true,
            (PieceType::Queen, PieceType::Queen) => true,
            (PieceType::King, PieceType::King) => true,
            _ => false,
        }
    }
}
impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PieceType::Pawn => write!(f, "Pawn"),
            PieceType::Knight => write!(f, "Knight"),
            PieceType::Bishop => write!(f, "Bishop"),
            PieceType::Rook => write!(f, "Rook"),
            PieceType::Queen => write!(f, "Queen"),
            PieceType::King => write!(f, "King"),
        }
    }
}

#[derive(Clone)]
pub enum Side {
    White,
    Black,
}
impl PartialEq for Side {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Side::White, Side::White) => true,
            (Side::Black, Side::Black) => true,
            _ => false,
        }
    }
}
impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Side::White => write!(f, "White"),
            Side::Black => write!(f, "Black"),
        }
    }
}

pub struct GameState {
    pieces: Pieces,
}

// Implementation of the GameState struct
impl GameState {
    pub fn new() -> Self {
        let pieces = vec![
            Piece {
                position: Position { x: 1, y: 1 },
                piece_type: PieceType::Rook,
                side: Side::White,
                has_moved: false,
            },
            Piece {
                position: Position { x: 2, y: 1 },
                piece_type: PieceType::Knight,
                side: Side::White,
                has_moved: false,
            },
            Piece {
                position: Position { x: 3, y: 1 },
                piece_type: PieceType::Bishop,
                side: Side::White,
                has_moved: false,
            },
            Piece {
                position: Position { x: 4, y: 1 },
                piece_type: PieceType::King,
                side: Side::White,
                has_moved: false,
            },
            Piece {
                position: Position { x: 5, y: 1 },
                piece_type: PieceType::Queen,
                side: Side::White,
                has_moved: false,
            },
            Piece {
                position: Position { x: 6, y: 1 },
                piece_type: PieceType::Bishop,
                side: Side::White,
                has_moved: false,
            },
            Piece {
                position: Position { x: 7, y: 1 },
                piece_type: PieceType::Knight,
                side: Side::White,
                has_moved: false,
            },
            Piece {
                position: Position { x: 8, y: 1 },
                piece_type: PieceType::Rook,
                side: Side::White,
                has_moved: false,
            },
            Piece {
                position: Position { x: 1, y: 2 },
                piece_type: PieceType::Pawn,
                side: Side::White,
                has_moved: false,
            },
            Piece {
                position: Position { x: 2, y: 2 },
                piece_type: PieceType::Pawn,
                side: Side::White,
                has_moved: false,
            },
            Piece {
                position: Position { x: 3, y: 2 },
                piece_type: PieceType::Pawn,
                side: Side::White,
                has_moved: false,
            },
            Piece {
                position: Position { x: 4, y: 2 },
                piece_type: PieceType::Pawn,
                side: Side::White,
                has_moved: false,
            },
            Piece {
                position: Position { x: 5, y: 2 },
                piece_type: PieceType::Pawn,
                side: Side::White,
                has_moved: false,
            },
            Piece {
                position: Position { x: 6, y: 2 },
                piece_type: PieceType::Pawn,
                side: Side::White,
                has_moved: false,
            },
            Piece {
                position: Position { x: 7, y: 2 },
                piece_type: PieceType::Pawn,
                side: Side::White,
                has_moved: false,
            },
            Piece {
                position: Position { x: 8, y: 2 },
                piece_type: PieceType::Pawn,
                side: Side::White,
                has_moved: false,
            },
            Piece {
                position: Position { x: 1, y: 8 },
                piece_type: PieceType::Rook,
                side: Side::Black,
                has_moved: false,
            },
            Piece {
                position: Position { x: 2, y: 8 },
                piece_type: PieceType::Knight,
                side: Side::Black,
                has_moved: false,
            },
            Piece {
                position: Position { x: 3, y: 8 },
                piece_type: PieceType::Bishop,
                side: Side::Black,
                has_moved: false,
            },
            Piece {
                position: Position { x: 4, y: 8 },
                piece_type: PieceType::King,
                side: Side::Black,
                has_moved: false,
            },
            Piece {
                position: Position { x: 5, y: 8 },
                piece_type: PieceType::Queen,
                side: Side::Black,
                has_moved: false,
            },
            Piece {
                position: Position { x: 6, y: 8 },
                piece_type: PieceType::Bishop,
                side: Side::Black,
                has_moved: false,
            },
            Piece {
                position: Position { x: 7, y: 8 },
                piece_type: PieceType::Knight,
                side: Side::Black,
                has_moved: false,
            },
            Piece {
                position: Position { x: 8, y: 8 },
                piece_type: PieceType::Rook,
                side: Side::Black,
                has_moved: false,
            },
            Piece {
                position: Position { x: 1, y: 7 },
                piece_type: PieceType::Pawn,
                side: Side::Black,
                has_moved: false,
            },
            Piece {
                position: Position { x: 2, y: 7 },
                piece_type: PieceType::Pawn,
                side: Side::Black,
                has_moved: false,
            },
            Piece {
                position: Position { x: 3, y: 7 },
                piece_type: PieceType::Pawn,
                side: Side::Black,
                has_moved: false,
            },
            Piece {
                position: Position { x: 4, y: 7 },
                piece_type: PieceType::Pawn,
                side: Side::Black,
                has_moved: false,
            },
            Piece {
                position: Position { x: 5, y: 7 },
                piece_type: PieceType::Pawn,
                side: Side::Black,
                has_moved: false,
            },
            Piece {
                position: Position { x: 6, y: 7 },
                piece_type: PieceType::Pawn,
                side: Side::Black,
                has_moved: false,
            },
            Piece {
                position: Position { x: 7, y: 7 },
                piece_type: PieceType::Pawn,
                side: Side::Black,
                has_moved: false,
            },
            Piece {
                position: Position { x: 8, y: 7 },
                piece_type: PieceType::Pawn,
                side: Side::Black,
                has_moved: false,
            },
        ];

        GameState { pieces }
    }

    pub fn get_piece(&self, x: i32, y: i32) -> Piece {
        let piece = self
            .pieces
            .iter()
            .find(|p| p.position.x == x && p.position.y == y);
        match piece {
            Some(p) => p.clone(),
            None => Piece {
                position: Position { x: 0, y: 0 },
                piece_type: PieceType::Pawn,
                side: Side::White,
                has_moved: false,
            },
        }
    }
    pub fn pos_has_piece(&self, x: i32, y: i32) -> bool {
        self.pieces
            .iter()
            .any(|p| p.position.x == x && p.position.y == y)
    }

    pub fn update_piece(&mut self, x_piece: i32, y_piece: i32, new_x: i32, new_y: i32) {
        if let Some(piece) = self
            .pieces
            .iter_mut()
            .find(|p| p.position.x == x_piece && p.position.y == y_piece)
        {
            piece.position.x = new_x;
            piece.position.y = new_y;
            piece.has_moved = true;
        }
    }

    pub fn remove_piece(&mut self, x_piece: i32, y_piece: i32) {
        if let Some(index) = self
            .pieces
            .iter()
            .position(|p| p.position.x == x_piece && p.position.y == y_piece)
        {
            self.pieces.remove(index);
        }
    }

    pub fn validate_move(&self, piece: Piece, new_x: i32, new_y: i32) -> bool {
        let inside_board = |x: i32, y: i32| x > 0 && x < 9 && y > 0 && y < 9;
        let x_moved = new_x - piece.position.x.abs();
        let x_moved_abs = x_moved.abs();
        let y_moved = new_y - piece.position.y.abs();
        let y_moved_abs = y_moved.abs();

        if x_moved == 0 && y_moved == 0 {
            return false;
        }

        if !inside_board(piece.position.x, piece.position.y) || !inside_board(new_x, new_y) {
            return false;
        }

        match piece.piece_type {
            PieceType::King => {
                if (x_moved_abs == 1 && y_moved_abs == 1)
                    || (x_moved_abs == 1 && y_moved_abs == 0)
                    || (x_moved_abs == 0 && y_moved_abs == 1)
                {
                    return true;
                }
                false
            }
            PieceType::Queen => {
                if x_moved_abs > 0 && y_moved_abs > 0 {
                    if x_moved_abs != y_moved_abs {
                        return false;
                    }
                    let x_is_positive = x_moved > 0;
                    for i in 1..y_moved {
                        if x_is_positive {
                            if self.pos_has_piece(piece.position.x + i.abs(), piece.position.y + i)
                            {
                                return false;
                            }
                        } else {
                            if self.pos_has_piece(piece.position.x - i.abs(), piece.position.y + i)
                            {
                                return false;
                            }
                        }
                    }
                } else {
                    for i in 1..x_moved {
                        if self.pos_has_piece(piece.position.x + i, piece.position.y) {
                            return false;
                        }
                    }
                    for i in 1..y_moved {
                        if self.pos_has_piece(piece.position.x, piece.position.y + i) {
                            return false;
                        }
                    }
                }
                true
            }
            PieceType::Bishop => {
                if x_moved_abs == 0 || y_moved_abs == 0 {
                    return false;
                }
                for i in 1..x_moved {
                    if self.pos_has_piece(piece.position.x + i, piece.position.y) {
                        return false;
                    }
                }
                for i in 1..y_moved {
                    if self.pos_has_piece(piece.position.x, piece.position.y + i) {
                        return false;
                    }
                }
                true
            }
            PieceType::Knight => {
                if (x_moved_abs == 2 && y_moved_abs == 1) || (x_moved_abs == 1 && y_moved_abs == 2)
                {
                    return true;
                }
                false
            }
            PieceType::Rook => {
                if x_moved_abs > 0 && y_moved_abs > 0 {
                    return false;
                }
                for i in 1..x_moved {
                    if self.pos_has_piece(piece.position.x + i, piece.position.y) {
                        return false;
                    }
                }
                for i in 1..y_moved {
                    if self.pos_has_piece(piece.position.x, piece.position.y + i) {
                        return false;
                    }
                }
                true
            }
            PieceType::Pawn => {
                if y_moved_abs > 2 {
                    return false;
                }
                if !piece.has_moved {
                    if y_moved_abs == 2 && x_moved_abs == 0 {
                        if self.pos_has_piece(new_x, new_y) {
                            return false;
                        }
                        return true;
                    }
                }

                if x_moved_abs == 0 {
                    if self.pos_has_piece(new_x, new_y) {
                        return false;
                    }
                    return true;
                }
                if self.pos_has_piece(new_x, new_y) {
                    return true;
                }
                false
            }
        }
    }

    pub fn move_piece(&mut self, x_piece: i32, y_piece: i32, new_x: i32, new_y: i32) -> String {
        let piece = self.get_piece(x_piece, y_piece);
        *SIDE_IS_PLAYING.lock().unwrap() = match piece.side {
            Side::White => Side::Black,
            Side::Black => Side::White,
        };
        if self.pos_has_piece(new_x, new_y) {
            self.remove_piece(new_x, new_y);
            let output = format!(
                "Moved {} to {}{} while hitting {}\r\n{}'s turn",
                piece.piece_type,
                match new_x {
                    1 => "a",
                    2 => "b",
                    3 => "c",
                    4 => "d",
                    5 => "e",
                    6 => "f",
                    7 => "g",
                    8 => "h",
                    _ => "?",
                },
                new_y,
                self.get_piece(new_x, new_y).piece_type,
                if *SIDE_IS_PLAYING.lock().unwrap() == Side::Black {
                    Side::Black
                } else {
                    Side::White
                }
            );
            self.update_piece(x_piece, y_piece, new_x, new_y);
            return output;
        }
        self.update_piece(x_piece, y_piece, new_x, new_y);
        format!(
            "Moved {} to {}{}\r\n{}'s turn, which piece would you like to move?",
            piece.piece_type,
            match new_x {
                1 => "a",
                2 => "b",
                3 => "c",
                4 => "d",
                5 => "e",
                6 => "f",
                7 => "g",
                8 => "h",
                _ => "?",
            },
            new_y,
            if *SIDE_IS_PLAYING.lock().unwrap() == Side::Black {
                Side::Black
            } else {
                Side::White
            }
        )
    }

    pub fn is_in_check(&self, king_x: i32, king_y: i32, side: Side) -> bool {
        for piece in self.pieces.iter() {
            if piece.side == side {
                continue;
            }
            if self.validate_move(piece.clone(), king_x, king_y) {
                return true;
            }
        }
        false
    }
    pub fn side_is_checkmate(&self, side: Side) -> bool {
        // Find the king of the given side
        let king = self
            .pieces
            .iter()
            .find(|p| p.piece_type == PieceType::King && p.side == side)
            .unwrap();

        // Get the position of the king
        let king_x = king.position.x;
        let king_y = king.position.y;

        // Check if the king is in check
        if self.is_in_check(king_x, king_y, side.clone()) {
            // Check if the king has any legal moves to escape check
            for i in -1..=1 {
                for j in -1..=1 {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    let new_x = king_x + i;
                    let new_y = king_y + j;
                    if new_x < 1 || new_x > 8 || new_y < 1 || new_y > 8 {
                        continue;
                    }
                    // Check if the king can move to the new position legally
                    if self.validate_move(king.clone(), new_x, new_y) {
                        // If there's a legal move, it's not checkmate
                        if self.is_in_check(new_x, new_y, side.clone()) == false {
                            return false;
                        }
                    }
                }
            }
            // If there are no legal moves, it's checkmate
            return true;
        }

        // If the king is not in check, it's not checkmate
        false
    }

    pub fn print_chessboard(&self) {
        // clear the console
        print!("\x1B[2J\x1B[1;1H");
        let mut board = vec![];
        board.push("  | a  | b  | c  | d  | e  | f  | g  | h  |".to_string());
        board.push("--|---------------------------------------|".to_string());
        for i in 1..9 {
            let mut row = format!("{} |", i);
            for j in 1..9 {
                if self.pos_has_piece(j, i) {
                    let piece = self.get_piece(j, i);
                    row += &format!(
                        " {}{} |",
                        match piece.side {
                            Side::White => "w",
                            Side::Black => "b",
                        },
                        match piece.piece_type {
                            PieceType::King => "K",
                            PieceType::Queen => "Q",
                            PieceType::Bishop => "B",
                            PieceType::Knight => "N",
                            PieceType::Rook => "R",
                            PieceType::Pawn => "P",
                        }
                    );
                } else {
                    row += "    |";
                }
            }
            board.push(row);
        }
        for row in board.iter().rev() {
            // print row raw without ""
            println!("{}", row);
        }
    }
}

lazy_static! {
    static ref GAME_STATE: Mutex<GameState> = Mutex::new(GameState::new());
    static ref SIDE_IS_PLAYING: Mutex<Side> = Mutex::new(Side::White);
    static ref POSITION_CURRENT: Mutex<Position> = Mutex::new(Position { x: 0, y: 0 });
    static ref POSITION_NEW: Mutex<Position> = Mutex::new(Position { x: 0, y: 0 });
}

pub fn handle_partial_move(move_string: String) -> bool {
    let mut position_current = POSITION_CURRENT.lock().unwrap();
    let mut position_new = POSITION_NEW.lock().unwrap();
    let mut game_state = GAME_STATE.lock().unwrap();
    let x = move_string.chars().nth(0).unwrap();
    let y = move_string.chars().nth(1).unwrap();

    // convert y string to i32
    let mut y_number = 0;
    match y {
        '1' => y_number = 1,
        '2' => y_number = 2,
        '3' => y_number = 3,
        '4' => y_number = 4,
        '5' => y_number = 5,
        '6' => y_number = 6,
        '7' => y_number = 7,
        '8' => y_number = 8,
        _ => {}
    }

    let mut x_number = 0;
    match x {
        'a' => x_number = 1,
        'b' => x_number = 2,
        'c' => x_number = 3,
        'd' => x_number = 4,
        'e' => x_number = 5,
        'f' => x_number = 6,
        'g' => x_number = 7,
        'h' => x_number = 8,
        _ => {}
    }

    if (x_number < 1 || x_number > 8) || (y_number < 1 || y_number > 8) {
        println!("Invalid move, please try again");
        return false;
    }

    if position_current.x == 0 && position_current.y == 0 {
        *position_current = Position {
            x: x_number,
            y: y_number,
        };
        let piece = game_state.get_piece(position_current.x, position_current.y);
        if piece.position.x == 0 && piece.position.y == 0 {
            println!("No piece found at that position, please try again");
            *position_current = Position { x: 0, y: 0 };
            *position_new = Position { x: 0, y: 0 };
            return false;
        }
        println!("Where would you like to move the piece?");
        return false;
    }
    *position_new = Position {
        x: x_number,
        y: y_number,
    };
    let piece = game_state.get_piece(position_current.x, position_current.y);
    let valid_move = game_state.validate_move(piece.clone(), position_new.x, position_new.y);

    if !valid_move {
        println!("Invalid move, please try again, which piece would you like to move?");
        *position_current = Position { x: 0, y: 0 };
        *position_new = Position { x: 0, y: 0 };
        return false;
    }
    let moved = game_state.move_piece(
        position_current.x,
        position_current.y,
        position_new.x,
        position_new.y,
    );
    game_state.print_chessboard();
    println!("{}", moved);
    *position_current = Position { x: 0, y: 0 };
    *position_new = Position { x: 0, y: 0 };
    true
}

pub fn side_is_checkmate() -> bool {
    let game_state = GAME_STATE.lock().unwrap();
    game_state.side_is_checkmate(if *SIDE_IS_PLAYING.lock().unwrap() == Side::Black {
        Side::Black
    } else {
        Side::White
    })
}
