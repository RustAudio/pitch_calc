use std::cmp::Ordering;
use std::ops::{Add, Sub, Mul, Div, Rem, Neg};
use super::{
    calc,
    DEFAULT_SCALE_WEIGHT,
    Hz,
    LetterOctave,
    Letter,
    Mel,
    Octave,
    ScaledPerc,
    ScaleWeight,
    Step,
    hz_from_perc,
    letter_octave_from_perc,
    mel_from_perc,
    scaled_perc_from_perc,
    step_from_perc,
};

/// Pitch representation in the form of a percentage between the min and max hz.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde_serialization", derive(Serialize, Deserialize))]
pub struct Perc(pub calc::Perc);

impl Perc {
    /// Return the value as a percentage.
    #[inline]
    pub fn perc(self) -> calc::Perc {
        let Perc(perc) = self;
        perc
    }

    /// Convert to unit value of the equivalent frequency in Hz.
    #[inline]
    pub fn hz(self) -> calc::Hz {
        let Perc(perc) = self;
        hz_from_perc(perc)
    }

    /// Convert to the equivalent frequency in Hz.
    #[inline]
    pub fn to_hz(self) -> Hz {
        Hz(self.hz())
    }

    /// Convert to a (Letter, Octave).
    #[inline]
    pub fn letter_octave(self) -> (Letter, Octave) {
        letter_octave_from_perc(self.perc())
    }

    /// Convert to Letter.
    #[inline]
    pub fn letter(self) -> Letter {
        let (letter, _) = self.letter_octave();
        letter
    }

    /// Convert to Octave.
    #[inline]
    pub fn octave(self) -> Octave {
        let (_, octave) = self.letter_octave();
        octave
    }

    /// Convert to LetterOctave.
    #[inline]
    pub fn to_letter_octave(self) -> LetterOctave {
        let (letter, octave) = self.letter_octave();
        LetterOctave(letter, octave)
    }

    /// Convert to the unit value of a Mel.
    #[inline]
    pub fn mel(self) -> calc::Mel {
        mel_from_perc(self.perc())
    }

    /// Convert to a Mel struct.
    #[inline]
    pub fn to_mel(self) -> Mel {
        Mel(self.mel())
    }

    /// Convert to a scaled percentage of the human hearing range with a given weight.
    #[inline]
    pub fn scaled_perc_with_weight(self, weight: ScaleWeight) -> calc::Perc {
        scaled_perc_from_perc(self.perc(), weight)
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
        step_from_perc(self.perc())
    }

    /// Convert to a floating point MIDI-esque Step.
    #[inline]
    pub fn to_step(self) -> Step {
        Step(self.step())
    }
}

impl Add for Perc {
    type Output = Perc;
    #[inline]
    fn add(self, rhs: Perc) -> Perc {
        Perc(self.perc() + rhs.perc())
    }
}

impl Sub for Perc {
    type Output = Perc;
    #[inline]
    fn sub(self, rhs: Perc) -> Perc {
        Perc(self.perc() - rhs.perc())
    }
}

impl Mul for Perc {
    type Output = Perc;
    #[inline]
    fn mul(self, rhs: Perc) -> Perc {
        Perc(self.perc() * rhs.perc())
    }
}

impl Div for Perc {
    type Output = Perc;
    #[inline]
    fn div(self, rhs: Perc) -> Perc {
        Perc(self.perc() / rhs.perc())
    }
}

impl Rem for Perc {
    type Output = Perc;
    #[inline]
    fn rem(self, rhs: Perc) -> Perc {
        Perc(self.perc() % rhs.perc())
    }
}

impl Neg for Perc {
    type Output = Perc;
    #[inline]
    fn neg(self) -> Perc {
        Perc(-self.perc())
    }
}

impl PartialEq for Perc {
    #[inline]
    fn eq(&self, other: &Perc) -> bool {
        self.perc() == other.perc()
    }
}

impl Eq for Perc {}

impl PartialOrd for Perc {
    #[inline]
    fn partial_cmp(&self, other: &Perc) -> Option<Ordering> {
        self.perc().partial_cmp(&other.perc())
    }
}

impl Ord for Perc {
    #[inline]
    fn cmp(&self, other: &Perc) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

