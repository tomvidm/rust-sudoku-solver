use su::Board;

pub struct Game {
    board: Board,
}

impl Game {
    fn test_game() -> Game {
        let raw_board: [[i8; 9]; 9] = [
            [1, 0, 0,   0, 0, 0,   0, 0, 0],
            [0, 2, 0,   0, 0, 0,   0, 0, 0],
            [0, 0, 3,   0, 0, 0,   0, 0, 0],

            [0, 0, 0,   4, 0, 0,   0, 0, 0],
            [0, 0, 0,   0, 5, 0,   0, 0, 0],
            [0, 0, 0,   0, 0, 6,   0, 0, 0],

            [0, 0, 0,   0, 0, 0,   7, 0, 0],
            [0, 0, 0,   0, 0, 0,   0, 8, 0],
            [0, 0, 0,   0, 0, 0,   0, 0, 9]
        ];

        Game{
            board: Board::new_from_array(raw_board)
        }
    }
}