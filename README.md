# pitch_calc [![Build Status](https://travis-ci.org/RustAudio/pitch_calc.svg?branch=master)](https://travis-ci.org/RustAudio/pitch_calc)

A library for musical pitch conversions!

`pitch_calc` provides functions and methods for converting between frequency, midi-step and letter-octave.

It looks like this.

```Rust
assert!(Hz(440.0).letter_octave() == (A, 4))
```


Types
-----

- [Hz](http://en.wikipedia.org/wiki/Hertz) ("times per second").
- [LetterOctave](http://en.wikipedia.org/wiki/Letter_notation) (musical letter notation).
- [Mel](http://en.wikipedia.org/wiki/Mel_scale) (a perceptual scale of pitches judged by listeners to be equal in distance from one another).
- Perc (Percentage of the human hearing range (20hz - 20khz)).
- ScaledPerc (Scaled percentage of the human hearing range).
- [Step](http://en.wikipedia.org/wiki/Semitone) (MIDI semitone steps).



Usage
-----

Add the following to your Cargo.toml.

```
[dependencies.pitch_calc]
git = "https://github.com/RustAudio/pitch_calc"
```
or
```
[dependencies]
pitch_calc = "X.X.X"
```
where "X.X.X" is the version you want (find the latest version in the Cargo.toml).

See [the example](https://github.com/RustAudio/pitch_calc/blob/master/examples/test.rs) for a simple demonstration.

