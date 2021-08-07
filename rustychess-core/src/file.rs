use crate::error::Error;
use std::fmt;
use std::mem::transmute;
use std::ops::Add;
use std::str::FromStr;

/// Describe a file (column) on a chess board
#[derive(Eq, Copy, Clone, PartialEq, PartialOrd, Debug, Hash)]
pub enum File {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}

/// How many ranks are there?
pub const NUM_FILES: usize = 8;

/// Enumerate all ranks
pub const ALL_FILES: [File; NUM_FILES] = [
    File::First,
    File::Second,
    File::Third,
    File::Fourth,
    File::Fifth,
    File::Sixth,
    File::Seventh,
    File::Eighth,
];

impl File {
    /// Convert a `usize` into a `File` (the inverse of to_index).  If the number is > 7, wrap
    /// around.
    #[inline]
    pub fn from_index(i: usize) -> File {
        unsafe { transmute((i as u8) & 7) }
    }

    /// Go one rank down.  If impossible, wrap around.
    #[inline]
    pub fn down(&self) -> File {
        File::from_index(self.to_index().wrapping_sub(1))
    }

    /// Go one file up.  If impossible, wrap around.
    #[inline]
    pub fn up(&self) -> File {
        File::from_index(self.to_index() + 1)
    }

    /// Convert this `File` into a `usize` between 0 and 7 (inclusive).
    #[inline]
    pub fn to_index(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub fn to_i8(&self) -> i8 {
        match self {
            &File::First => 1,
            &File::Second => 2,
            &File::Third => 3,
            &File::Fourth => 4,
            &File::Fifth => 5,
            &File::Sixth => 6,
            &File::Seventh => 7,
            &File::Eighth => 8,
        }
    }

    pub fn from_i8(s: i8) -> Result<Self, Error> {
        match s {
            1 => Ok(File::First),
            2 => Ok(File::Second),
            3 => Ok(File::Third),
            4 => Ok(File::Fourth),
            5 => Ok(File::Fifth),
            6 => Ok(File::Sixth),
            7 => Ok(File::Seventh),
            8 => Ok(File::Eighth),
            _ => Err(Error::InvalidFile),
        }
    }

    pub fn sub(self, other: Self) -> usize {
        let mut r = 0;
        let mut v = self.clone();
        while v != other {
            r += 1;
            if self < other {
                v = v.up()
            } else {
                v = v.down()
            }
        }
        r
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

impl FromStr for File {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 1 {
            return Err(Error::InvalidFile);
        }
        match s.chars().next().unwrap() {
            '1' => Ok(File::First),
            '2' => Ok(File::Second),
            '3' => Ok(File::Third),
            '4' => Ok(File::Fourth),
            '5' => Ok(File::Fifth),
            '6' => Ok(File::Sixth),
            '7' => Ok(File::Seventh),
            '8' => Ok(File::Eighth),
            _ => Err(Error::InvalidFile),
        }
    }
}
