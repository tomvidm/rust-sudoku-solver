#![allow(dead_code)]

#[derive(Copy, Clone)]
pub struct Coord {
    row: i8,
    col: i8
}

impl Coord {
    fn new(row: i8, col: i8) -> Coord {
        Coord{
            row: row,
            col: col
        }
    }
    fn to_row(&self) -> i8 {self.row}
    fn to_col(&self) -> i8 {self.col}
    fn to_square(&self) -> i8 {
        let r_major = (self.row - self.row % 3) / 3;
        let c_major = (self.col - self.col % 3) / 3;
        3 * r_major + c_major
    }
}

#[derive(Copy, Clone)]
pub struct Move {
    coord: Coord,
    new_value: i8
}

impl Move {
    pub fn new(row: i8, col: i8, val: i8) -> Move {
        Move {
            coord: Coord::new(row, col),
            new_value: val
        }
    }

    pub fn new_from_coord_and_value(coord: Coord, val: i8) -> Move {
        Move {
            coord: coord,
            new_value: val
        }
    }
}

#[derive(Copy, Clone)]
struct Tile {
    value: i8
}

impl Tile {
    fn new(val: i8) -> Tile {
        Tile{value: val}
    }
}

pub struct Board {
    matrix: [[Tile; 9]; 9],
    allowed_in_row: [[bool; 9]; 9],
    allowed_in_col: [[bool; 9]; 9],
    allowed_in_square: [[bool; 9]; 9]
}

impl Board {
    pub fn new() -> Board {
        Board{
            matrix: [[Tile::new(0); 9]; 9], // Initialize all cells to 0
            allowed_in_row: [[true; 9]; 9],
            allowed_in_col: [[true; 9]; 9],
            allowed_in_square: [[true; 9]; 9]
        }
    }

    pub fn new_from_array(array: [[i8; 9]; 9]) -> Board {
        let mut board = Board::new();
        for r in 0..9 {
            for c in 0..9 {
                let val = array[r as usize][c as usize];
                let coord = Coord::new(r, c);
                let entry = Move::new(r, c, val);
                board.put_if_legal(entry);
            }
        }

        return board;
    }

    fn set_val(&mut self, coord: Coord, val: i8) {
        self.matrix[coord.to_row() as usize][coord.to_col() as usize] = Tile::new(val);
    }

    fn get_val(&self, coord: Coord) -> i8 {
        self.matrix[coord.to_row() as usize][coord.to_col() as usize].value
    }

    fn is_allowed_in_row(&self, row: i8, value: i8) -> bool {
        self.allowed_in_row[(row as usize)][(value as usize) - 1]
    }

    fn is_allowed_in_col(&self, col: i8, value: i8) -> bool {
        self.allowed_in_col[(col as usize)][(value as usize) - 1]
    }

    fn is_allowed_in_square(&self, square: i8, value: i8) -> bool {
        self.allowed_in_square[(square as usize)][(value as usize) - 1]
    }

    fn count_legal_values_at(&self, coord: Coord) -> i8 {
        let mut count: i8 = 0;
        for v in 1..=9 {
            let hypothetical_move = Move::new_from_coord_and_value(coord, v);
            if self.is_move_legal(hypothetical_move) {
                count += 1;
            }
        }
        return count
    }

    fn list_legal_moves_at(&self, coord: Coord) -> Vec<Move> {
        let mut result: Vec<Move> = Vec::new();
        for v in 1..=9 {
            let new_move = Move::new_from_coord_and_value(coord, v);
            if self.is_move_legal(new_move.clone()) {
                result.push(new_move);
            }
        }
        return result;
    }

    fn list_legal_moves(&self) -> Vec<Move> {
        let mut result: Vec<Move> = Vec::new();
        for r in 0..9 {
            for c in 0..9 {
                let coord = Coord::new(r, c);
                let mut legal_here = self.list_legal_moves_at(coord);
                result.append(&mut legal_here);
            }
        }
        return result;
    }

    fn is_move_legal(&self, attempt: Move) -> bool {
        if attempt.new_value == 0 {
            return true
        } else {
            self.is_allowed_in_row(attempt.coord.to_row(), attempt.new_value) &&
            self.is_allowed_in_col(attempt.coord.to_col(), attempt.new_value) &&
            self.is_allowed_in_square(attempt.coord.to_square(), attempt.new_value)
        }
    }

    fn allow(&mut self, coord: Coord, value: i8) {
        self.allowed_in_row[coord.to_row() as usize][(value as usize) - 1] = true;
        self.allowed_in_col[coord.to_col() as usize][(value as usize) - 1] = true;
        self.allowed_in_square[coord.to_square() as usize][(value as usize) - 1] = true;
    }

    fn disallow(&mut self, coord: Coord, value: i8) {
        self.allowed_in_row[coord.to_row() as usize][(value as usize) - 1] = false;
        self.allowed_in_col[coord.to_col() as usize][(value as usize) - 1] = false;
        self.allowed_in_square[coord.to_square() as usize][(value as usize) - 1] = false;
    }

    fn disallow_and_allow_old(&mut self, coord: Coord, value: i8) {
        if value > 0 {
            self.disallow(coord, value);
        }

        let old_value = self.get_val(coord);
        if old_value > 0 {
            let old_entry = Move::new_from_coord_and_value(coord, old_value);
            self.allow(coord, old_value);
        }
    }

    fn put_if_legal(&mut self, entry: Move) {
        if self.is_move_legal(entry) {
            self.disallow_and_allow_old(entry.coord, entry.new_value);
        }
    }
}

#[cfg(test)]
#[test]
fn test_valid_board() {
    let valid_raw_board: [[i8; 9]; 9] = [
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
    let mut board = Board::new_from_array(valid_raw_board);
}