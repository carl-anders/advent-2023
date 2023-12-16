#![allow(dead_code)]
pub mod grid2d;

use std::ops::{Range, RangeBounds};
pub trait BorrowTwo<T> {
    fn borrow_two(&mut self, a: usize, b: usize) -> (&mut T, &mut T);
}

impl<T> BorrowTwo<T> for [T] {
    fn borrow_two(&mut self, a: usize, b: usize) -> (&mut T, &mut T) {
        assert!(a < self.len() && b < self.len());
        assert!(a != b);
        if a < b {
            if let [first, .., second] = &mut self[a..=b] {
                return (first, second);
            }
        } else if let [second, .., first] = &mut self[b..=a] {
            return (first, second);
        }
        panic!()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LongBitArr<const SIZE: usize> {
    data: [u64; SIZE],
}
impl<const SIZE: usize> LongBitArr<SIZE> {
    const _SIZE_CHECK: () = assert!(!(SIZE == 0), "Size can't be 0");
    #[allow(clippy::let_unit_value)]
    pub const fn new() -> Self {
        let () = Self::_SIZE_CHECK;
        Self { data: [0; SIZE] }
    }
    pub fn get(&self, index: usize) -> bool {
        assert!(index < SIZE * 64);
        unsafe { (self.data.get_unchecked(index / 64) >> (index % 64)) & 1 != 0 }
    }
    pub fn set(&mut self, index: usize) {
        assert!(index < SIZE * 64);
        unsafe {
            *self.data.get_unchecked_mut(index / 64) |= 1 << (index % 64);
        }
    }
    pub fn clear(&mut self, index: usize) {
        assert!(index < SIZE * 64);
        unsafe {
            *self.data.get_unchecked_mut(index / 64) &= !(1 << (index % 64));
        }
    }
    pub fn count_ones(&self) -> usize {
        self.data.iter().map(|d| d.count_ones() as usize).sum()
    }
}

pub struct BitIterator<T>(T);
macro_rules! impl_BitIterator {
    ($($t:ty),+) => {
        $(impl Iterator for BitIterator<$t> {
            type Item = $t;
            fn next(&mut self) -> Option<Self::Item> {
                if self.0 == 0 {
                    None
                } else {
                    let next = self.0.trailing_zeros();
                    self.0 ^= 1 << next;
                    Some(next as $t)
                }
            }
        })*
    }
}
impl_BitIterator!(usize, u8, u16, u32, u64, u128);

pub trait IntoBitIterator {
    type Item;
    fn into_bit_iter(self) -> BitIterator<Self::Item>;
}
macro_rules! impl_IntoBitIterator {
    ($($t:ty),+) => {
        $(impl IntoBitIterator for $t {
            type Item = $t;
            fn into_bit_iter(self) -> BitIterator<Self::Item> {
                BitIterator(self)
            }
        })*
    }
}
impl_IntoBitIterator!(usize, u8, u16, u32, u64, u128);

pub trait BitArray {
    fn get(&self, index: Self) -> bool;
    fn set(&mut self, index: Self);
    fn clear(&mut self, index: Self);
}
macro_rules! impl_BitArray {
    ($($t:ty),+) => {
        $(impl BitArray for $t {
            fn get(&self, index: Self) -> bool {
                (self >> index) & 1 != 0
            }
            fn set(&mut self, index: Self) {
                *self |= 1 << index;
            }
            fn clear(&mut self, index: Self) {
                *self &= !(1 << index);
            }
        })*
    }
}
impl_BitArray!(usize, u8, u16, u32, u64, u128);

pub trait RangeIntersect<T: Ord, U: RangeBounds<T>>: RangeBounds<T> {
    fn intersect(&self, other: &U) -> Option<U>;
}
impl<T: Ord + Copy> RangeIntersect<T, Self> for Range<T> {
    fn intersect(&self, other: &Self) -> Option<Self> {
        if self.end < other.start || other.end < self.start {
            None
        } else {
            Some(self.start.max(other.start)..self.end.min(other.end))
        }
    }
}

#[derive(Debug, Clone)]
pub struct FirstAndLast<T> {
    first: Option<T>,
    last: Option<T>
}
impl<T: Clone> FirstAndLast<T> {
    pub const fn new() -> Self {
        Self { first: None,
        last: None }
    }
    pub fn push(&mut self, item: T) {
        if self.first.is_none() {
            self.first = Some(item.clone());
        }
        self.last = Some(item);
    }
    pub fn get(self) -> Option<(T, T)> {
        if let Some(first) = self.first {
            if let Some(last) = self.last {
                return Some((first, last));
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct MergedRange<T> {
    ranges: Vec<Range<T>>,
}
impl<T: Ord + Copy> MergedRange<T> {
    pub const fn new() -> Self {
        Self { ranges: vec![] }
    }
    pub fn add(&mut self, other: Range<T>) {
        self.ranges.push(other);
        if self.ranges.len() > 1 {
            self.simplify();
        }
    }
    fn simplify(&mut self) {
        self.ranges.sort_unstable_by(|a, b| a.start.cmp(&b.start));
        'outer: loop {
            for index in 0..self.ranges.len() - 1 {
                let (a, b) = self.ranges.borrow_two(index, index + 1);
                if a.end >= b.start && a.start <= b.end {
                    *a = a.start.min(b.start)..a.end.max(b.end);
                    self.ranges.remove(index + 1);
                    continue 'outer;
                }
            }
            break;
        }
    }
    pub fn ranges(&self) -> Vec<Range<T>> {
        self.ranges.clone()
    }
}
impl<T: Ord + Copy> FromIterator<Range<T>> for MergedRange<T> {
    fn from_iter<Q>(iter: Q) -> Self
    where
        Q: IntoIterator<Item = Range<T>>,
    {
        let mut new = Self::new();
        for range in iter {
            new.add(range);
        }
        new
    }
}
