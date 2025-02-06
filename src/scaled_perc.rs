use super::{
    calc, hz_from_scaled_perc, letter_octave_from_scaled_perc, mel_from_scaled_perc,
    perc_from_scaled_perc, step_from_scaled_perc, Hz, Letter, LetterOctave, Mel, Octave, Perc,
    Step,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

pub type ScaleWeight = calc::Weight;
pub const DEFAULT_SCALE_WEIGHT: ScaleWeight = 4.0;

/// Pitch representation in the form of a scaled percentage between the min and max hz.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScaledPerc(pub calc::Perc, pub ScaleWeight);

impl ScaledPerc {
    /// A constructor for a ScaledPerc that uses the default weight.
    pub fn new(perc: calc::Perc) -> ScaledPerc {
        ScaledPerc(perc, DEFAULT_SCALE_WEIGHT)
    }

    /// Return the value as a scaled percentage.
    #[inline]
    pub fn scaled_perc(&self) -> calc::Perc {
        let ScaledPerc(perc, _) = *self;
        perc
    }

    /// Return the scale weight.
    #[inline]
    pub fn scale_weight(&self) -> ScaleWeight {
        let ScaledPerc(_, weight) = *self;
        weight
    }

    /// Convert to the unit value of the equivalent frequency in Hz.
    #[inline]
    pub fn hz(&self) -> calc::Hz {
        let ScaledPerc(perc, weight) = *self;
        hz_from_scaled_perc(perc, weight)
    }

    /// Convert to the equivalent frequency in Hz.
    #[inline]
    pub fn to_hz(&self) -> Hz {
        Hz(self.hz())
    }

    /// Convert to (Letter, Octave) tuple.
    #[inline]
    pub fn letter_octave(&self) -> (Letter, Octave) {
        let ScaledPerc(perc, weight) = *self;
        letter_octave_from_scaled_perc(perc, weight)
    }

    /// Convert to Letter.
    #[inline]
    pub fn letter(&self) -> Letter {
        let (letter, _) = self.letter_octave();
        letter
    }

    /// Convert to Octave.
    #[inline]
    pub fn octave(&self) -> Octave {
        let (_, octave) = self.letter_octave();
        octave
    }

    /// Convert to LetterOctave.
    #[inline]
    pub fn to_letter_octave(&self) -> LetterOctave {
        let (letter, octave) = self.letter_octave();
        LetterOctave(letter, octave)
    }

    /// Convert to the unit value of a Mel.
    #[inline]
    pub fn mel(&self) -> calc::Mel {
        let ScaledPerc(perc, weight) = *self;
        mel_from_scaled_perc(perc, weight)
    }

    /// Convert to a Mel struct.
    #[inline]
    pub fn to_mel(&self) -> Mel {
        Mel(self.mel())
    }

    /// Convert to the unit value of a Perc struct.
    #[inline]
    pub fn perc(&self) -> calc::Perc {
        let ScaledPerc(perc, weight) = *self;
        perc_from_scaled_perc(perc, weight)
    }

    /// Convert to Perc.
    #[inline]
    pub fn to_perc(&self) -> Perc {
        Perc(self.perc())
    }

    /// Convert to the unit value of a Step.
    #[inline]
    pub fn step(&self) -> calc::Step {
        let ScaledPerc(perc, weight) = *self;
        step_from_scaled_perc(perc, weight)
    }

    /// Convert to a floating point MIDI-esque Step.
    #[inline]
    pub fn to_step(&self) -> Step {
        Step(self.step())
    }
}

impl Add for ScaledPerc {
    type Output = ScaledPerc;
    #[inline]
    fn add(self, rhs: ScaledPerc) -> ScaledPerc {
        (self.to_perc() + rhs.to_perc()).to_scaled_perc_with_weight(self.scale_weight())
    }
}

impl Sub for ScaledPerc {
    type Output = ScaledPerc;
    #[inline]
    fn sub(self, rhs: ScaledPerc) -> ScaledPerc {
        (self.to_perc() - rhs.to_perc()).to_scaled_perc_with_weight(self.scale_weight())
    }
}

impl Mul for ScaledPerc {
    type Output = ScaledPerc;
    #[inline]
    fn mul(self, rhs: ScaledPerc) -> ScaledPerc {
        (self.to_perc() * rhs.to_perc()).to_scaled_perc_with_weight(self.scale_weight())
    }
}

impl Div for ScaledPerc {
    type Output = ScaledPerc;
    #[inline]
    fn div(self, rhs: ScaledPerc) -> ScaledPerc {
        (self.to_perc() / rhs.to_perc()).to_scaled_perc_with_weight(self.scale_weight())
    }
}

impl Rem for ScaledPerc {
    type Output = ScaledPerc;
    #[inline]
    fn rem(self, rhs: ScaledPerc) -> ScaledPerc {
        (self.to_perc() % rhs.to_perc()).to_scaled_perc_with_weight(self.scale_weight())
    }
}

impl Neg for ScaledPerc {
    type Output = ScaledPerc;
    #[inline]
    fn neg(self) -> ScaledPerc {
        ScaledPerc(-self.scaled_perc(), self.scale_weight())
    }
}

impl PartialEq for ScaledPerc {
    #[inline]
    fn eq(&self, other: &ScaledPerc) -> bool {
        self.perc() == other.perc()
    }
}

impl Eq for ScaledPerc {}

impl PartialOrd for ScaledPerc {
    #[inline]
    fn partial_cmp(&self, other: &ScaledPerc) -> Option<Ordering> {
        self.to_perc().partial_cmp(&other.to_perc())
    }
}
