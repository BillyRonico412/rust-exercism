#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    fn is_valid(&self) -> bool {
        0 <= self.0 && self.0 < 8 && 0 <= self.1 && self.1 < 8
    }
    pub fn new(col: i32, row: i32) -> Option<Self> {
        let chess_position = ChessPosition(col, row);
        if chess_position.is_valid() {
            Some(chess_position)
        } else {
            None
        }
    }
}

const QUEEN_ATTACKS: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let mut current_position = self.position;
        QUEEN_ATTACKS.iter().any(|&attack| {
            loop {
                current_position.0 += attack.0 as i32;
                current_position.1 += attack.1 as i32;
                if !current_position.is_valid() {
                    current_position = self.position;
                    break false;
                }
                if current_position == other.position {
                    break true;
                }
            }
        })
    }
}
