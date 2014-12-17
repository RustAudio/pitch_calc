
use super::{
    LetterOctave,
    ToLetterOctave,
    Perc,
    ToPerc,
    ScaledPerc,
    ScaleWeight,
    ToScaledPerc,
    Step,
    ToStep,
    letter_octave_from_hz,
    perc_from_hz,
    scaled_perc_from_hz,
    step_from_hz,
};

pub const MAX: f32 = 20_000.0;
pub const MIN: f32 = 20.0;

/// Pitch representation in the form of a frequency (hz).
#[deriving(Show, Copy, Clone, Encodable, Decodable)]
pub struct Hz(pub f32);

/// For types that can be represented in hz.
pub trait ToHz {
    /// Return the current type in the form of Hz.
    fn to_hz(&self) -> Hz;
    /// Return the value in hz.
    #[inline]
    fn hz(&self) -> f32 { let Hz(hz) = self.to_hz(); hz }
}

impl ToHz for Hz {
    #[inline]
    fn to_hz(&self) -> Hz { *self }
}

impl ToLetterOctave for Hz {
    #[inline]
    fn to_letter_octave(&self) -> LetterOctave {
        let Hz(hz) = *self;
        let (letter, octave) = letter_octave_from_hz(hz);
        LetterOctave(letter, octave)
    }
}

impl ToPerc for Hz {
    #[inline]
    fn to_perc(&self) -> Perc {
        let Hz(hz) = *self;
        Perc(perc_from_hz(hz))
    }
}

impl ToScaledPerc for Hz {
    #[inline]
    fn to_scaled_perc_with_weight(&self, weight: ScaleWeight) -> ScaledPerc {
        let Hz(hz) = *self;
        ScaledPerc(scaled_perc_from_hz(hz, weight), weight)
    }
}

impl ToStep for Hz {
    #[inline]
    fn to_step(&self) -> Step {
        let Hz(hz) = *self;
        Step(step_from_hz(hz))
    }
}

impl Add<Hz, Hz> for Hz {
    #[inline]
    fn add(self, rhs: Hz) -> Hz {
        Hz(self.hz() + rhs.hz())
    }
}

impl Sub<Hz, Hz> for Hz {
    #[inline]
    fn sub(self, rhs: Hz) -> Hz {
        Hz(self.hz() - rhs.hz())
    }
}

impl Mul<Hz, Hz> for Hz {
    #[inline]
    fn mul(self, rhs: Hz) -> Hz {
        Hz(self.hz() * rhs.hz())
    }
}

impl Div<Hz, Hz> for Hz {
    #[inline]
    fn div(self, rhs: Hz) -> Hz {
        Hz(self.hz() / rhs.hz())
    }
}

impl Rem<Hz, Hz> for Hz {
    #[inline]
    fn rem(self, rhs: Hz) -> Hz {
        Hz(self.hz() % rhs.hz())
    }
}

impl Neg<Hz> for Hz {
    #[inline]
    fn neg(&self) -> Hz {
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

