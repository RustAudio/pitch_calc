
use super::{
    Letter,
    Hz,
    ToHz,
    Perc,
    ToPerc,
    ScaledPerc,
    ScaleWeight,
    ToScaledPerc,
    Step,
    ToStep,
    hz_from_letter_octave,
    perc_from_letter_octave,
    scaled_perc_from_letter_octave,
    step_from_letter_octave,
};

pub type Octave = i32;

/// Pitch representation in the form of a frequency (hz).
#[deriving(Show, Clone, Encodable, Decodable)]
pub struct LetterOctave(pub Letter, pub Octave);

/// For types that can be represented as a musical Letter and an Octave.
pub trait ToLetterOctave {
    /// Return the current type in the form of LetterOctave.
    fn to_letter_octave(&self) -> LetterOctave;
    /// Return the value as (Letter, Octave).
    #[inline]
    fn letter_octave(&self) -> (Letter, Octave) {
        let LetterOctave(letter, octave) = self.to_letter_octave();
        (letter, octave)
    }
    /// Return just the Letter.
    #[inline]
    fn letter(&self) -> Letter {
        let LetterOctave(letter, _) = self.to_letter_octave();
        letter
    }
    /// Return just the octave.
    fn octave(&self) -> Octave {
        let LetterOctave(_, octave) = self.to_letter_octave();
        octave
    }
}

impl ToHz for LetterOctave {
    #[inline]
    fn to_hz(&self) -> Hz {
        let LetterOctave(letter, octave) = *self;
        Hz(hz_from_letter_octave(letter, octave))
    }
}

impl ToLetterOctave for LetterOctave {
    #[inline]
    fn to_letter_octave(&self) -> LetterOctave { *self }
}

impl ToPerc for LetterOctave {
    #[inline]
    fn to_perc(&self) -> Perc {
        let LetterOctave(letter, octave) = *self;
        Perc(perc_from_letter_octave(letter, octave))
    }
}

impl ToScaledPerc for LetterOctave {
    #[inline]
    fn to_scaled_perc_with_weight(&self, weight: ScaleWeight) -> ScaledPerc {
        let LetterOctave(letter, octave) = *self;
        ScaledPerc(scaled_perc_from_letter_octave(letter, octave, weight), weight)
    }
}

impl ToStep for LetterOctave {
    #[inline]
    fn to_step(&self) -> Step {
        let LetterOctave(letter, octave) = *self;
        Step(step_from_letter_octave(letter, octave))
    }
}

impl Add<LetterOctave, LetterOctave> for LetterOctave {
    #[inline]
    fn add(&self, rhs: &LetterOctave) -> LetterOctave {
        (self.to_step() + rhs.to_step()).to_letter_octave()
    }
}

impl Sub<LetterOctave, LetterOctave> for LetterOctave {
    #[inline]
    fn sub(&self, rhs: &LetterOctave) -> LetterOctave {
        (self.to_step() - rhs.to_step()).to_letter_octave()
    }
}

impl Mul<LetterOctave, LetterOctave> for LetterOctave {
    #[inline]
    fn mul(&self, rhs: &LetterOctave) -> LetterOctave {
        (self.to_step() * rhs.to_step()).to_letter_octave()
    }
}

impl Div<LetterOctave, LetterOctave> for LetterOctave {
    #[inline]
    fn div(&self, rhs: &LetterOctave) -> LetterOctave {
        (self.to_step() / rhs.to_step()).to_letter_octave()
    }
}

impl Rem<LetterOctave, LetterOctave> for LetterOctave {
    #[inline]
    fn rem(&self, rhs: &LetterOctave) -> LetterOctave {
        (self.to_step() % rhs.to_step()).to_letter_octave()
    }
}

impl Neg<LetterOctave> for LetterOctave {
    #[inline]
    fn neg(&self) -> LetterOctave {
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
        let LetterOctave(letter, octave) = *self;
        let LetterOctave(other_letter, other_octave) = *other;
        match octave.partial_cmp(&other_octave) {
            Some(Equal) => letter.partial_cmp(&other_letter),
            ordering => ordering,
        }
    }
}

impl Ord for LetterOctave {
    #[inline]
    fn cmp(&self, other: &LetterOctave) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

