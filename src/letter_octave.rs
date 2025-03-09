use super::{
    calc, hz_from_letter_octave, mel_from_letter_octave, perc_from_letter_octave,
    scaled_perc_from_letter_octave, step_from_letter_octave, Hz, Letter, Mel, Perc, ScaleWeight,
    ScaledPerc, Step, DEFAULT_SCALE_WEIGHT,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

pub type Octave = i32;

/// Pitch representation in the form of a frequency (hz).
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LetterOctave(pub Letter, pub Octave);

impl LetterOctave {
    /// Return the value as (Letter, Octave).
    #[inline]
    pub fn letter_octave(self) -> (Letter, Octave) {
        let LetterOctave(letter, octave) = self;
        (letter, octave)
    }

    /// Return just the Letter.
    #[inline]
    pub fn letter(self) -> Letter {
        let LetterOctave(letter, _) = self;
        letter
    }

    /// Return just the octave.
    #[inline]
    pub fn octave(self) -> Octave {
        let LetterOctave(_, octave) = self;
        octave
    }

    /// Convert to the unit value of Hz with the equivalent pitch.
    #[inline]
    pub fn hz(self) -> calc::Hz {
        let LetterOctave(letter, octave) = self;
        hz_from_letter_octave(letter, octave)
    }

    /// Convert to a Hz with the equivalent pitch.
    #[inline]
    pub fn to_hz(self) -> Hz {
        Hz(self.hz())
    }

    /// Convert to the unit value of a Mel with equivalent pitch.
    #[inline]
    pub fn mel(self) -> calc::Mel {
        let LetterOctave(letter, octave) = self;
        mel_from_letter_octave(letter, octave)
    }

    /// Convert to a Mel struct.
    #[inline]
    pub fn to_mel(self) -> Mel {
        Mel(self.mel())
    }

    /// Convert to the unit value of a Perc.
    #[inline]
    pub fn perc(self) -> calc::Perc {
        let LetterOctave(letter, octave) = self;
        perc_from_letter_octave(letter, octave)
    }

    /// Convert to a percentage of the human hearing range.
    #[inline]
    pub fn to_perc(self) -> Perc {
        Perc(self.perc())
    }

    /// Convert to a scaled percentage of the human hearing range with a given weight.
    #[inline]
    pub fn scaled_perc_with_weight(self, weight: ScaleWeight) -> calc::Perc {
        let LetterOctave(letter, octave) = self;
        scaled_perc_from_letter_octave(letter, octave, weight)
    }

    /// Convert to a scaled percentage of the human hearing range.
    #[inline]
    pub fn scaled_perc(self) -> calc::Perc {
        self.scaled_perc_with_weight(DEFAULT_SCALE_WEIGHT)
    }

    /// Convert to a scaled percentage of the human hearing range with a given weight.
    #[inline]
    pub fn to_scaled_perc_with_weight(self, weight: ScaleWeight) -> ScaledPerc {
        ScaledPerc(self.scaled_perc_with_weight(weight), weight)
    }

    /// Convert to a scaled percentage of the human hearing range.
    #[inline]
    pub fn to_scaled_perc(self) -> ScaledPerc {
        self.to_scaled_perc_with_weight(DEFAULT_SCALE_WEIGHT)
    }

    /// Convert to the unit value of a Step.
    #[inline]
    pub fn step(self) -> calc::Step {
        let LetterOctave(letter, octave) = self;
        step_from_letter_octave(letter, octave)
    }

    /// Convert to a floating point MIDI-esque Step.
    #[inline]
    pub fn to_step(self) -> Step {
        Step(self.step())
    }
}

impl Add for LetterOctave {
    type Output = LetterOctave;
    #[inline]
    fn add(self, rhs: LetterOctave) -> LetterOctave {
        (self.to_step() + rhs.to_step()).to_letter_octave()
    }
}

impl Sub for LetterOctave {
    type Output = LetterOctave;
    #[inline]
    fn sub(self, rhs: LetterOctave) -> LetterOctave {
        (self.to_step() - rhs.to_step()).to_letter_octave()
    }
}

impl Mul for LetterOctave {
    type Output = LetterOctave;
    #[inline]
    fn mul(self, rhs: LetterOctave) -> LetterOctave {
        (self.to_step() * rhs.to_step()).to_letter_octave()
    }
}

impl Div for LetterOctave {
    type Output = LetterOctave;
    #[inline]
    fn div(self, rhs: LetterOctave) -> LetterOctave {
        (self.to_step() / rhs.to_step()).to_letter_octave()
    }
}

impl Rem for LetterOctave {
    type Output = LetterOctave;
    #[inline]
    fn rem(self, rhs: LetterOctave) -> LetterOctave {
        (self.to_step() % rhs.to_step()).to_letter_octave()
    }
}

impl Neg for LetterOctave {
    type Output = LetterOctave;
    #[inline]
    fn neg(self) -> LetterOctave {
        (-self.to_step()).to_letter_octave()
    }
}

impl PartialEq for LetterOctave {
    #[inline]
    fn eq(&self, other: &LetterOctave) -> bool {
        let LetterOctave(letter, octave) = *self;
        let LetterOctave(other_letter, other_octave) = *other;
        letter == other_letter && octave == other_octave
    }
}

impl Eq for LetterOctave {}

impl PartialOrd for LetterOctave {
    #[inline]
    fn partial_cmp(&self, other: &LetterOctave) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LetterOctave {
    #[inline]
    fn cmp(&self, other: &LetterOctave) -> Ordering {
        let LetterOctave(letter, octave) = *self;
        let LetterOctave(other_letter, other_octave) = *other;
        match octave.cmp(&other_octave) {
            Ordering::Equal => letter.cmp(&other_letter),
            ordering => ordering,
        }
    }
}
