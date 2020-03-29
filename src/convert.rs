use super::{Hz, LetterOctave, Mel, Perc, ScaledPerc, Step};
use crate::calc::Hz as CalcHz;
use std::convert::From;

/// Implement a single From<T> using the passed expression
macro_rules! impl_from {
    ($FromType:ty, $ToType:ty, $id:ident => $conv:expr) => {
        impl From<$FromType> for $ToType {
            fn from($id: $FromType) -> Self {
                $conv
            }
        }
    };

    ($FromType:ty, $ToType:ty, $member:ident) => {
        impl_from!($FromType, $ToType, other => other.$member());
    };
}

/// Implement all type pairs using their respective conversion functions
macro_rules! impl_all_pairs {
    ( $member:ident => $To:ty ) => { };

    ( $head_member:ident => $Head:ty, $( $tail_member:ident => $Tail:ty ),* )
        => {
        $(
            impl_from!($Head, $Tail, $tail_member);
            impl_from!($Tail, $Head, $head_member);
        )*

        impl_all_pairs!($($tail_member => $Tail),*);
    }
}

// Implement From<T> for all fully defined pitch types
impl_all_pairs!(
    to_hz => Hz,
    to_mel => Mel,
    to_letter_octave => LetterOctave,
    to_scaled_perc => ScaledPerc,
    to_perc => Perc,
    to_step => Step
    );

// Additionally implement From for calc::Hz = f32
impl_from!(CalcHz, Hz, other => Hz(other));

#[cfg(test)]
mod tests {
    use super::super::*;
    use std::convert::Into;

    fn into_test_gen<T: Into<Hz>>(val: T) -> Hz {
        val.into()
    }

    #[test]
    fn conversion() {
        let lo = LetterOctave(Letter::A, 4);
        assert!(Hz::from(lo) == Hz(440.0));
    }

    #[test]
    fn function_call() {
        let lo = LetterOctave(Letter::A, 4);
        assert!(into_test_gen(lo) == Hz(440.0));
    }
}
