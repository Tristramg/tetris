pub const STEP: f32 = 50.0;
pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;

pub struct SquarePositions(pub [(u8, u8); 4]);
pub struct Tetromino {
    pub orientations: [SquarePositions; 4],
}

pub const T: Tetromino = Tetromino {
    orientations: [
        SquarePositions([(0, 0), (1, 0), (2, 0), (1, 1)]),
        SquarePositions([(2, 0), (2, 1), (2, 2), (1, 1)]),
        SquarePositions([(2, 2), (1, 2), (0, 2), (1, 1)]),
        SquarePositions([(0, 2), (0, 1), (0, 0), (1, 1)]),
    ],
};
