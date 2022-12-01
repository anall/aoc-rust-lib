use std::ops;

#[derive(Debug, Default)]
pub struct Point2D(pub i64, pub i64);
impl Point2D {
    #[must_use]
    pub fn new() -> Point2D {
        Point2D(0, 0)
    }

    #[must_use]
    pub fn manhattan_distance(&self) -> i64 {
        self.0.abs() + self.1.abs()
    }
}

impl_op_ex!(+ |a : &Point2D, b : &Point2D| -> Point2D { Point2D( a.0 + b.0, a.1 + b.1 ) } );
impl_op_ex!(*|a: &Point2D, b: &i64| -> Point2D { Point2D(a.0 * b, a.1 * b) });
impl_op_ex!(+= |a : &mut Point2D, b : &Point2D| { a.0 += b.0; a.1 += b.1; } );

#[derive(Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    #[must_use]
    pub fn delta(&self, mul: i64) -> Point2D {
        match self {
            Direction::North => Point2D(0, mul),
            Direction::East => Point2D(mul, 0),
            Direction::South => Point2D(0, -mul),
            Direction::West => Point2D(-mul, 0),
        }
    }

    fn from_idx(dir: i32) -> Self {
        match dir {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => panic!("got illegal direction {}", dir),
        }
    }

    fn to_idx(&self) -> i32 {
        match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    }

    #[must_use]
    pub fn rotate_left(&self, by: i32) -> Direction {
        assert_eq!(by % 90, 0);
        let rot = by / 90;
        Self::from_idx((self.to_idx() - rot + 4) % 4)
    }

    #[must_use]
    pub fn rotate_right(&self, by: i32) -> Direction {
        assert_eq!(by % 90, 0);
        let rot = by / 90;
        Self::from_idx((self.to_idx() + rot + 4) % 4)
    }
}
