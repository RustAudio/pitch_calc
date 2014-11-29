
use std::num::{
    Float,
    SignedInt,
};
use super::{
    Letter,
    MAX_HZ,
    MIN_HZ,
    Octave,
    ScaleWeight,
    TOTAL_LETTERS
};
use utils::modulo;

/// Useful for Pitch calculations.
const TWELFTH_ROOT_OF_TWO: f32 = 1.059463094359;
/// The pitch `A 4` represented in steps.
const TUNING_PITCH_A4: f32 = 57.0;
/// The pitch `A 4` represented in hz.
const PITCH_INDEX: f32 = 440.0;

/// Find and return the smallest distance
/// between two letters in semitones as an int.
#[inline]
pub fn difference_in_semitones(letter_a: Letter, letter_b: Letter) -> int {
    let diff = (letter_a as int - letter_b as int).abs();
    if diff > 6 { diff - 12 } else { diff }
}

/// Calculate hz from (Letter, Octave).
#[inline]
pub fn hz_from_letter_octave(letter: Letter, octave: Octave) -> f32 {
    hz_from_step(step_from_letter_octave(letter, octave))
}

/// Calculate frequency in hz from percentage.
#[inline]
pub fn hz_from_perc(perc: f64) -> f32 {
    perc as f32 * (MAX_HZ - MIN_HZ) + MIN_HZ
}

/// Calculate hz from scaled percentage.
#[inline]
pub fn hz_from_scaled_perc(scaled: f64, weight: f32) -> f32 {
    hz_from_perc(perc_from_scaled_perc(scaled, weight))
}

/// Calculate hz from pitch as `step`.
#[inline]
pub fn hz_from_step(step: f32) -> f32 {
    PITCH_INDEX * TWELFTH_ROOT_OF_TWO.powf(step - TUNING_PITCH_A4)
}

/// Calculate (Letter, Octave) from hz.
#[inline]
pub fn letter_octave_from_hz(hz: f32) -> (Letter, Octave) {
    letter_octave_from_step(step_from_hz(hz))
}

/// Calculate (Letter, Octave) from a frequency percentage.
#[inline]
pub fn letter_octave_from_perc(perc: f64) -> (Letter, Octave) {
    letter_octave_from_step(step_from_perc(perc))
}

/// Calculate (Letter, Octave) from a scaled frequency percentage.
#[inline]
pub fn letter_octave_from_scaled_perc(scaled: f64, weight: f32) -> (Letter, Octave) {
    letter_octave_from_step(step_from_scaled_perc(scaled, weight))
}

/// Calculate pitch as (Letter, Octave) from pitch as step.
#[inline]
pub fn letter_octave_from_step(step: f32) -> (Letter, Octave) {
    let rounded = step.round() as Octave;
    let letter_step = modulo(rounded, TOTAL_LETTERS as Octave);
    (FromPrimitive::from_i32(letter_step).unwrap(), (rounded - letter_step) / 12)
}

/// Calculate percentage from hz.
#[inline]
pub fn perc_from_hz(hz: f32) -> f64 {
    (hz - MIN_HZ) as f64 / (MAX_HZ - MIN_HZ) as f64
}

/// Calculate percentage from letter octave.
#[inline]
pub fn perc_from_letter_octave(letter: Letter, octave: Octave) -> f64 {
    perc_from_step(step_from_letter_octave(letter, octave))
}

/// Calculate percentage from scaled percentage.
#[inline]
pub fn perc_from_scaled_perc(scaled: f64, weight: f32) -> f64 {
    scaled.powf(weight as f64)
}

/// Calculate frequency percentage from pitch as `step`.
#[inline]
pub fn perc_from_step(step: f32) -> f64 {
    perc_from_hz(hz_from_step(step))
}

/// Calculate scaled percentage from hz.
#[inline]
pub fn scaled_perc_from_hz(hz: f32, weight: ScaleWeight) -> f64 {
    scaled_perc_from_perc(perc_from_hz(hz), weight)
}

/// Calculate percentage from letter octave.
#[inline]
pub fn scaled_perc_from_letter_octave(letter: Letter, octave: Octave, weight: ScaleWeight) -> f64 {
    scaled_perc_from_step(step_from_letter_octave(letter, octave), weight)
}

/// Calculate scaled percentage from percentage.
#[inline]
pub fn scaled_perc_from_perc(perc: f64, weight: ScaleWeight) -> f64 {
    perc.powf(1f64 / weight as f64)
}

/// Calculate scaled frequency percentage from pitch as `step`.
#[inline]
pub fn scaled_perc_from_step(step: f32, weight: ScaleWeight) -> f64 {
    scaled_perc_from_hz(hz_from_step(step), weight)
}

/// Calculate the pitch `step` from frequency in hz.
#[inline]
pub fn step_from_hz(hz: f32) -> f32 {
    (hz / PITCH_INDEX).log2() / TWELFTH_ROOT_OF_TWO.log2() + TUNING_PITCH_A4
}

/// Calculate the pitch `step` from (Letter, Octave).
#[inline]
pub fn step_from_letter_octave(letter: Letter, octave: Octave) -> f32 {
    octave as f32 * 12f32 + letter.to_f32().unwrap() as f32
}

/// Calculate the pitch `step` from frequency precentage.
#[inline]
pub fn step_from_perc(perc: f64) -> f32 {
    step_from_hz(hz_from_perc(perc))
}

/// Calculate the pitch `step` from a scaled frequency precentage.
#[inline]
pub fn step_from_scaled_perc(scaled: f64, weight: f32) -> f32 {
    step_from_hz(hz_from_scaled_perc(scaled, weight))
}

