
use num::{
    Float,
    FromPrimitive,
    ToPrimitive,
};
use super::{
    Letter,
    MAX_HZ,
    MIN_HZ,
    Octave,
    TOTAL_LETTERS
};
use utils::modulo;

/// Useful for conversions between Step and Hz.
const TWELFTH_ROOT_OF_TWO: f32 = 1.059463094359;
/// The pitch `A 4` represented in steps.
const TUNING_PITCH_A4: f32 = 69.0;
/// The pitch `A 4` represented in hz.
const PITCH_INDEX: f32 = 440.0;
/// Octave offset to match MIDI step standard (i.e. A4 == 69).
const MIDI_OCTAVE_OFFSET: Octave = 1;

pub type Hz = f32;
pub type Mel = f32;
pub type Perc = f64;
pub type Semitones = i32;
pub type Step = f32;
pub type Weight = f32;

/// Find and return the smallest distance
/// between two letters in semitones as an int.
#[inline]
pub fn difference_in_semitones(letter_a: Letter, letter_b: Letter) -> Semitones {
    let diff = (letter_a as Semitones - letter_b as Semitones).abs();
    if diff > 6 { diff - 12 } else { diff }
}

/// Calculate hz from (Letter, Octave).
#[inline]
pub fn hz_from_letter_octave(letter: Letter, octave: Octave) -> Hz {
    hz_from_step(step_from_letter_octave(letter, octave))
}

/// Calculate hz from mel.
#[inline]
pub fn hz_from_mel(mel: Mel) -> Hz {
    (10.0.powf(mel / 2595.0) - 1.0) * 700.0
}

/// Calculate frequency in hz from percentage.
#[inline]
pub fn hz_from_perc(perc: Perc) -> Hz {
    perc as Hz * (MAX_HZ - MIN_HZ) + MIN_HZ
}

/// Calculate hz from scaled percentage.
#[inline]
pub fn hz_from_scaled_perc(scaled: Perc, weight: Weight) -> Hz {
    hz_from_perc(perc_from_scaled_perc(scaled, weight))
}

/// Calculate hz from pitch as `step`.
#[inline]
pub fn hz_from_step(step: Step) -> Hz {
    PITCH_INDEX * TWELFTH_ROOT_OF_TWO.powf(step - TUNING_PITCH_A4)
}

/// Calculate (Letter, Octave) from hz.
#[inline]
pub fn letter_octave_from_hz(hz: Hz) -> (Letter, Octave) {
    letter_octave_from_step(step_from_hz(hz))
}

/// Calculate (Letter, Octave) from mel.
#[inline]
pub fn letter_octave_from_mel(mel: Mel) -> (Letter, Octave) {
    letter_octave_from_hz(hz_from_mel(mel))
}

/// Calculate (Letter, Octave) from a frequency percentage.
#[inline]
pub fn letter_octave_from_perc(perc: Perc) -> (Letter, Octave) {
    letter_octave_from_step(step_from_perc(perc))
}

/// Calculate (Letter, Octave) from a scaled frequency percentage.
#[inline]
pub fn letter_octave_from_scaled_perc(scaled: Perc, weight: Weight) -> (Letter, Octave) {
    letter_octave_from_step(step_from_scaled_perc(scaled, weight))
}

/// Calculate pitch as (Letter, Octave) from pitch as step.
#[inline]
pub fn letter_octave_from_step(step: Step) -> (Letter, Octave) {
    let rounded = step.round() as Octave;
    let letter_step = modulo(rounded, Octave::from(TOTAL_LETTERS));
    (FromPrimitive::from_i32(letter_step).unwrap(), (rounded - letter_step) / 12 - MIDI_OCTAVE_OFFSET)
}

/// Calculate mel from hz.
/// Formula used from http://en.wikipedia.org/wiki/Mel_scale
#[inline]
pub fn mel_from_hz(hz: Hz) -> Mel {
    (1.0 + hz / 700.0).log10() * 2595.0
}

/// Calculate mel from (Letter, Octave).
#[inline]
pub fn mel_from_letter_octave(letter: Letter, octave: Octave) -> Mel {
    mel_from_hz(hz_from_letter_octave(letter, octave))
}

/// Calculate mel from percentage.
#[inline]
pub fn mel_from_perc(perc: Perc) -> Mel {
    mel_from_hz(hz_from_perc(perc))
}

/// Calculate mel from scaled percentage.
#[inline]
pub fn mel_from_scaled_perc(scaled: Perc, weight: Weight) -> Mel {
    mel_from_hz(hz_from_scaled_perc(scaled, weight))
}

/// Calculate mel from step.
#[inline]
pub fn mel_from_step(step: Step) -> Mel {
    mel_from_hz(hz_from_step(step))
}

/// Calculate percentage from hz.
#[inline]
pub fn perc_from_hz(hz: Hz) -> Perc {
    Perc::from(hz - MIN_HZ) / Perc::from(MAX_HZ - MIN_HZ)
}

/// Calculate percentage from letter octave.
#[inline]
pub fn perc_from_letter_octave(letter: Letter, octave: Octave) -> Perc {
    perc_from_step(step_from_letter_octave(letter, octave))
}

/// Calculate percentage from mel.
#[inline]
pub fn perc_from_mel(mel: Mel) -> Perc {
    perc_from_hz(hz_from_mel(mel))
}

/// Calculate percentage from scaled percentage.
#[inline]
pub fn perc_from_scaled_perc(scaled: Perc, weight: Weight) -> Perc {
    scaled.powf(Perc::from(weight))
}

/// Calculate frequency percentage from pitch as `step`.
#[inline]
pub fn perc_from_step(step: Step) -> Perc {
    perc_from_hz(hz_from_step(step))
}

/// Calculate scaled percentage from hz.
#[inline]
pub fn scaled_perc_from_hz(hz: Hz, weight: Weight) -> Perc {
    scaled_perc_from_perc(perc_from_hz(hz), weight)
}

/// Calculate scaled percentage from letter octave.
#[inline]
pub fn scaled_perc_from_letter_octave(letter: Letter, octave: Octave, weight: Weight) -> Perc {
    scaled_perc_from_step(step_from_letter_octave(letter, octave), weight)
}

/// Calculate scaled percentage from mel.
#[inline]
pub fn scaled_perc_from_mel(mel: Mel, weight: Weight) -> Perc {
    scaled_perc_from_hz(hz_from_mel(mel), weight)
}

/// Calculate scaled percentage from percentage.
#[inline]
pub fn scaled_perc_from_perc(perc: Perc, weight: Weight) -> Perc {
    perc.powf(1.0 / Perc::from(weight))
}

/// Calculate scaled frequency percentage from pitch as `step`.
#[inline]
pub fn scaled_perc_from_step(step: Step, weight: Weight) -> Perc {
    scaled_perc_from_hz(hz_from_step(step), weight)
}

/// Calculate the pitch `step` from frequency in hz.
#[inline]
pub fn step_from_hz(hz: Hz) -> Step {
    (hz / PITCH_INDEX).log2() / TWELFTH_ROOT_OF_TWO.log2() + TUNING_PITCH_A4
}

/// Calculate the pitch `step` from (Letter, Octave).
#[inline]
pub fn step_from_letter_octave(letter: Letter, octave: Octave) -> Step {
    (MIDI_OCTAVE_OFFSET + octave) as Step * 12.0 + letter.to_f32().unwrap()
}

/// Calculate the pitch `step` from mel.
#[inline]
pub fn step_from_mel(mel: Mel) -> Step {
    step_from_hz(hz_from_mel(mel))
}

/// Calculate the pitch `step` from frequency precentage.
#[inline]
pub fn step_from_perc(perc: Perc) -> Step {
    step_from_hz(hz_from_perc(perc))
}

/// Calculate the pitch `step` from a scaled frequency precentage.
#[inline]
pub fn step_from_scaled_perc(scaled: Perc, weight: Weight) -> Step {
    step_from_hz(hz_from_scaled_perc(scaled, weight))
}

