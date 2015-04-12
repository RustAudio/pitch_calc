
use num::PrimInt as Int;

/// The modulo function (handles negatives differently to Rust's remainder `%` operator).
#[inline]
pub fn modulo<I: Int>(a: I, b: I) -> I {
    match a % b {
        r if (r > I::zero() && b < I::zero())
          || (r < I::zero() && b > I::zero()) => (r + b),
        r                                     => r,
    }
}

