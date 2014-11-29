//!
//!  pitch_calc
//!
//!
//!  Created by Mitchell Nordine at 11:26PM on November 02, 2014.
//!

#![feature(macro_rules)]

extern crate serialize;

pub use self::calc::{
    difference_in_semitones,
    hz_from_letter_octave,
    hz_from_perc,
    hz_from_scaled_perc,
    hz_from_step,
    letter_octave_from_hz,
    letter_octave_from_perc,
    letter_octave_from_scaled_perc,
    letter_octave_from_step,
    perc_from_hz,
    perc_from_letter_octave,
    perc_from_scaled_perc,
    perc_from_step,
    scaled_perc_from_hz,
    scaled_perc_from_letter_octave,
    scaled_perc_from_perc,
    scaled_perc_from_step,
    step_from_hz,
    step_from_letter_octave,
    step_from_perc,
    step_from_scaled_perc,
};
pub use self::letter::{
    Letter,
    TOTAL_LETTERS,
};
pub use self::letter_octave::{
    Octave,
    LetterOctave,
    ToLetterOctave,
};
pub use self::hz::{
    Hz,
    ToHz
};
pub use self::hz::MAX as MAX_HZ;
pub use self::hz::MIN as MIN_HZ;
pub use self::perc::{
    Perc,
    ToPerc,
};
pub use self::scaled_perc::{
    ScaledPerc,
    ScaleWeight,
    ToScaledPerc,
};
pub use self::step::{
    Step,
    ToStep,
};

pub mod calc;
pub mod letter;
pub mod letter_octave;
pub mod hz;
pub mod perc;
pub mod scaled_perc;
pub mod step;
pub mod utils;

/// To be implemented for types that can convert between different Pitch units.
pub trait Pitch: ToHz + ToLetterOctave + ToPerc + ToScaledPerc + ToStep {}
impl Pitch for Hz {}
impl Pitch for LetterOctave {}
impl Pitch for Perc {}
impl Pitch for ScaledPerc {}
impl Pitch for Step {}

#[macro_export]
/// A macro to simplify implementation of the Pitch trait.
macro_rules! impl_pitch(
    ($P:ty, $($pitch:ident).*) => (
        impl ::pitch::Pitch for $P { }

        impl ::pitch::ToHz for $P {
            #[inline]
            fn to_hz(&self) -> ::pitch::Hz {
                self$(.$pitch)*.to_hz()
            }
        }

        impl ::pitch::ToLetterOctave for $P {
            #[inline]
            fn to_letter_octave(&self) -> ::pitch::LetterOctave {
                self$(.$pitch)*.to_letter_octave()
            }
        }

        impl ::pitch::ToPerc for $P {
            #[inline]
            fn to_perc(&self) -> ::pitch::Perc {
                self$(.$pitch)*.to_perc()
            }
        }

        impl ::pitch::ToScaledPerc for $P {
            #[inline]
            fn to_scaled_perc_with_weight(&self, weight: ::pitch::ScaleWeight) -> ::pitch::ScaledPerc {
                self$(.$pitch)*.to_scaled_perc_with_weight(weight)
            }
        }

        impl ::pitch::ToStep for $P {
            #[inline]
            fn to_step(&self) -> ::pitch::Step {
                self$(.$pitch)*.to_step()
            }
        }
    );
)

