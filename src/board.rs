#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GamePiece {
    pub color: PieceColor,
    pub crowned: bool,
}

impl GamePiece {

    /// new method creates a new instance of GamePiece.
    ///
    /// #Arguments
    ///
    /// color - a parameter of type PieceColor denoting color of piece.
    ///
    /// #Return
    ///
    /// Returns the instance of type GamePiece.
    pub fn new(color: PieceColor) -> GamePiece {
        GamePiece {
            color,
            crowned: false,
        }
    }

    /// crowned method make a game piece crowned.
    ///
    /// #Arguments
    ///
    /// piece - A parameter of type GamePiece that is to be crowned.
    ///
    /// #Return
    ///
    /// Returns the GamePiece that is crowned.
    pub fn crowned(piece: GamePiece) -> GamePiece {
        GamePiece {
            color: piece.color,
            crowned: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Coordinate(pub usize, pub usize);

impl Coordinate {

    /// on_board method checks if the piece is on the board.
    ///
    /// #Return
    ///
    /// Returns a bool value denoting if the piece is on the board.
    pub fn on_board(self) -> bool {
        let Coordinate(x_coord, y_coord) = self;
        x_coord <= 7 && y_coord <= 7
    }

    /// jump_targets_from method gives all the location to which a jump can be made.
    ///
    /// #Return
    ///
    /// Returns the iterator over a vector containing Coordinate type objects denoting targets for jump.
    pub fn jump_targets_from(&self) -> impl Iterator<Item = Coordinate> {
        let mut jumps = Vec::new();
        let Coordinate(x_coord, y_coord) = *self;
        if y_coord >= 2 {
            jumps.push(Coordinate(x_coord + 2, y_coord - 2));
        }
        jumps.push(Coordinate(x_coord + 2, y_coord + 2));

        if x_coord >= 2 && y_coord >= 2 {
            jumps.push(Coordinate(x_coord - 2, y_coord - 2));
        }
        if x_coord >= 2 {
            jumps.push(Coordinate(x_coord - 2, y_coord + 2));
        }
        jumps.into_iter()
    }

    /// jump_targets_from method gives all the location to which a move can be made.
    ///
    /// #Return
    ///
    /// Returns the iterator over a vector containing Coordinate type objects denoting targets for the move.
    pub fn move_targets_from(&self) -> impl Iterator<Item = Coordinate> {
        let mut moves = Vec::new();
        let Coordinate(x_coord, y_coord) = *self;
        if x_coord >= 1 {
            moves.push(Coordinate(x_coord - 1, y_coord + 1));
        }
        moves.push(Coordinate(x_coord + 1, y_coord + 1));
        if y_coord >= 1 {
            moves.push(Coordinate(x_coord + 1, y_coord - 1));
        }
        if x_coord >= 1 && y_coord >= 1 {
            moves.push(Coordinate(x_coord - 1, y_coord - 1));
        }
        moves.into_iter()
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Move {
    pub from: Coordinate,
    pub to: Coordinate,
}

impl Move {

    /// new method creates a new instance of Move.
    ///
    /// #Arguments
    ///
    /// from - a tuple containing the starting coordinates.
    /// to - a tuple containing the destination coordinates.
    ///
    /// #Return
    ///
    /// Returns the instance of type Move.
    pub fn new(from: (usize, usize), to: (usize, usize)) -> Move {
        Move {
            from: Coordinate(from.0, from.1),
            to: Coordinate(to.0, to.1),
        }
    }
}
