pitch_calc
==========

A library for musical pitch conversions!

`pitch_calc` provides functions and traits for converting between frequency, midi-step and letter-octave.

It looks like this.

```Rust
assert!(Hz(440.0).letter_octave() == (A, 4))
```

Usage
-----

Add the following to your Cargo.toml.

```
[dependencies.pitch_calc]
git = "https://github.com/RustAudio/pitch_calc"
```

See [the example](https://github.com/RustAudio/pitch_calc/blob/master/examples/test.rs) for a simple demonstration.

