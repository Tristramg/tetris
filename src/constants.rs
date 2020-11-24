use bevy::prelude::Color;
use rand::prelude::Distribution;

#[derive(Clone)]
pub struct SquarePositions(pub [(isize, isize); 4]);
#[derive(Clone)]
pub struct Tetromino {
    pub orientations: [SquarePositions; 4],
    pub color: Color,
}

fn all_pieces() -> Vec<Tetromino> {
    vec![
        // T
        Tetromino {
            orientations: [
                SquarePositions([(0, 0), (1, 0), (2, 0), (1, 1)]),
                SquarePositions([(2, 0), (2, 1), (2, 2), (1, 1)]),
                SquarePositions([(2, 2), (1, 2), (0, 2), (1, 1)]),
                SquarePositions([(0, 2), (0, 1), (0, 0), (1, 1)]),
            ],
            color: Color::rgb(0.5, 0.5, 1.0),
        },
        // O
        Tetromino {
            orientations: [
                SquarePositions([(0, 0), (1, 0), (0, 1), (1, 1)]),
                SquarePositions([(0, 0), (1, 0), (0, 1), (1, 1)]),
                SquarePositions([(0, 0), (1, 0), (0, 1), (1, 1)]),
                SquarePositions([(0, 0), (1, 0), (0, 1), (1, 1)]),
            ],
            color: Color::rgb(1.0, 0.5, 0.5),
        },
        // I
        Tetromino {
            orientations: [
                SquarePositions([(0, 0), (0, 1), (0, 2), (0, 3)]),
                SquarePositions([(0, 0), (1, 0), (2, 0), (3, 0)]),
                SquarePositions([(1, 0), (1, 1), (1, 2), (1, 3)]),
                SquarePositions([(0, 1), (1, 1), (2, 1), (3, 1)]),
            ],
            color: Color::rgb(0.5, 1.0, 0.5),
        },
        // L
        Tetromino {
            orientations: [
                SquarePositions([(0, 0), (0, 1), (0, 2), (1, 2)]),
                SquarePositions([(0, 1), (1, 1), (2, 1), (2, 0)]),
                SquarePositions([(0, 0), (1, 0), (1, 1), (1, 2)]),
                SquarePositions([(0, 1), (0, 0), (1, 0), (2, 0)]),
            ],
            color: Color::rgb(1.0, 1.0, 0.5),
        },
        // J
        Tetromino {
            orientations: [
                SquarePositions([(0, 2), (1, 2), (1, 1), (1, 0)]),
                SquarePositions([(2, 2), (2, 1), (1, 1), (0, 1)]),
                SquarePositions([(1, 0), (0, 0), (0, 1), (0, 2)]),
                SquarePositions([(0, 0), (0, 1), (1, 1), (2, 1)]),
            ],
            color: Color::rgb(1.0, 0.5, 1.0),
        },
        // S
        Tetromino {
            orientations: [
                SquarePositions([(0, 1), (1, 1), (1, 0), (2, 0)]),
                SquarePositions([(1, 2), (1, 1), (0, 1), (0, 0)]),
                SquarePositions([(0, 1), (1, 1), (1, 0), (2, 0)]),
                SquarePositions([(1, 2), (1, 1), (0, 1), (0, 0)]),
            ],
            color: Color::rgb(0.5, 1.0, 1.0),
        },
        // Z
        Tetromino {
            orientations: [
                SquarePositions([(0, 0), (1, 0), (1, 1), (2, 1)]),
                SquarePositions([(1, 0), (1, 1), (0, 1), (0, 2)]),
                SquarePositions([(0, 0), (1, 0), (1, 1), (2, 1)]),
                SquarePositions([(1, 0), (1, 1), (0, 1), (0, 2)]),
            ],
            color: Color::rgb(0.75, 0.5, 1.0),
        },
    ]
}

pub fn rand_tetromino() -> Tetromino {
    let pieces = all_pieces();
    let mut rng = rand::thread_rng();
    let dist = rand::distributions::Uniform::new(0, pieces.len());
    pieces[dist.sample(&mut rng)].clone()
}
