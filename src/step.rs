#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use super::{
    calc,
    DEFAULT_SCALE_WEIGHT,
    Hz,
    hz_from_step,
    Letter,
    letter_octave_from_step,
    LetterOctave,
    Mel,
    mel_from_step,
    Octave,
    Perc,
    perc_from_step,
    scaled_perc_from_step,
    ScaledPerc,
    ScaleWeight,
};

/// Pitch representation in the form of a MIDI-esque Step.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Step(pub calc::Step);

impl Step {
    /// Return the value in steps.
    #[inline]
    pub fn step(self) -> calc::Step {
        let Step(step) = self;
        step
    }

    /// Return the unit value of the equivalent frequency Hz.
    #[inline]
    pub fn hz(self) -> calc::Hz {
        let Step(step) = self;
        hz_from_step(step)
    }

    /// Convert to the equivalent frequency in Hz.
    #[inline]
    pub fn to_hz(self) -> Hz {
        Hz(self.hz())
    }

    /// Convert to the closest equivalent (Letter, Octave).
    #[inline]
    pub fn letter_octave(self) -> (Letter, Octave) {
        letter_octave_from_step(self.step())
    }

    /// Convert to the closest equivalent Letter.
    #[inline]
    pub fn letter(self) -> Letter {
        let (letter, _) = self.letter_octave();
        letter
    }

    /// Convert to the closest equivalent Octave.
    #[inline]
    pub fn octave(self) -> Octave {
        let (_, octave) = self.letter_octave();
        octave
    }

    /// Convert to the closest equivalent LetterOctave.
    #[inline]
    pub fn to_letter_octave(self) -> LetterOctave {
        let (letter, octave) = self.letter_octave();
        LetterOctave(letter, octave)
    }

    /// Convert to a Mel unit value.
    #[inline]
    pub fn mel(self) -> calc::Mel {
        mel_from_step(self.step())
    }

    /// Convert to a Mel struct.
    #[inline]
    pub fn to_mel(self) -> Mel {
        Mel(self.mel())
    }

    /// Convert to the unit value of the equivalent Perc.
    #[inline]
    pub fn perc(self) -> calc::Perc {
        perc_from_step(self.step())
    }

    /// Convert to a percentage of the human hearing range.
    #[inline]
    pub fn to_perc(self) -> Perc {
        Perc(self.perc())
    }

    /// Convert to a scaled percentage of the human hearing range with a given weight.
    #[inline]
    pub fn scaled_perc_with_weight(self, weight: ScaleWeight) -> calc::Perc {
        scaled_perc_from_step(self.step(), weight)
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
}

impl Add for Step {
    type Output = Step;
    #[inline]
    fn add(self, rhs: Step) -> Step {
        Step(self.step() + rhs.step())
    }
}

impl Sub for Step {
    type Output = Step;
    #[inline]
    fn sub(self, rhs: Step) -> Step {
        Step(self.step() - rhs.step())
    }
}

impl Mul for Step {
    type Output = Step;
    #[inline]
    fn mul(self, rhs: Step) -> Step {
        Step(self.step() * rhs.step())
    }
}

impl Div for Step {
    type Output = Step;
    #[inline]
    fn div(self, rhs: Step) -> Step {
        Step(self.step() / rhs.step())
    }
}

impl Rem for Step {
    type Output = Step;
    #[inline]
    fn rem(self, rhs: Step) -> Step {
        Step(self.step() % rhs.step())
    }
}

impl Neg for Step {
    type Output = Step;
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

impl<T> From<T> for Step where T: Into<f32> {
    fn from(t: T) -> Step {
        Step(t.into())
    }
}

#[cfg(test)]
mod tests {
    use super::Step;

    macro_rules! t {
        (
            $($x:ty),+
        ) => {
            fn from(val: f32) {
                $(
                    assert_eq!(Step::from(val as $x), Step(val));
                )*
            }

            #[test]
            fn test_from_integer() {
                from(0.0);
                from(60.0);
                from(127.0);
            }

            fn into(val: f32) {
                $(
                    let actual: Step = (val as $x).into();
                    assert_eq!(actual, Step(val));
                )*
            }

            #[test]
            fn test_into_integer() {
                into(0.0);
                into(60.0);
                into(127.0);
            }
        };
    }

    t!(u8, u16, i8, i16);
}
