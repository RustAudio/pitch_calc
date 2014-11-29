
use super::{
    Hz,
    ToHz,
    LetterOctave,
    ToLetterOctave,
    Perc,
    ToPerc,
    ScaledPerc,
    ScaleWeight,
    ToScaledPerc,
    hz_from_step,
    letter_octave_from_step,
    perc_from_step,
    scaled_perc_from_step,
};

/// Pitch representation in the form of a MIDI-esque Step.
#[deriving(Show, Clone, Encodable, Decodable)]
pub struct Step(pub f32);

/// For types that can be represented as a pitch step.
pub trait ToStep {
    /// Return the current type in the form of a Step.
    fn to_step(&self) -> Step;
    /// Return the value in steps.
    #[inline]
    fn step(&self) -> f32 { let Step(step) = self.to_step(); step }
}

impl ToHz for Step {
    #[inline]
    fn to_hz(&self) -> Hz {
        let Step(step) = *self;
        Hz(hz_from_step(step))
    }
}

impl ToLetterOctave for Step {
    #[inline]
    fn to_letter_octave(&self) -> LetterOctave {
        let Step(step) = *self;
        let (letter, octave) = letter_octave_from_step(step);
        LetterOctave(letter, octave)
    }
}

impl ToPerc for Step {
    #[inline]
    fn to_perc(&self) -> Perc {
        let Step(step) = *self;
        Perc(perc_from_step(step))
    }
}

impl ToScaledPerc for Step {
    #[inline]
    fn to_scaled_perc_with_weight(&self, weight: ScaleWeight) -> ScaledPerc {
        let Step(step) = *self;
        ScaledPerc(scaled_perc_from_step(step, weight), weight)
    }
}

impl ToStep for Step {
    #[inline]
    fn to_step(&self) -> Step { *self }
}

impl Add<Step, Step> for Step {
    #[inline]
    fn add(&self, rhs: &Step) -> Step {
        Step(self.step() + rhs.step())
    }
}

impl Sub<Step, Step> for Step {
    #[inline]
    fn sub(&self, rhs: &Step) -> Step {
        Step(self.step() - rhs.step())
    }
}

impl Mul<Step, Step> for Step {
    #[inline]
    fn mul(&self, rhs: &Step) -> Step {
        Step(self.step() * rhs.step())
    }
}

impl Div<Step, Step> for Step {
    #[inline]
    fn div(&self, rhs: &Step) -> Step {
        Step(self.step() / rhs.step())
    }
}

impl Rem<Step, Step> for Step {
    #[inline]
    fn rem(&self, rhs: &Step) -> Step {
        Step(self.step() % rhs.step())
    }
}

impl Neg<Step> for Step {
    #[inline]
    fn neg(&self) -> Step {
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

