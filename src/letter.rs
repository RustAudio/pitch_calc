
use self::Letter::{
    C, Csh, Db, D, Dsh, Eb, E, F, Fsh, Gb, G, Gsh, Ab, A, Ash, Bb, B
};
use std::num::{Float, FromPrimitive, ToPrimitive};
use utils::modulo;

pub const TOTAL_LETTERS: u8 = 12;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, RustcEncodable, RustcDecodable)]
pub enum Letter {
    C, Csh, Db, D, Dsh, Eb, E, F, Fsh, Gb, G, Gsh, Ab, A, Ash, Bb, B
}

impl FromPrimitive for Letter {
    fn from_i64(n: i64) -> Option<Letter> {
        match n {
            n if modulo(n, 12) == 0  => Some(C),
            n if modulo(n, 12) == 1  => Some(Csh),
            n if modulo(n, 12) == 2  => Some(D),
            n if modulo(n, 12) == 3  => Some(Dsh),
            n if modulo(n, 12) == 4  => Some(E),
            n if modulo(n, 12) == 5  => Some(F),
            n if modulo(n, 12) == 6  => Some(Fsh),
            n if modulo(n, 12) == 7  => Some(G),
            n if modulo(n, 12) == 8  => Some(Gsh),
            n if modulo(n, 12) == 9  => Some(A),
            n if modulo(n, 12) == 10 => Some(Ash),
            n if modulo(n, 12) == 11 => Some(B),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<Letter> {
        match n {
            n if modulo(n, 12) == 0  => Some(C),
            n if modulo(n, 12) == 1  => Some(Csh),
            n if modulo(n, 12) == 2  => Some(D),
            n if modulo(n, 12) == 3  => Some(Dsh),
            n if modulo(n, 12) == 4  => Some(E),
            n if modulo(n, 12) == 5  => Some(F),
            n if modulo(n, 12) == 6  => Some(Fsh),
            n if modulo(n, 12) == 7  => Some(G),
            n if modulo(n, 12) == 8  => Some(Gsh),
            n if modulo(n, 12) == 9  => Some(A),
            n if modulo(n, 12) == 10 => Some(Ash),
            n if modulo(n, 12) == 11 => Some(B),
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

impl<T: ToPrimitive> ToLetter for T {
    fn to_letter(&self) -> Letter {
        FromPrimitive::from_f64(self.to_f64().unwrap().round()).unwrap()
    }
}

