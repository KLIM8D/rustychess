use crate::error::Error;
use std::fmt;
use std::mem::transmute;
use std::str::FromStr;

/// Describe a rank (row) on a chess board
#[derive(Eq, Copy, Clone, PartialEq, PartialOrd, Debug, Hash)]
pub enum Rank {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

/// How many ranks are there?
pub const NUM_RANKS: usize = 8;

/// Enumerate all ranks
pub const ALL_RANKS: [Rank; NUM_RANKS] = [
    Rank::A,
    Rank::B,
    Rank::C,
    Rank::D,
    Rank::E,
    Rank::F,
    Rank::G,
    Rank::H,
];

impl Rank {
    /// Convert a `usize` into a `Rank` (the inverse of to_index).  If i > 7, wrap around.
    #[inline]
    pub fn from_index(i: usize) -> Rank {
        unsafe { transmute((i as u8) & 7) }
    }

    /// Go one rank to the left.  If impossible, wrap around.
    #[inline]
    pub fn left(&self) -> Rank {
        Rank::from_index(self.to_index().wrapping_sub(1))
    }

    /// Go one rank to the right.  If impossible, wrap around.
    #[inline]
    pub fn right(&self) -> Rank {
        Rank::from_index(self.to_index() + 1)
    }

    /// Convert this `Rank` into a `usize` from 0 to 7 inclusive.
    #[inline]
    pub fn to_index(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub fn to_str(&self) -> &str {
        match self {
            Rank::A => "a",
            Rank::B => "b",
            Rank::C => "c",
            Rank::D => "d",
            Rank::E => "e",
            Rank::F => "f",
            Rank::G => "g",
            Rank::H => "h",
        }
    }

    pub fn sub(self, other: Self) -> usize {
        let mut r = 0;
        let mut v = self.clone();
        while v != other {
            r += 1;
            if self < other {
                v = v.right()
            } else {
                v = v.left()
            }
        }
        r
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

impl FromStr for Rank {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 1 {
            return Err(Error::InvalidRank);
        }
        match s.to_lowercase().chars().next().unwrap() {
            'a' => Ok(Rank::A),
            'b' => Ok(Rank::B),
            'c' => Ok(Rank::C),
            'd' => Ok(Rank::D),
            'e' => Ok(Rank::E),
            'f' => Ok(Rank::F),
            'g' => Ok(Rank::G),
            'h' => Ok(Rank::H),
            _ => Err(Error::InvalidRank),
        }
    }
}
