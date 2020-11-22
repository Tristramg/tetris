use rand::prelude::Distribution;

#[derive(Clone)]
pub struct SquarePositions(pub [(isize, isize); 4]);
#[derive(Clone)]
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

pub const O: Tetromino = Tetromino {
    orientations: [
        SquarePositions([(0, 0), (1, 0), (0, 1), (1, 1)]),
        SquarePositions([(0, 0), (1, 0), (0, 1), (1, 1)]),
        SquarePositions([(0, 0), (1, 0), (0, 1), (1, 1)]),
        SquarePositions([(0, 0), (1, 0), (0, 1), (1, 1)]),
    ],
};

pub const I: Tetromino = Tetromino {
    orientations: [
        SquarePositions([(0, 0), (0, 1), (0, 2), (0, 3)]),
        SquarePositions([(0, 0), (1, 0), (2, 0), (3, 0)]),
        SquarePositions([(1, 0), (1, 1), (1, 2), (1, 3)]),
        SquarePositions([(0, 1), (1, 1), (2, 1), (3, 1)]),
    ],
};

pub const L: Tetromino = Tetromino {
    orientations: [
        SquarePositions([(0, 0), (0, 1), (0, 2), (1, 2)]),
        SquarePositions([(0, 1), (1, 1), (2, 1), (2, 0)]),
        SquarePositions([(0, 0), (1, 0), (1, 1), (1, 2)]),
        SquarePositions([(0, 1), (0, 0), (1, 0), (2, 0)]),
    ],
};

pub const J: Tetromino = Tetromino {
    orientations: [
        SquarePositions([(0, 2), (1, 2), (1, 1), (1, 0)]),
        SquarePositions([(2, 2), (2, 1), (1, 1), (0, 1)]),
        SquarePositions([(1, 0), (0, 0), (0, 1), (0, 2)]),
        SquarePositions([(0, 0), (0, 1), (1, 1), (2, 1)]),
    ],
};

pub const S: Tetromino = Tetromino {
    orientations: [
        SquarePositions([(0, 1), (1, 1), (1, 0), (2, 0)]),
        SquarePositions([(1, 2), (1, 1), (0, 1), (0, 0)]),
        SquarePositions([(0, 1), (1, 1), (1, 0), (2, 0)]),
        SquarePositions([(1, 2), (1, 1), (0, 1), (0, 0)]),
    ],
};

pub const Z: Tetromino = Tetromino {
    orientations: [
        SquarePositions([(0, 0), (1, 0), (1, 1), (2, 1)]),
        SquarePositions([(1, 0), (1, 1), (0, 1), (0, 2)]),
        SquarePositions([(0, 0), (1, 0), (1, 1), (2, 1)]),
        SquarePositions([(1, 0), (1, 1), (0, 1), (0, 2)]),
    ],
};

pub const PIECES: [Tetromino; 7] = [T, O, I, L, J, S, Z];

pub fn rand_tetromino() -> Tetromino {
    let mut rng = rand::thread_rng();
    let dist = rand::distributions::Uniform::new(0, PIECES.len());
    PIECES[dist.sample(&mut rng)].clone()
}
