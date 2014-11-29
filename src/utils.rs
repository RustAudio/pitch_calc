
use std::num::Int;

/// The modulo function (handles negatives differently to Rust's remainder `%` operator).
#[inline]
pub fn modulo<I: Int>(a: I, b: I) -> I {
    match a % b {
        r if (r > Int::zero() && b < Int::zero())
          || (r < Int::zero() && b > Int::zero()) => (r + b),
        r                                         => r,
    }
}

