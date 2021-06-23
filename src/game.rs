use super::board::{Coordinate, GamePiece, Move, PieceColor};

pub struct GameEngine {
    board: [[Option<GamePiece>; 8]; 8],
    current_turn: PieceColor,
    move_count: u32,
}

pub struct MoveResult {
    pub move_made: Move,
    pub crowned: bool,
}

impl GameEngine {
    /// new method creates a new instance of GameEngine with default values.
    ///
    /// #Return
    ///
    /// Returns the instance of type GameEngine.
    pub fn new() -> GameEngine {
        let mut engine = GameEngine {
            board: [[None; 8]; 8],
            current_turn: PieceColor::Black,
            move_count: 0,
        };
        engine.initialize_pieces();
        engine
    }

    /// initialize_pieces method initialises the pieces on the board.
    pub fn initialize_pieces(&mut self) {
        [1, 3, 5, 7, 0, 2, 4, 6, 1, 3, 5, 7]
            .iter()
            .zip([0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2].iter())
            .map(|(x_coord, y_coord)| (*x_coord as usize, *y_coord as usize))
            .for_each(|(x_coord, y_coord)| {
                self.board[x_coord][y_coord] = Some(GamePiece::new(PieceColor::White));
            });

        [0, 2, 4, 6, 1, 3, 5, 7, 0, 2, 4, 6]
            .iter()
            .zip([5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7].iter())
            .map(|(x_coord, y_coord)| (*x_coord as usize, *y_coord as usize))
            .for_each(|(x_coord, y_coord)| {
                self.board[x_coord][y_coord] = Some(GamePiece::new(PieceColor::Black));
            });
    }

    /// move_piece method make the move desired by user.
    ///
    /// #Arguments
    ///
    /// move_desired - a reference of type Move which holds the move to be made.
    ///
    /// #Return
    ///
    /// Returns the instance of type MoveResult denoting the result.
    pub fn move_piece(&mut self, move_desired: &Move) -> Result<MoveResult, ()> {
        let legal_moves = self.legal_moves();

        if !legal_moves.contains(move_desired) {
            return Err(());
        }

        let Coordinate(from_x, from_y) = move_desired.from;
        let Coordinate(to_x, to_y) = move_desired.to;
        let piece = self.board[from_x][from_y].unwrap();
        let midpiece_coordinate = self.midpiece_coordinate(from_x, from_y, to_x, to_y);
        if let Some(Coordinate(x, y)) = midpiece_coordinate {
            self.board[x][y] = None; // remove the jumped piece
        }

        // Move piece from source to destination
        self.board[to_x][to_y] = Some(piece);
        self.board[from_x][from_y] = None;

        let crowned = if self.should_crown(piece, move_desired.to) {
            self.crown_piece(move_desired.to);
            true
        } else {
            false
        };
        self.advance_turn();

        Ok(MoveResult {
            move_made: move_desired.clone(),
            crowned,
        })
    }

    /// get_piece method gives the piece from a given location on the board.
    ///
    /// #Arguments
    ///
    /// coord - a object of type Coordinate denoting the location to be fetched.
    ///
    /// #Return
    ///
    /// Returns the Result type value containing the GamePiece wrapped in Option enum.
    pub fn get_piece(&self, coord: Coordinate) -> Result<Option<GamePiece>, ()> {
        let Coordinate(coord_x, coord_y) = coord;
        if coord_x <= 7 && coord_y <= 7 {
            Ok(self.board[coord_x][coord_y])
        } else {
            Err(())
        }
    }

    /// current_turn method tells the player who has current turn.
    ///
    /// #Return
    ///
    /// Returns an PieceColor enum containing the color of current player's pieces.
    pub fn current_turn(&self) -> PieceColor {
        self.current_turn
    }

    /// advance_turn method toggles the current turn of players.
    fn advance_turn(&mut self) {
        if self.current_turn == PieceColor::Black {
            self.current_turn = PieceColor::White
        } else {
            self.current_turn = PieceColor::Black
        }
        self.move_count += 1;
    }

    /// should_crown method checks if the piece should crown or not.
    ///
    /// #Arguments
    ///
    /// piece - A GamePiece type object that is to be checked.
    /// coord - A Coordinate type object specifying the location of piece.
    ///
    /// #Return
    ///
    /// Returns bool value denoting if the piece should crown.
    fn should_crown(&self, piece: GamePiece, coord: Coordinate) -> bool {
        let Coordinate(_coord_x, coord_y) = coord;

        (coord_y == 0 && piece.color == PieceColor::Black)
            || (coord_y == 7 && piece.color == PieceColor::White)
    }

    /// crown_piece method crowns a given piece on the board.
    ///
    /// #Arguments
    ///
    /// coord - A Coordinate type object denoting the location whose piece id to be crowned.
    ///
    /// #Return
    ///
    /// Returns a bool value telling if the piece is crowned succesfully.
    fn crown_piece(&mut self, coord: Coordinate) -> bool {
        let Coordinate(coord_x, coord_y) = coord;
        if let Some(piece) = self.board[coord_x][coord_y] {
            self.board[coord_x][coord_y] = Some(GamePiece::crowned(piece));
            true
        } else {
            false
        }
    }

    /// is_crowned method checks if the piece is crowned or not.
    ///
    /// #Arguments
    ///
    /// coord - A Coordinate type object denoting location of piece to be checked.
    ///
    /// #Return
    ///
    /// Returns a bool value denoting if the piece is crowned or not.
    pub fn is_crowned(&self, coord: Coordinate) -> bool {
        let Coordinate(coord_x, coord_y) = coord;
        match self.board[coord_x][coord_y] {
            Some(piece) => piece.crowned,
            None => false,
        }
    }

    /// move_count method tells the number of moves made.
    ///
    /// #Return
    ///
    /// Returns an u32 value denoting the count of moves.
    pub fn move_count(&self) -> u32 {
        self.move_count
    }

    /// legal_moves method gives all the legal moves for all locations on the board.
    ///
    /// #Return
    ///
    /// Returns vector containing the legal moves.
    fn legal_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        for col in 0..8 {
            for row in 0..8 {
                if let Some(piece) = self.board[col][row] {
                    if piece.color == self.current_turn {
                        let loc = Coordinate(col, row);
                        let mut vmoves = self.valid_moves_from(loc);
                        moves.append(&mut vmoves);
                    }
                }
            }
        }

        moves
    }

    /// valid_moves_from method gives all the valid moves from a particular location on the board.
    ///
    /// #Arguments
    ///
    /// loc - A Coordinate type object denoting location of piece whose valid moves are desired.
    ///
    /// #Return
    ///
    /// Returns the vector of valid moves.
    fn valid_moves_from(&self, loc: Coordinate) -> Vec<Move> {
        let Coordinate(x, y) = loc;
        if let Some(piece) = self.board[x][y] {
            let mut jumps = loc
                .jump_targets_from()
                .filter(|coord| self.valid_jump(&piece, &loc, &coord))
                .map(|ref coord| Move {
                    from: loc.clone(),
                    to: coord.clone(),
                })
                .collect::<Vec<Move>>();
            let mut moves = loc
                .move_targets_from()
                .filter(|coord| self.valid_move(&piece, &loc, &coord))
                .map(|ref coord| Move {
                    from: loc.clone(),
                    to: coord.clone(),
                })
                .collect::<Vec<Move>>();
            jumps.append(&mut moves);
            jumps
        } else {
            Vec::new()
        }
    }

    /// midpiece_coordinate method gives the location of piece in between the jump.
    ///
    /// #Arguments
    ///
    /// from_x - an usize parameter for x coordinate of starting location.
    /// from_y - an usize parameter for y coordinate of starting location.
    /// to_x - an usize parameter for x coordinate of final location.
    /// to_y - an usize parameter for y coordinate of final location.
    ///
    /// #Return
    ///
    /// Returns the Coordinate of mid piece wrapped in Option.
    fn midpiece_coordinate(
        &self,
        from_x: usize,
        from_y: usize,
        to_x: usize,
        to_y: usize,
    ) -> Option<Coordinate> {
        if to_x == from_x + 2 && to_y == from_y + 2 {
            Some(Coordinate(from_x + 1, from_y + 1))
        } else if from_x >= 2 && from_y >= 2 && to_x == from_x - 2 && to_y == from_y - 2 {
            Some(Coordinate(from_x - 1, from_y - 1))
        } else if from_x >= 2 && to_x == from_x - 2 && to_y == from_y + 2 {
            Some(Coordinate(from_x - 1, from_y + 1))
        } else if from_y >= 2 && to_x == from_x + 2 && to_y == from_y - 2 {
            Some(Coordinate(from_x + 1, from_y - 1))
        } else {
            None
        }
    }

    /// midpiece method gives the piece in between the jump.
    ///
    /// #Arguments
    ///
    /// from_x - an usize parameter for x coordinate of starting location.
    /// from_y - an usize parameter for y coordinate of starting location.
    /// to_x - an usize parameter for x coordinate of final location.
    /// to_y - an usize parameter for y coordinate of final location.
    ///
    /// #Return
    ///
    /// Returns the GamePiece in middle wrapped in Option.
    fn midpiece(
        &self,
        from_x: usize,
        from_y: usize,
        to_x: usize,
        to_y: usize,
    ) -> Option<GamePiece> {
        match self.midpiece_coordinate(from_x, from_y, to_x, to_y) {
            Some(Coordinate(coord_x, coord_y)) => self.board[coord_x][coord_y],
            None => None,
        }
    }

    /// valid_jump method check if the jump is valid from a location on board to other.
    ///
    /// #Arguments
    ///
    /// moving_piece - A GamePiece type reference for the Piece making the jump.
    /// from - A Coordinate type reference denoting starting location.
    /// to - A Coordinate type reference denoting final location.
    ///
    /// #Return
    ///
    /// Returns a bool value telling if the jump is valid.
    fn valid_jump(&self, moving_piece: &GamePiece, from: &Coordinate, to: &Coordinate) -> bool {
        if !to.on_board() || !from.on_board() {
            false
        } else {
            let Coordinate(from_x, from_y) = *from;
            let Coordinate(to_x, to_y) = *to;

            let midpiece = self.midpiece(from_x, from_y, to_x, to_y);
            match midpiece {
                Some(piece) if piece.color != moving_piece.color => true,
                _ => false,
            }
        }
    }

    /// valid_move method check if the move is valid from a location on board to other.
    ///
    /// #Arguments
    ///
    /// moving_piece - A GamePiece type reference for the Piece making the move.
    /// from - A Coordinate type reference denoting starting location.
    /// to - A Coordinate type reference denoting final location.
    ///
    /// #Return
    ///
    /// Returns a bool value telling if the move is valid.
    fn valid_move(&self, moving_piece: &GamePiece, from: &Coordinate, to: &Coordinate) -> bool {
        if !to.on_board() || !from.on_board() {
            false
        } else {
            let Coordinate(to_x, to_y) = *to;
            if let Some(_piece) = self.board[to_x][to_y] {
                false
            } else {
                let Coordinate(_from_x, from_y) = *from;
                let mut valid = false;
                if to_y > from_y && moving_piece.color == PieceColor::White {
                    // white moves down
                    valid = true;
                }
                if to_y < from_y && moving_piece.color == PieceColor::Black {
                    // black moves up
                    valid = true;
                }
                if to_y > from_y && moving_piece.color == PieceColor::Black && moving_piece.crowned
                {
                    // crowned black move down
                    valid = true;
                }
                if to_y < from_y && moving_piece.color == PieceColor::White && moving_piece.crowned
                {
                    // crowned white move up
                    valid = true;
                }
                valid
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::board::{Coordinate, GamePiece, Move, PieceColor};
    use super::GameEngine;

    #[test]
    fn should_crown_success() {
        let engine = GameEngine::new();
        let black = GamePiece::new(PieceColor::Black);
        let res = engine.should_crown(black, Coordinate(3, 0));
        assert!(res);
    }

    #[test]
    fn should_crown_failure() {
        let engine = GameEngine::new();
        let black = GamePiece::new(PieceColor::Black);
        let res_no_crown = engine.should_crown(black, Coordinate(5, 2));
        assert_eq!(res_no_crown, false);
    }

    #[test]
    fn crown_success() {
        let mut engine = GameEngine::new();
        engine.initialize_pieces();
        let crowned = engine.crown_piece(Coordinate(1, 0));
        assert!(crowned);
    }

    #[test]
    fn crown_failure() {
        let mut engine = GameEngine::new();
        engine.initialize_pieces();
        let crowned = engine.crown_piece(Coordinate(2, 0));
        assert_eq!(crowned, false);
    }

    #[test]
    fn is_crown_success() {
        let mut engine = GameEngine::new();
        engine.initialize_pieces();
        let crowned = engine.crown_piece(Coordinate(1, 0));
        assert!(crowned);
        assert!(engine.is_crowned(Coordinate(1, 0)));
    }

    #[test]
    fn is_crown_failure() {
        let mut engine = GameEngine::new();
        engine.initialize_pieces();
        assert_eq!(engine.is_crowned(Coordinate(1, 0)), false);
    }

    #[test]
    fn advance_turn() {
        let mut engine = GameEngine::new();
        engine.advance_turn();
        assert_eq!(engine.current_turn(), PieceColor::White);
    }

    #[test]
    fn move_count_success() {
        let mut engine = GameEngine::new();
        engine.advance_turn();
        engine.advance_turn();
        assert_eq!(engine.move_count(), 2);
    }

    #[test]
    fn move_targets_success() {
        let coord = Coordinate(0, 5);
        let targets = coord.move_targets_from().collect::<Vec<Coordinate>>();
        assert_eq!(targets, [Coordinate(1, 6), Coordinate(1, 4)]);
    }

    #[test]
    fn valid_from_success() {
        let coord_1 = Coordinate(0, 5);
        let coord_2 = Coordinate(2, 5);

        let mut engine = GameEngine::new();
        engine.initialize_pieces();
        let move_1 = engine.valid_moves_from(coord_1);
        let move_2 = engine.valid_moves_from(coord_2);
        assert_eq!(
            move_1,
            [Move {
                from: Coordinate(0, 5),
                to: Coordinate(1, 4),
            }]
        );
        assert_eq!(
            move_2,
            [
                Move {
                    from: Coordinate(2, 5),
                    to: Coordinate(3, 4),
                },
                Move {
                    from: Coordinate(2, 5),
                    to: Coordinate(1, 4),
                }
            ]
        );
    }

    #[test]
    fn valid_from_failure() {
        let coord = Coordinate(1, 0);
        let mut engine = GameEngine::new();
        engine.initialize_pieces();
        let move_res = engine.valid_moves_from(coord);
        assert_eq!(move_res, vec![]);
    }

    #[test]
    fn legal_moves_black_success() {
        let mut engine = GameEngine::new();
        engine.initialize_pieces();
        let moves = engine.legal_moves();
        assert_eq!(
            moves,
            [
                Move {
                    from: Coordinate(0, 5),
                    to: Coordinate(1, 4),
                },
                Move {
                    from: Coordinate(2, 5),
                    to: Coordinate(3, 4),
                },
                Move {
                    from: Coordinate(2, 5),
                    to: Coordinate(1, 4),
                },
                Move {
                    from: Coordinate(4, 5),
                    to: Coordinate(5, 4),
                },
                Move {
                    from: Coordinate(4, 5),
                    to: Coordinate(3, 4),
                },
                Move {
                    from: Coordinate(6, 5),
                    to: Coordinate(7, 4),
                },
                Move {
                    from: Coordinate(6, 5),
                    to: Coordinate(5, 4),
                }
            ]
        );
    }

    #[test]
    fn legal_moves_white_success() {
        let mut engine = GameEngine::new();
        engine.initialize_pieces();
        engine.advance_turn();
        let moves = engine.legal_moves();
        assert_eq!(
            moves,
            [
                Move {
                    from: Coordinate(1, 2),
                    to: Coordinate(0, 3),
                },
                Move {
                    from: Coordinate(1, 2),
                    to: Coordinate(2, 3),
                },
                Move {
                    from: Coordinate(3, 2),
                    to: Coordinate(2, 3),
                },
                Move {
                    from: Coordinate(3, 2),
                    to: Coordinate(4, 3),
                },
                Move {
                    from: Coordinate(5, 2),
                    to: Coordinate(4, 3),
                },
                Move {
                    from: Coordinate(5, 2),
                    to: Coordinate(6, 3),
                },
                Move {
                    from: Coordinate(7, 2),
                    to: Coordinate(6, 3),
                }
            ]
        );
    }

    #[test]
    fn jump_targets_success() {
        let coord = Coordinate(3, 3);
        let targets = coord.jump_targets_from().collect::<Vec<Coordinate>>();
        assert_eq!(
            targets,
            [
                Coordinate(5, 1),
                Coordinate(5, 5),
                Coordinate(1, 1),
                Coordinate(1, 5)
            ]
        );
    }

    #[test]
    fn jump_moves_validation_success() {
        let mut engine = GameEngine::new();
        engine.initialize_pieces();
        engine.board[1][4] = Some(GamePiece::new(PieceColor::White));
        let moves = engine.legal_moves();
        assert_eq!(
            moves,
            [
                Move {
                    from: Coordinate(0, 5),
                    to: Coordinate(2, 3),
                },
                Move {
                    from: Coordinate(2, 5),
                    to: Coordinate(0, 3)
                },
                Move {
                    from: Coordinate(2, 5),
                    to: Coordinate(3, 4)
                },
                Move {
                    from: Coordinate(4, 5),
                    to: Coordinate(5, 4)
                },
                Move {
                    from: Coordinate(4, 5),
                    to: Coordinate(3, 4)
                },
                Move {
                    from: Coordinate(6, 5),
                    to: Coordinate(7, 4)
                },
                Move {
                    from: Coordinate(6, 5),
                    to: Coordinate(5, 4)
                }
            ]
        );
    }

    #[test]
    fn test_basic_move_success() {
        let mut engine = GameEngine::new();
        engine.initialize_pieces();
        let res = engine.move_piece(&Move::new((0, 5), (1, 4)));
        assert!(res.is_ok());

        let old = engine.board[0][5];
        let new = engine.board[1][4];
        assert_eq!(old, None);
        assert_eq!(
            new,
            Some(GamePiece {
                color: PieceColor::Black,
                crowned: false
            })
        );

        // fail to perform illegal move
        let res = engine.move_piece(&Move::new((1, 4), (2, 4))); // can't move horiz
        assert!(!res.is_ok());
        assert_eq!(engine.board[2][4], None);
    }
}
