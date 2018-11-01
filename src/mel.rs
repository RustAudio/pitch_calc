use std::cmp::Ordering;
use std::ops::{Add, Sub, Mul, Div, Rem, Neg};
use super::{
    calc,
    DEFAULT_SCALE_WEIGHT,
    Hz,
    LetterOctave,
    Letter,
    Octave,
    Perc,
    ScaledPerc,
    ScaleWeight,
    Step,
    hz_from_mel,
    letter_octave_from_mel,
    perc_from_mel,
    scaled_perc_from_mel,
    step_from_mel,
};

/// Mel value representation
///     - based on the Mel scale coined by Stevens, Volkmann and Newman in 1937.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde_serialization", derive(Serialize, Deserialize))]
pub struct Mel(pub calc::Mel);

impl Mel {
    /// Return the unit value of the Mel struct.
    #[inline]
    pub fn mel(self) -> calc::Mel {
        let Mel(mel) = self;
        mel
    }

    /// Convert to hz.
    #[inline]
    pub fn hz(self) -> calc::Hz {
        hz_from_mel(self.mel())
    }

    /// Convert to a Hz struct.
    #[inline]
    pub fn to_hz(self) -> Hz {
        Hz(self.hz())
    }

    /// Convert to (Letter, Octave) tuple.
    #[inline]
    pub fn letter_octave(self) -> (Letter, Octave) {
        letter_octave_from_mel(self.mel())
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

    /// Convert to LetterOctave struct with the closest pitch.
    #[inline]
    pub fn to_letter_octave(self) -> LetterOctave {
        let (letter, octave) = self.letter_octave();
        LetterOctave(letter, octave)
    }

    /// Convert to a percentage of the human hearing range.
    #[inline]
    pub fn perc(self) -> calc::Perc {
        perc_from_mel(self.mel())
    }

    /// Convert to a Perc struct.
    #[inline]
    pub fn to_perc(self) -> Perc {
        Perc(self.perc())
    }

    /// Convert to a scaled percentage of the human hearing range with a given weight.
    #[inline]
    pub fn scaled_perc_with_weight(self, weight: ScaleWeight) -> calc::Perc {
        scaled_perc_from_mel(self.mel(), weight)
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
        step_from_mel(self.mel())
    }

    /// Convert to a Step struct.
    #[inline]
    pub fn to_step(self) -> Step {
        Step(self.step())
    }
}

impl Add for Mel {
    type Output = Mel;
    #[inline]
    fn add(self, rhs: Mel) -> Mel {
        Mel(self.mel() + rhs.mel())
    }
}

impl Sub for Mel {
    type Output = Mel;
    #[inline]
    fn sub(self, rhs: Mel) -> Mel {
        Mel(self.mel() - rhs.mel())
    }
}

impl Mul for Mel {
    type Output = Mel;
    #[inline]
    fn mul(self, rhs: Mel) -> Mel {
        Mel(self.mel() * rhs.mel())
    }
}

impl Div for Mel {
    type Output = Mel;
    #[inline]
    fn div(self, rhs: Mel) -> Mel {
        Mel(self.mel() / rhs.mel())
    }
}

impl Rem for Mel {
    type Output = Mel;
    #[inline]
    fn rem(self, rhs: Mel) -> Mel {
        Mel(self.mel() % rhs.mel())
    }
}

impl Neg for Mel {
    type Output = Mel;
    #[inline]
    fn neg(self) -> Mel {
        Mel(-self.mel())
    }
}

impl PartialEq for Mel {
    #[inline]
    fn eq(&self, other: &Mel) -> bool {
        self.mel() == other.mel()
    }
}

impl Eq for Mel {}

impl PartialOrd for Mel {
    #[inline]
    fn partial_cmp(&self, other: &Mel) -> Option<Ordering> {
        self.mel().partial_cmp(&other.mel())
    }
}

impl Ord for Mel {
    #[inline]
    fn cmp(&self, other: &Mel) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

