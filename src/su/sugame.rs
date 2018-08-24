#[allow(dead_code)]

extern crate serde_json;

use su::Board;

#[derive(Deserialize)]
struct RawBoard {
    data: [[i8; 9]; 9]
}

pub struct Game {
    board: Board,
}

impl Game {
    fn test_game() -> Option<Game> {
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
        let raw_board_struct = raw_board.clone();
        let serialized = serde_json::to_string(&raw_board).unwrap();
        println!("{}", serialized);

        match Board::new_from_array(raw_board) {
            Some(board) => return Some(
                Game{
                    board: board
                }
            ),
            None => return None
        }
    }
}

#[cfg(test)]
#[test]
fn test_this() {
    let game = Game::test_game();

    assert!(false);
}