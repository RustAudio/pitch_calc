
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
    hz_from_step,
    letter_octave_from_step,
    perc_from_step,
    scaled_perc_from_step,
};

/// Pitch representation in the form of a MIDI-esque Step.
#[deriving(Show, Copy, Clone, Encodable, Decodable)]
pub struct Step(pub calc::Step);

impl Step {

    /// Return the value in steps.
    #[inline]
    pub fn step(&self) -> calc::Step { let Step(step) = *self; step }

    /// Return the unit value of the equivalent frequency Hz.
    #[inline]
    pub fn hz(&self) -> calc::Hz {
        let Step(step) = *self;
        hz_from_step(step)
    }

    /// Convert to the equivalent frequency in Hz.
    #[inline]
    pub fn to_hz(&self) -> Hz {
        Hz(self.hz())
    }

    /// Convert to the closest equivalent (Letter, Octave).
    #[inline]
    pub fn letter_octave(&self) -> (Letter, Octave) {
        letter_octave_from_step(self.step())
    }

    /// Convert to the closest equivalent Letter.
    #[inline]
    pub fn letter(&self) -> Letter {
        let (letter, _) = self.letter_octave();
        letter
    }

    /// Convert to the closest equivalent Octave.
    #[inline]
    pub fn octave(&self) -> Octave {
        let (_, octave) = self.letter_octave();
        octave
    }

    /// Convert to the closest equivalent LetterOctave.
    #[inline]
    pub fn to_letter_octave(&self) -> LetterOctave {
        let (letter, octave) = self.letter_octave();
        LetterOctave(letter, octave)
    }

    /// Convert to the unit value of the equivalent Perc.
    #[inline]
    pub fn perc(&self) -> calc::Perc {
        perc_from_step(self.step())
    }

    /// Convert to a percentage of the human hearing range.
    #[inline]
    pub fn to_perc(&self) -> Perc {
        Perc(self.perc())
    }

    /// Convert to a scaled percentage of the human hearing range with a given weight.
    #[inline]
    pub fn scaled_perc_with_weight(&self, weight: ScaleWeight) -> calc::Perc {
        scaled_perc_from_step(self.step(), weight)
    }

    /// Convert to a scaled percentage of the human hearing range.
    #[inline]
    pub fn scaled_perc(&self) -> calc::Perc {
        self.scaled_perc_with_weight(DEFAULT_SCALE_WEIGHT)
    }

    /// Convert to a scaled percentage of the human hearing range with a given weight.
    #[inline]
    pub fn to_scaled_perc_with_weight(&self, weight: ScaleWeight) -> ScaledPerc {
        ScaledPerc(self.scaled_perc_with_weight(weight), weight)
    }

    /// Convert to a scaled percentage of the human hearing range.
    #[inline]
    pub fn to_scaled_perc(&self) -> ScaledPerc {
        self.to_scaled_perc_with_weight(DEFAULT_SCALE_WEIGHT)
    }

}

impl Add<Step, Step> for Step {
    #[inline]
    fn add(self, rhs: Step) -> Step {
        Step(self.step() + rhs.step())
    }
}

impl Sub<Step, Step> for Step {
    #[inline]
    fn sub(self, rhs: Step) -> Step {
        Step(self.step() - rhs.step())
    }
}

impl Mul<Step, Step> for Step {
    #[inline]
    fn mul(self, rhs: Step) -> Step {
        Step(self.step() * rhs.step())
    }
}

impl Div<Step, Step> for Step {
    #[inline]
    fn div(self, rhs: Step) -> Step {
        Step(self.step() / rhs.step())
    }
}

impl Rem<Step, Step> for Step {
    #[inline]
    fn rem(self, rhs: Step) -> Step {
        Step(self.step() % rhs.step())
    }
}

impl Neg<Step> for Step {
    #[inline]
    fn neg(self) -> Step {
        Step(-self.step())
    }
}

impl PartialEq for Step {
    #[inline]
    fn eq(&self, other: &Step) -> bool {
        self.step() == other.step()
    }
}

impl Eq for Step {}

impl PartialOrd for Step {
    #[inline]
    fn partial_cmp(&self, other: &Step) -> Option<Ordering> {
        self.step().partial_cmp(&other.step())
    }
}

impl Ord for Step {
    #[inline]
    fn cmp(&self, other: &Step) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

