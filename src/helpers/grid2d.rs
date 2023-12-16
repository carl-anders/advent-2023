#![allow(dead_code)]
use std::ops::{Add, AddAssign, Sub, SubAssign};

use ndarray::Array2;
use num_traits::{One, WrappingAdd, WrappingSub};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position2D<T> {
    pub x: T,
    pub y: T,
}
impl<T: Copy> Position2D<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    pub const fn new_xy(xy: (T, T)) -> Self {
        Self { x: xy.0, y: xy.1 }
    }
    pub const fn new_yx(yx: (T, T)) -> Self {
        Self { x: yx.1, y: yx.0 }
    }
    pub const fn xy(&self) -> (T, T) {
        (self.x, self.y)
    }
    pub const fn yx(&self) -> (T, T) {
        (self.y, self.x)
    }
}
impl<T: Copy + Add<Output = T> + Sub<Output = T>> Position2D<T> {
    pub fn add_x(&self, x: T) -> Self {
        Self {
            x: self.x + x,
            y: self.y,
        }
    }
    pub fn add_y(&self, y: T) -> Self {
        Self {
            x: self.x,
            y: self.y + y,
        }
    }
}
impl<T: Copy + Sub<Output = T>> Position2D<T> {
    pub fn sub_x(&self, x: T) -> Self {
        Self {
            x: self.x - x,
            y: self.y,
        }
    }
    pub fn sub_y(&self, y: T) -> Self {
        Self {
            x: self.x,
            y: self.y - y,
        }
    }
}
impl<T: Copy + WrappingSub> Position2D<T> {
    pub fn wrapping_sub_x(&self, x: &T) -> Self {
        Self {
            x: self.x.wrapping_sub(x),
            y: self.y,
        }
    }
    pub fn wrapping_sub_y(&self, y: &T) -> Self {
        Self {
            x: self.x,
            y: self.y.wrapping_sub(y),
        }
    }
}
impl<T: Copy + WrappingAdd> Position2D<T> {
    pub fn wrapping_add_x(&self, x: &T) -> Self {
        Self {
            x: self.x.wrapping_add(x),
            y: self.y,
        }
    }
    pub fn wrapping_add_y(&self, y: &T) -> Self {
        Self {
            x: self.x,
            y: self.y.wrapping_add(y),
        }
    }
}

impl<T: Copy + PartialOrd + Sub<Output = T> + Add<Output = T>> Position2D<T> {
    pub fn manhattan(&self, other: &Self) -> T {
        (if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        }) + (if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        })
    }
}

impl<T: Add<Output = T>> Add<Self> for Position2D<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl<T: AddAssign> AddAssign for Position2D<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: Sub<Output = T>> Sub<Self> for Position2D<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl<T: SubAssign> SubAssign for Position2D<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction8Way {
    N = 0,
    NE = 1,
    E = 2,
    SE = 3,
    S = 4,
    SW = 5,
    W = 6,
    NW = 7,
}
impl Direction8Way {
    pub const UP: Self = Self::N;
    pub const RIGHT: Self = Self::E;
    pub const DOWN: Self = Self::S;
    pub const LEFT: Self = Self::W;
    pub const EVERY: [Self; 8] = [
        Self::N,
        Self::NE,
        Self::E,
        Self::SE,
        Self::S,
        Self::SW,
        Self::W,
        Self::NW,
    ];
}
impl<T: Copy + One + WrappingAdd<Output = T> + WrappingSub<Output = T>> Add<Direction8Way>
    for Position2D<T>
{
    type Output = Self;
    fn add(self, other: Direction8Way) -> Self {
        match other {
            Direction8Way::N => self.wrapping_sub_y(&T::one()),
            Direction8Way::NE => self.wrapping_sub_y(&T::one()).wrapping_add_x(&T::one()),
            Direction8Way::E => self.wrapping_add_x(&T::one()),
            Direction8Way::SE => self.wrapping_add_y(&T::one()).wrapping_add_x(&T::one()),
            Direction8Way::S => self.wrapping_add_y(&T::one()),
            Direction8Way::SW => self.wrapping_add_y(&T::one()).wrapping_sub_x(&T::one()),
            Direction8Way::W => self.wrapping_sub_x(&T::one()),
            Direction8Way::NW => self.wrapping_sub_y(&T::one()).wrapping_sub_x(&T::one()),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction4Way {
    East = 0,
    South = 1,
    West = 2,
    North = 3,
}
impl Direction4Way {
    pub const EVERY: [Self; 4] = [Self::North, Self::East, Self::South, Self::West];
    pub const fn turn_right(self, times: usize) -> Self {
        match times % 4 {
            1 => match self {
                Self::East => Self::South,
                Self::South => Self::West,
                Self::West => Self::North,
                Self::North => Self::East,
            },
            2 => match self {
                Self::East => Self::West,
                Self::South => Self::North,
                Self::West => Self::East,
                Self::North => Self::South,
            },
            3 => match self {
                Self::East => Self::North,
                Self::South => Self::East,
                Self::West => Self::South,
                Self::North => Self::West,
            },
            _ => self,
        }
    }
}
impl<T: Copy + One + WrappingAdd<Output = T> + WrappingSub<Output = T>> Add<Direction4Way>
    for Position2D<T>
{
    type Output = Self;
    fn add(self, other: Direction4Way) -> Self {
        match other {
            Direction4Way::East => self.wrapping_add_x(&T::one()),
            Direction4Way::South => self.wrapping_add_y(&T::one()),
            Direction4Way::West => self.wrapping_sub_x(&T::one()),
            Direction4Way::North => self.wrapping_sub_y(&T::one()),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Turn {
    Right,
    Left,
}
impl Add<Turn> for Direction4Way {
    type Output = Self;
    fn add(self, other: Turn) -> Self {
        self.turn_right(if other == Turn::Right { 1 } else { 3 })
    }
}

pub fn parse_str_grid<T, F>(input: &str, mut mapper: F) -> Result<Array2<T>, std::io::Error>
where
    F: FnMut(char) -> T,
{
    let mut height = 0;
    let mut width = None;
    let mut grid = Vec::new();
    for line in input.lines() {
        height += 1;
        let mut line_width = 0;
        for char in line.chars() {
            grid.push(mapper(char));
            line_width += 1;
        }
        if width.is_none() {
            width = Some(line_width);
        } else if width.unwrap() != line_width {
            return Err(std::io::Error::other("Input not a rectangle"));
        }
    }
    Array2::from_shape_vec((height, width.unwrap_or(0)), grid)
        .map_err(|e| std::io::Error::other(format!("Array2 error: {e:?}")))
}
