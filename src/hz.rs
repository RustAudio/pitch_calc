use std::cmp::Ordering;
use std::ops::{Add, Sub, Mul, Div, Rem, Neg};
use super::{
    calc,
    DEFAULT_SCALE_WEIGHT,
    LetterOctave,
    Letter,
    Mel,
    Octave,
    Perc,
    ScaledPerc,
    ScaleWeight,
    Step,
    letter_octave_from_hz,
    mel_from_hz,
    perc_from_hz,
    scaled_perc_from_hz,
    step_from_hz,
};

pub const MAX: calc::Hz = 20_000.0;
pub const MIN: calc::Hz = 20.0;

/// Pitch representation in the form of a frequency (hz).
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde_serialization", derive(Serialize, Deserialize))]
pub struct Hz(pub calc::Hz);

impl Hz {
    /// Return the unit value of the Hz struct.
    #[inline]
    pub fn hz(self) -> calc::Hz {
        let Hz(hz) = self;
        hz
    }

    /// Convert to (Letter, Octave) tuple.
    #[inline]
    pub fn letter_octave(self) -> (Letter, Octave) {
        let Hz(hz) = self;
        letter_octave_from_hz(hz)
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

    /// Convert to a LetterOctave struct with the same pitch.
    #[inline]
    pub fn to_letter_octave(self) -> LetterOctave {
        let (letter, octave) = self.letter_octave();
        LetterOctave(letter, octave)
    }

    /// Convert to the unit value of a Mel.
    #[inline]
    pub fn mel(self) -> calc::Mel {
        mel_from_hz(self.hz())
    }

    /// Convert to a Mel struct.
    #[inline]
    pub fn to_mel(self) -> Mel {
        Mel(self.mel())
    }

    /// Convert to the unit value of a Perc struct.
    #[inline]
    pub fn perc(self) -> calc::Perc {
        let Hz(hz) = self;
        perc_from_hz(hz)
    }

    /// Convert to a percentage of the human hearing range.
    #[inline]
    pub fn to_perc(self) -> Perc {
        Perc(self.perc())
    }

    /// Convert to a scaled percentage of the human hearing range with a given weight.
    #[inline]
    pub fn scaled_perc_with_weight(self, weight: ScaleWeight) -> calc::Perc {
        let Hz(hz) = self;
        scaled_perc_from_hz(hz, weight)
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
        let Hz(hz) = self;
        step_from_hz(hz)
    }

    /// Convert to a floating point MIDI-esque Step.
    #[inline]
    pub fn to_step(self) -> Step {
        Step(self.step())
    }
}

impl Add for Hz {
    type Output = Hz;
    #[inline]
    fn add(self, rhs: Hz) -> Hz {
        Hz(self.hz() + rhs.hz())
    }
}

impl Sub for Hz {
    type Output = Hz;
    #[inline]
    fn sub(self, rhs: Hz) -> Hz {
        Hz(self.hz() - rhs.hz())
    }
}

impl Mul for Hz {
    type Output = Hz;
    #[inline]
    fn mul(self, rhs: Hz) -> Hz {
        Hz(self.hz() * rhs.hz())
    }
}

impl Div for Hz {
    type Output = Hz;
    #[inline]
    fn div(self, rhs: Hz) -> Hz {
        Hz(self.hz() / rhs.hz())
    }
}

impl Rem for Hz {
    type Output = Hz;
    #[inline]
    fn rem(self, rhs: Hz) -> Hz {
        Hz(self.hz() % rhs.hz())
    }
}

impl Neg for Hz {
    type Output = Hz;
    #[inline]
    fn neg(self) -> Hz {
        Hz(-self.hz())
    }
}

impl PartialEq for Hz {
    #[inline]
    fn eq(&self, other: &Hz) -> bool {
        self.hz() == other.hz()
    }
}

impl Eq for Hz {}

impl PartialOrd for Hz {
    #[inline]
    fn partial_cmp(&self, other: &Hz) -> Option<Ordering> {
        self.hz().partial_cmp(&other.hz())
    }
}

impl Ord for Hz {
    #[inline]
    fn cmp(&self, other: &Hz) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
