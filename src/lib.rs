//!
//!  pitch_calc
//!
//!
//!  Created by Mitchell Nordine at 11:26PM on November 02, 2014.
//!

pub use self::calc::{
    difference_in_semitones,
    hz_from_letter_octave,
    hz_from_mel,
    hz_from_perc,
    hz_from_scaled_perc,
    hz_from_step,
    letter_octave_from_hz,
    letter_octave_from_mel,
    letter_octave_from_perc,
    letter_octave_from_scaled_perc,
    letter_octave_from_step,
    mel_from_hz,
    mel_from_letter_octave,
    mel_from_perc,
    mel_from_scaled_perc,
    mel_from_step,
    perc_from_hz,
    perc_from_letter_octave,
    perc_from_mel,
    perc_from_scaled_perc,
    perc_from_step,
    scaled_perc_from_hz,
    scaled_perc_from_letter_octave,
    scaled_perc_from_mel,
    scaled_perc_from_perc,
    scaled_perc_from_step,
    step_from_hz,
    step_from_letter_octave,
    step_from_mel,
    step_from_perc,
    step_from_scaled_perc,
};
pub use self::hz::Hz;
pub use self::hz::MAX as MAX_HZ;
pub use self::hz::MIN as MIN_HZ;
pub use self::letter::{
    Letter,
    TOTAL_LETTERS,
};
pub use self::letter_octave::{
    Octave,
    LetterOctave,
};
pub use self::mel::Mel;
pub use self::perc::Perc;
pub use self::scaled_perc::{
    DEFAULT_SCALE_WEIGHT,
    ScaledPerc,
    ScaleWeight,
};
pub use self::step::Step;

pub mod calc;
pub mod hz;
pub mod letter;
pub mod letter_octave;
pub mod mel;
pub mod perc;
pub mod scaled_perc;
pub mod step;
pub mod utils;
pub mod convert;
