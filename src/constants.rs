pub struct SquarePositions(pub [(isize, isize); 4]);
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
