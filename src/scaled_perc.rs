
use super::{
    Hz,
    ToHz,
    LetterOctave,
    ToLetterOctave,
    Perc,
    ToPerc,
    Step,
    ToStep,
    hz_from_scaled_perc,
    letter_octave_from_scaled_perc,
    perc_from_scaled_perc,
    step_from_scaled_perc,
};

pub type ScaleWeight = f32;
pub const DEFAULT_SCALE_WEIGHT: ScaleWeight = 4.0;

/// Pitch representation in the form of a scaled percentage between the min and max hz.
#[deriving(Show, Copy, Clone, Encodable, Decodable)]
pub struct ScaledPerc(pub f64, pub ScaleWeight);

impl ScaledPerc {
    /// A constructor for a ScaledPerc that uses the default weight.
    pub fn new(perc: f64) -> ScaledPerc { ScaledPerc(perc, DEFAULT_SCALE_WEIGHT) }
}

/// For types that can be represented in hz.
pub trait ToScaledPerc {
    /// Return the current type in the form of a ScaledPerc.
    fn to_scaled_perc_with_weight(&self, weight: ScaleWeight) -> ScaledPerc;
    /// Return the current type in the form of a ScaledPerc.
    fn to_scaled_perc(&self) -> ScaledPerc {
        self.to_scaled_perc_with_weight(DEFAULT_SCALE_WEIGHT)
    }
    /// Return the value as a scaled percentage.
    #[inline]
    fn scaled_perc(&self) -> f64 {
        let ScaledPerc(perc, _) = self.to_scaled_perc();
        perc
    }
    /// Return the scale weight.
    #[inline]
    fn scale_weight(&self) -> ScaleWeight {
        let ScaledPerc(_, weight) = self.to_scaled_perc();
        weight
    }
}

impl ToHz for ScaledPerc {
    #[inline]
    fn to_hz(&self) -> Hz {
        let ScaledPerc(perc, weight) = *self;
        Hz(hz_from_scaled_perc(perc, weight))
    }
}

impl ToLetterOctave for ScaledPerc {
    #[inline]
    fn to_letter_octave(&self) -> LetterOctave {
        let ScaledPerc(perc, weight) = *self;
        let (letter, octave) = letter_octave_from_scaled_perc(perc, weight);
        LetterOctave(letter, octave)
    }
}

impl ToPerc for ScaledPerc {
    #[inline]
    fn to_perc(&self) -> Perc {
        let ScaledPerc(perc, weight) = *self;
        Perc(perc_from_scaled_perc(perc, weight))
    }
}

impl ToScaledPerc for ScaledPerc {
    #[inline]
    fn to_scaled_perc_with_weight(&self, weight: ScaleWeight) -> ScaledPerc {
        let ScaledPerc(perc, _) = *self;
        ScaledPerc(perc, weight)
    }
}

impl ToStep for ScaledPerc {
    #[inline]
    fn to_step(&self) -> Step {
        let ScaledPerc(perc, weight) = *self;
        Step(step_from_scaled_perc(perc, weight))
    }
}

impl Add<ScaledPerc, ScaledPerc> for ScaledPerc {
    #[inline]
    fn add(self, rhs: ScaledPerc) -> ScaledPerc {
        (self.to_perc() + rhs.to_perc()).to_scaled_perc_with_weight(self.scale_weight())
    }
}

impl Sub<ScaledPerc, ScaledPerc> for ScaledPerc {
    #[inline]
    fn sub(self, rhs: ScaledPerc) -> ScaledPerc {
        (self.to_perc() - rhs.to_perc()).to_scaled_perc_with_weight(self.scale_weight())
    }
}

impl Mul<ScaledPerc, ScaledPerc> for ScaledPerc {
    #[inline]
    fn mul(self, rhs: ScaledPerc) -> ScaledPerc {
        (self.to_perc() * rhs.to_perc()).to_scaled_perc_with_weight(self.scale_weight())
    }
}

impl Div<ScaledPerc, ScaledPerc> for ScaledPerc {
    #[inline]
    fn div(self, rhs: ScaledPerc) -> ScaledPerc {
        (self.to_perc() / rhs.to_perc()).to_scaled_perc_with_weight(self.scale_weight())
    }
}

impl Rem<ScaledPerc, ScaledPerc> for ScaledPerc {
    #[inline]
    fn rem(self, rhs: ScaledPerc) -> ScaledPerc {
        (self.to_perc() % rhs.to_perc()).to_scaled_perc_with_weight(self.scale_weight())
    }
}

impl Neg<ScaledPerc> for ScaledPerc {
    #[inline]
    fn neg(&self) -> ScaledPerc {
        ScaledPerc(-self.scaled_perc(), self.scale_weight())
    }
}

impl PartialEq for ScaledPerc {
    #[inline]
    fn eq(&self, other: &ScaledPerc) -> bool {
        self.perc() == other.perc()
    }
}

impl Eq for ScaledPerc {}

impl PartialOrd for ScaledPerc {
    #[inline]
    fn partial_cmp(&self, other: &ScaledPerc) -> Option<Ordering> {
        self.to_perc().partial_cmp(&other.to_perc())
    }
}

impl Ord for ScaledPerc {
    #[inline]
    fn cmp(&self, other: &ScaledPerc) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

