//!
//!  pitch_calc
//!
//!
//!  Created by Mitchell Nordine at 11:26PM on November 02, 2014.
//!

#![feature(core)]

extern crate rand;
extern crate "rustc-serialize" as rustc_serialize;

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
};
pub use self::hz::Hz;
pub use self::hz::MAX as MAX_HZ;
pub use self::hz::MIN as MIN_HZ;
pub use self::perc::Perc;
pub use self::scaled_perc::{
    DEFAULT_SCALE_WEIGHT,
    ScaledPerc,
    ScaleWeight,
};
pub use self::step::Step;

pub mod calc;
pub mod letter;
pub mod letter_octave;
pub mod hz;
pub mod perc;
pub mod scaled_perc;
pub mod step;
pub mod utils;

