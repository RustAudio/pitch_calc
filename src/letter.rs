use self::Letter::{
    C, Csh, Db, D, Dsh, Eb, E, F, Fsh, Gb, G, Gsh, Ab, A, Ash, Bb, B
};
use num::{FromPrimitive, ToPrimitive};
use num::PrimInt as Int;
use std::cmp::Ordering;
use utils::modulo;

pub const TOTAL_LETTERS: u8 = 12;

/// The letter representation for each step in the 12-tone, equal temperament, chromatic scale.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde_serialization", derive(Serialize, Deserialize))]
pub enum Letter {
    C, Csh, Db, D, Dsh, Eb, E, F, Fsh, Gb, G, Gsh, Ab, A, Ash, Bb, B
}


impl PartialOrd for Letter {
    fn partial_cmp(&self, other: &Letter) -> Option<Ordering> {
        let other_u8 = other.to_u8().unwrap();
        self.to_u8().unwrap().partial_cmp(&other_u8)
    }
}

impl Ord for Letter {
    fn cmp(&self, other: &Letter) -> Ordering {
        let other_u8 = other.to_u8().unwrap();
        self.to_u8().unwrap().cmp(&other_u8)
    }
}

impl PartialEq for Letter {
    fn eq(&self, other: &Letter) -> bool {
        self.to_u8().unwrap() == other.to_u8().unwrap()
    }
}

impl Eq for Letter {}


impl Letter {

    /// Returns whether or not the note would be a black key on a standard piano or keyboard.
    pub fn is_black_key(self) -> bool {
        use self::Letter::*;
        match self {
            Csh | Db | Dsh | Eb | Fsh | Gb | Gsh | Ab | Ash | Bb => true,
            C | D | E | F | G | A | B => false,
        }
    }

}


impl FromPrimitive for Letter {
    fn from_i64(n: i64) -> Option<Letter> {
        match modulo(n, 12) {
            0  => Some(C),
            1  => Some(Csh),
            2  => Some(D),
            3  => Some(Dsh),
            4  => Some(E),
            5  => Some(F),
            6  => Some(Fsh),
            7  => Some(G),
            8  => Some(Gsh),
            9  => Some(A),
            10 => Some(Ash),
            11 => Some(B),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<Letter> {
        match modulo(n, 12) {
            0  => Some(C),
            1  => Some(Csh),
            2  => Some(D),
            3  => Some(Dsh),
            4  => Some(E),
            5  => Some(F),
            6  => Some(Fsh),
            7  => Some(G),
            8  => Some(Gsh),
            9  => Some(A),
            10 => Some(Ash),
            11 => Some(B),
            _ => None,
        }
    }
}

impl ToPrimitive for Letter {
    fn to_i64(&self) -> Option<i64> {
        match *self {
            C        => Some(0),
            Csh | Db => Some(1),
            D        => Some(2),
            Dsh | Eb => Some(3),
            E        => Some(4),
            F        => Some(5),
            Fsh | Gb => Some(6),
            G        => Some(7),
            Gsh | Ab => Some(8),
            A        => Some(9),
            Ash | Bb => Some(10),
            B        => Some(11),
        }
    }
    fn to_u64(&self) -> Option<u64> {
        match *self {
            C        => Some(0),
            Csh | Db => Some(1),
            D        => Some(2),
            Dsh | Eb => Some(3),
            E        => Some(4),
            F        => Some(5),
            Fsh | Gb => Some(6),
            G        => Some(7),
            Gsh | Ab => Some(8),
            A        => Some(9),
            Ash | Bb => Some(10),
            B        => Some(11),
        }
    }
}

impl ::rand::Rand for Letter {
    fn rand<R: ::rand::Rng>(rng: &mut R) -> Letter {
        rng.gen_range(0, 12).to_letter()
    }
}

/// A trait to be implemented for all primitives for easy conversion to the Letter type.
pub trait ToLetter {
    /// Cast a primitive type to a Letter.
    fn to_letter(&self) -> Letter;
}

impl<T: Int> ToLetter for T {
    fn to_letter(&self) -> Letter {
        FromPrimitive::from_i64(self.to_i64().unwrap()).unwrap()
    }
}


impl<T: Int> ::std::ops::Add<T> for Letter {
    type Output = Letter;
    fn add(self, rhs: T) -> Letter {
        let semitones = modulo(rhs.to_i64().unwrap(), 12).to_i16().unwrap();
        FromPrimitive::from_i16(modulo(self.to_i16().unwrap() + semitones, 12)).unwrap()
    }
}

impl<T: Int> ::std::ops::Sub<T> for Letter {
    type Output = Letter;
    fn sub(self, rhs: T) -> Letter {
        let semitones = modulo(rhs.to_i64().unwrap(), 12).to_i16().unwrap();
        FromPrimitive::from_i16(modulo(self.to_i16().unwrap() - semitones, 12)).unwrap()
    }
}

impl ::std::ops::Add for Letter {
    type Output = Letter;
    fn add(self, rhs: Letter) -> Letter {
        self + rhs.to_u8().unwrap()
    }
}

impl ::std::ops::Sub for Letter {
    type Output = Letter;
    fn sub(self, rhs: Letter) -> Letter {
        self - rhs.to_i16().unwrap()
    }
}

