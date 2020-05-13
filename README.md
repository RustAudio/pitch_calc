# pitch_calc [![Build Status](https://travis-ci.org/RustAudio/pitch_calc.svg?branch=master)](https://travis-ci.org/RustAudio/pitch_calc) [![Crate info](https://img.shields.io/crates/v/pitch_calc.svg)](https://crates.io/crates/pitch_calc) [![API docs](https://img.shields.io/badge/docs.rs-pitch__calc-green.svg)](https://docs.rs/pitch_calc/)

A library for musical pitch conversions!

`pitch_calc` provides functions and methods for converting between frequency, midi-step and letter-octave.

It looks like this.

```Rust
assert!(Hz(440.0).letter_octave() == (A, 4))
```

Types
-----

- [**Hz**](http://en.wikipedia.org/wiki/Hertz): "times per second".
- [**LetterOctave**](http://en.wikipedia.org/wiki/Letter_notation): musical letter notation.
- [**Mel**](http://en.wikipedia.org/wiki/Mel_scale): a perceptual scale of pitches judged by listeners to be equal in distance from one another.
- **Perc**: Percentage of the average human hearing range (20hz - 20khz).
- **ScaledPerc**: Scaled percentage of the human hearing range.
- [**Step**](http://en.wikipedia.org/wiki/Semitone): MIDI semitone steps.
