
use super::{
    Hz,
    ToHz,
    LetterOctave,
    ToLetterOctave,
    ScaledPerc,
    ScaleWeight,
    ToScaledPerc,
    Step,
    ToStep,
    hz_from_perc,
    letter_octave_from_perc,
    scaled_perc_from_perc,
    step_from_perc,
};

/// Pitch representation in the form of a percentage between the min and max hz.
#[deriving(Show, Copy, Clone, Encodable, Decodable)]
pub struct Perc(pub f64);

/// For types that can be represented as a frequency percentage.
pub trait ToPerc {
    /// Return the current type in the form of a Perc.
    fn to_perc(&self) -> Perc;
    /// Return the value as a percentage.
    fn perc(&self) -> f64 { let Perc(perc) = self.to_perc(); perc }
}

impl ToHz for Perc {
    #[inline]
    fn to_hz(&self) -> Hz {
        let Perc(perc) = *self;
        Hz(hz_from_perc(perc))
    }
}

impl ToLetterOctave for Perc {
    #[inline]
    fn to_letter_octave(&self) -> LetterOctave {
        let Perc(perc) = *self;
        let (letter, octave) = letter_octave_from_perc(perc);
        LetterOctave(letter, octave)
    }
}

impl ToPerc for Perc {
    #[inline]
    fn to_perc(&self) -> Perc { *self }
}

impl ToScaledPerc for Perc {
    #[inline]
    fn to_scaled_perc_with_weight(&self, weight: ScaleWeight) -> ScaledPerc {
        let Perc(perc) = *self;
        ScaledPerc(scaled_perc_from_perc(perc, weight), weight)
    }
}

impl ToStep for Perc {
    #[inline]
    fn to_step(&self) -> Step {
        let Perc(perc) = *self;
        Step(step_from_perc(perc))
    }
}

impl Add<Perc, Perc> for Perc {
    #[inline]
    fn add(self, rhs: Perc) -> Perc {
        Perc(self.perc() + rhs.perc())
    }
}

impl Sub<Perc, Perc> for Perc {
    #[inline]
    fn sub(self, rhs: Perc) -> Perc {
        Perc(self.perc() - rhs.perc())
    }
}

impl Mul<Perc, Perc> for Perc {
    #[inline]
    fn mul(self, rhs: Perc) -> Perc {
        Perc(self.perc() * rhs.perc())
    }
}

impl Div<Perc, Perc> for Perc {
    #[inline]
    fn div(self, rhs: Perc) -> Perc {
        Perc(self.perc() / rhs.perc())
    }
}

impl Rem<Perc, Perc> for Perc {
    #[inline]
    fn rem(self, rhs: Perc) -> Perc {
        Perc(self.perc() % rhs.perc())
    }
}

impl Neg<Perc> for Perc {
    #[inline]
    fn neg(&self) -> Perc {
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

