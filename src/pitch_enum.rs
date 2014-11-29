//!
//!  NOTE:
//!     
//!     This module is untested and inactive. I was working on it before I decided that
//!     it would be better to have individual types and traits rather than an enum with
//!     variants. If you feel like this module should live, go for it!
//!

pub use pitch::pitch::*;

pub const DEFAULT_SCALE_WEIGHT: ScaleWeight = 4.0;

/// When converting between scaled percentage 'a' with a^n, ScaleWeight is 'n'.
pub type ScaleWeight = f32;

mod pitch {

    use letter::{Letter, Octave};
    use super::ScaleWeight;

    /// Several different representations of musical Pitch.
    #[deriving(Show, Encodable, Decodable)]
    pub enum Pitch {
        Hz(f32),
        Step(f32),
        Perc(f64),
        ScaledPerc(f64, ScaleWeight),
        LetterOctave(Letter, Octave),
    }

}

impl Pitch {

    /// Return the pitch value as hz.
    #[inline]
    fn hz(&self) -> f32 {
        match *self {
            Hz(hz) => hz,
            LetterOctave(letter, octave) => hz_from_letter_octave(letter, octave),
            Perc(perc) => hz_from_perc(perc),
            ScaledPerc(perc, weight) => hz_from_scaled_perc(perc, weight),
            Step(step) => hz_from_step(step),
        }
    }

    /// Return the pitch value as (Letter, Octave).
    #[inline]
    fn letter_octave(&self) -> (Letter, Octave) {
        match *self {
            Hz(hz) => letter_octave_from_hz(hz),
            LetterOctave(_, letter, octave) => (letter, octave),
            Perc(perc) => letter_octave_from_perc(perc),
            ScaledPerc(perc, weight) => letter_octave_from_scaled_perc(perc, weight),
            Step(step) => letter_octave_from_step(step),
        }
    }

    /// Return the pitch value as a percentage.
    #[inline]
    fn perc(&self) -> f64 {
        match *self {
            Hz(hz) => perc_from_hz(hz),
            LetterOctave(letter, octave) => perc_from_letter_octave(letter, octave),
            Perc(perc) => perc,
            ScaledPerc(perc, weight) => perc_from_scaled_perc(perc, weight),
            Step(step) => perc_from_step(step),
        }
    }

    /// Return the pitch value as a scaled percentage.
    #[inline]
    fn scaled_perc_with_weight(&self, weight: ScaleWeight) -> f64 {
        match *self {
            Hz(hz) => scaled_perc_from_hz(hz, weight),
            LetterOctave(letter, octave) => scaled_perc_from_letter_octave(letter, octave, weight),
            Perc(perc) => scaled_perc_from_perc(perc, weight),
            ScaledPerc(perc, weight) => perc,
            Step(step) => scaled_perc_from_step(step, weight),
        }
    }

    /// Return the pitch value as a scaled percentage.
    #[inline]
    fn scaled_perc(&self) -> f64 {
        self.scaled_perc_with_weight(DEFAULT_SCALE_WEIGHT)
    }

    /// Return the pitch value as a Step.
    #[inline]
    fn step(&self) -> f32 {
        match *self {
            Hz(hz) => step_from_hz(hz),
            LetterOctave(letter, octave) => step_from_letter_octave(letter, octave),
            Perc(perc) => step_from_perc(perc),
            ScaledPerc(perc, weight) => step_from_scaled_perc(perc, weight),
            Step(step) => step,
        }
    }

    /// Return the current variant as a Hz variant.
    #[inline]
    fn to_hz(&self) -> Pitch {
        Hz(self.hz())
    }

    /// Return the current variant as a LetterOctave variant.
    #[inline]
    fn to_letter_octave(&self) -> Pitch {
        let (letter, octave) = self.letter_octave();
        LetterOctave(letter, octave)
    }

    /// Return the current variant as a Perc variant.
    #[inline]
    fn to_perc(&self) -> Pitch {
        Perc(self.perc())
    }

    /// Return the current variant as a ScaledPerc variant.
    #[inline]
    fn to_scaled_perc_with_weight(&self, weight: ScaleWeight) -> Pitch {
        ScaledPerc(self.scaled_perc_with_weight(weight), weight)
    }

    /// Return the current variant as a ScaledPerc variant.
    #[inline]
    fn to_scaled_perc(&self) -> Pitch {
        self.to_scaled_perc_with_weight(DEFAULT_SCALE_WEIGHT)
    }

    /// Return the current variant as a Step variant.
    #[inline]
    fn to_step(&self) -> Pitch {
        Step(self.step())
    }

    /// Transform the current variant into a Hz variant.
    #[inline]
    fn as_hz(&mut self) {
        let hz = self.to_hz();
        *self = hz;
    }

    /// Transform the current variant into a LetterOctave variant.
    #[inline]
    fn as_letter_octave(&mut self) {
        let letter_octave = self.to_letter_octave();
        *self = letter_octave;
    }

    /// Transform the current variant into a Perc variant.
    #[inline]
    fn as_perc(&mut self) {
        let perc = self.to_perc();
        *self = perc;
    }

    /// Transform the current variant into a ScaledPerc variant.
    #[inline]
    fn as_scaled_perc_with_weight(&mut self, weight: ScaleWeight) {
        let scale_weight = self.to_scale_weight_with_weight(weight);
        *self = scale_weight;
    }

    /// Transform the current variant into a ScaledPerc variant.
    #[inline]
    fn as_scaled_perc(&mut self) {
        self.as_scaled_weight_with_weight(DEFAULT_SCALE_WEIGHT)
    }

    /// Transform the current variant into a ScaledPerc variant.
    #[inline]
    fn as_step(&mut self) {
        let step = self.to_step();
        *self = step;
    }

}

impl Add<Pitch, Pitch> for Pitch {
    fn add(&self, rhs: &Pitch) -> Pitch {
        match (*self, *rhs) {
            (Hz(hz), Hz(other_hz)) => Hz(hz + other_hz),
            (LetterOctave(..), LetterOctave(..)) |
            (LetterOctave(..), Step(_)) |
            (Step(_), LetterOctave(..)) => Step(self.step() + rhs.step()),
            (Perc(perc), Perc(other_perc)) => Perc(perc + other_perc),
            (Step(step), Step(other_step)) => Step(step + other_step),
            _ => Perc(self.perc() + rhs.perc()),
        }
    }
}

impl Sub<Pitch, Pitch> for Pitch {
    fn sub(&self, rhs: &Pitch) -> Pitch {
        match (*self, *rhs) {
            (Hz(hz), Hz(other_hz)) => Hz(hz - other_hz),
            (LetterOctave(..), LetterOctave(..)) |
            (LetterOctave(..), Step(_)) |
            (Step(_), LetterOctave(..)) => Step(self.step() - rhs.step()),
            (Perc(perc), Perc(other_perc)) => Perc(perc - other_perc),
            (Step(step), Step(other_step)) => Step(step - other_step),
            _ => Perc(self.perc() - rhs.perc()),
        }
    }
}

impl Mul<Pitch, Pitch> for Pitch {
    fn mul(&self, rhs: &Pitch) -> Pitch {
        match (*self, *rhs) {
            (Hz(hz), Hz(other_hz)) => Hz(hz * other_hz),
            (LetterOctave(..), LetterOctave(..)) |
            (LetterOctave(..), Step(_)) |
            (Step(_), LetterOctave(..)) => Step(self.step() * rhs.step()),
            (Perc(perc), Perc(other_perc)) => Perc(perc * other_perc),
            (Step(step), Step(other_step)) => Step(step * other_step),
            _ => Perc(self.perc() * rhs.perc()),
        }
    }
}

impl Div<Pitch, Pitch> for Pitch {
    fn div(&self, rhs: &Pitch) -> Pitch {
        match (*self, *rhs) {
            (Hz(hz), Hz(other_hz)) => Hz(hz / other_hz),
            (LetterOctave(..), LetterOctave(..)) |
            (LetterOctave(..), Step(_)) |
            (Step(_), LetterOctave(..)) => Step(self.step() / rhs.step()),
            (Perc(perc), Perc(other_perc)) => Perc(perc / other_perc),
            (Step(step), Step(other_step)) => Step(step / other_step),
            _ => Perc(self.perc() / rhs.perc()),
        }
    }
}

impl Neg<Pitch> for Pitch {
    fn neg(&self) -> Pitch {
        match *self {
            Hz(hz) => Hz(-hz),
            Perc(perc) => Perc(-perc),
            Step(step) => Step(-step),
            LetterOctave(..) => Step(-self.step()),
            _ => Perc(-self.perc()),
        }
    }
}

impl Rem<Pitch, Pitch> for Pitch {
    fn rem(&self, rhs: &Pitch) -> Pitch {
        match (*self, *rhs) {
            (Hz(hz), Hz(other_hz)) => Hz(hz % other_hz),
            (LetterOctave(..), LetterOctave(..)) |
            (LetterOctave(..), Step(_)) |
            (Step(_), LetterOctave(..)) => Step(self.step() % rhs.step()),
            (Perc(perc), Perc(other_perc)) => Perc(perc % other_perc),
            (Step(step), Step(other_step)) => Step(step % other_step),
            _ => Perc(self.perc() % rhs.perc()),
        }
    }
}

impl PartialEq for Pitch {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (Hz(hz), Hz(other_hz)) => hz == other_hz,
            (LetterOctave(l, o), LetterOctave(o_l, o_o)) => l == o && o_l == o_o,
            (LetterOctave(..), Step(_)) |
            (Step(_), LetterOctave(..)) => self.step() == rhs.step(),
            (Perc(perc), Perc(other_perc)) => perc == other_perc,
            (Step(step), Step(other_step)) => step == other_step,
            _ => self.perc() == other.perc(),
        }
    }
}

impl Eq for Pitch {}

impl PartialOrd for Pitch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (*self, *other) {
            (Hz(hz), Hz(other_hz)) => hz.partial_cmp(&other_hz),
            (Step(step), Step(other_step)) => step.partial_cmp(&other_step),
            (Perc(perc), Perc(other_perc)) => perc.partial_cmp(&other_perc),
            (LetterOctave(l, o), LetterOctave(o_l, o_o)) => match o.partial_cmp(&o_o) {
                Some(Equal) => l.to_i8().unwrap().partial_cmp(&o_l.to_i8().unwrap()),
                ordering => ordering,
            },
            _ => self.perc().partial_cmp(&other.perc()),
        }
    }
}

impl Ord for Pitch {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// A trait for returning a reference to the Pitch enum.
pub trait HasPitch {
    /// Return a Pitch enum.
    fn pitch(&self) -> Pitch;
    /// Return a reference to a Pitch enum.
    fn pitch_ref(&self) -> &Pitch;
    /// Return a mutable reference to a Pitch enum.
    fn pitch_mut(&mut self) -> &mut Pitch;
}

macro_rules! impl_has_pitch(
    ($T:ty, $self:ident $(.$pitch:ident)*) => (
        impl ::pitch::HasPitch for $T {
            fn pitch(&self) -> Pitch { $self $(.$pitch)* }
            fn pitch_ref(&self) -> &Pitch { &(*$self) $(.$pitch)* }
            fn pitch_mut(&mut self) -> &mut Pitch { &mut (*$self) $(.$pitch)* }
        }
    )
)

