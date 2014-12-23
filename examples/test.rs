//! 
//! An example of the pitch_calc crate in action.
//!

extern crate pitch_calc;

use pitch_calc::{
    Hz,
    Letter,
    LetterOctave,
    ScaledPerc,
    Step,
};

fn main() {

    println!("")

    // You can convert midi-step to letter octave
    assert!(Step(52.0).to_letter_octave() == LetterOctave(Letter::E, 4))
    // Or hz to letter octave.
    assert!(Hz(220.0).letter_octave() == (Letter::A, 3))
    // Or the other way around.
    assert!(LetterOctave(Letter::A, 4).to_hz() == Hz(440.0))

    // This will print a bunch of midi-steps in their musical letter form.
    for i in range(0u, 12) {
        println!("{}: {}", i, Step(i as f32).letter())
    }

    println!("")

    // This will print A at octaves 0 - 9 next to the equivalent frequency in hz.
    for i in range(0i32, 10) {
        println!("A{} == {}hz", i, LetterOctave(Letter::A, i).hz())
    }

    println!("")

    // The `ToPerc` trait allows us to convert any pitch to a percentage between the human hearing
    // range (20hz - 20_000hz)
    println!("20hz == {}%", Hz(20.0).perc() * 100.0)
    println!("20_000hz == {}%", Hz(20_000.0).perc() * 100.0)
    println!("10_010hz == {}%", Hz(10_010.0).perc() * 100.0)

    println!("")

    // We can also "weight" a particular area of the frequency spectrum using the scaled
    // percentage type.
    // - weight > 1 will weight the low end of the spectrum.
    // - 0 < weight < 1 will weight the high end of the spectrum.
    // More precisely, `ScaledPerc` represents ``` (Percentage / 100.0)^weight ```.
    let weight = 3.0;
    for i in range(0u, 10) {
        let perc = ScaledPerc(i as f64 / 10.0, weight);
        println!("{}% == {}hz is closest to {}", perc.perc(), perc.hz(), perc.letter_octave())
    }

    println!("Great success!");

}

