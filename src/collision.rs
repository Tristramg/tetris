use bevy::math::{Vec2, Vec3};
const E: f32 = 1.0;
#[derive(Debug)]
pub struct Bounds {
    min: Vec2,
    max: Vec2,
}

// Negative value means an overlap
// On one axis given two segments [a, b] and [c, d]
// How far away are those segments, or how much do they overlap?
fn distance(a: f32, b: f32, c: f32, d: f32) -> f32 {
    // 0.0, 30.0, 20.0, 40.0)

    if b <= c {
        // [a-b c-d] distance
        c - b
    } else if d <= a {
        // [c-d a-b] distance
        a - d
    } else if c <= a && b <= d {
        // [c a-b d] overlap
        a - b
    } else if c <= a && d <= b {
        // [c a d b] overlap
        a - d
    } else if a <= c && b <= d {
        // [a c b d] overlap
        c - b
    } else {
        0.0
    }
}

impl Bounds {
    pub fn from_pos_size(pos: Vec3, size: Vec2) -> Self {
        Self {
            min: pos.truncate() - size / 2.0,
            max: pos.truncate() + size / 2.0,
        }
    }

    pub fn x_distance(&self, other: &Self) -> f32 {
        distance(self.min.x(), self.max.x(), other.min.x(), other.max.x())
    }

    pub fn y_distance(&self, other: &Self) -> f32 {
        distance(self.min.y(), self.max.y(), other.min.y(), other.max.y())
    }

    pub fn left(&self, other: &Self) -> bool {
        (other.max.x() - self.min.x()).abs() < E && self.y_distance(other) < -E
    }

    pub fn right(&self, other: &Self) -> bool {
        (other.min.x() - self.max.x()).abs() < E && self.y_distance(other) < -E
    }

    pub fn bottom(&self, other: &Self) -> bool {
        (other.max.y() - self.min.y()).abs() < E && self.x_distance(other) < -E
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_distance() {
        assert_eq!(distance(0.0, 10.0, 20.0, 30.0), 10.0);
        assert_eq!(distance(0.0, 30.0, 20.0, 40.0), -10.0);
    }
    #[test]
    fn test_left() {
        let a = Bounds {
            min: Vec2::new(10.0, 0.0),
            max: Vec2::new(30.0, 10.0),
        };
        let b = Bounds {
            min: Vec2::new(0.0, 0.0),
            max: Vec2::new(10.0, 10.0),
        };
        assert!(a.left(&b));
        assert!(!b.left(&a));
    }

    #[test]
    fn test_right() {
        let a = Bounds {
            min: Vec2::new(10.0, 0.0),
            max: Vec2::new(30.0, 10.0),
        };
        let b = Bounds {
            min: Vec2::new(0.0, 0.0),
            max: Vec2::new(10.0, 10.0),
        };
        assert!(!b.left(&a));
    }

    #[test]
    fn test_bottom() {
        let a = Bounds {
            min: Vec2::new(0.0, -20.0),
            max: Vec2::new(10.0, -10.0),
        };
        let b = Bounds {
            min: Vec2::new(0.0, -30.0),
            max: Vec2::new(10.0, -20.0),
        };
        assert!(a.bottom(&b));
    }
}
