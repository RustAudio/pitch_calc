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
    ToHz,
    ToLetterOctave,
    ToPerc,
};


fn main() {

    println!("")

    assert!(Step(52.0).to_letter_octave() == LetterOctave(Letter::E, 4))
    assert!(Hz(220.0).to_letter_octave() == LetterOctave(Letter::A, 3))
    assert!(LetterOctave(Letter::A, 4).to_hz() == Hz(440.0))

    for i in range(0u, 12) {
        println!("{}: {}", i, Step(i as f32).letter())
    }

    println!("")

    for i in range(0i32, 10) {
        println!("A{} == {}hz", i, LetterOctave(Letter::A, i).hz())
    }

    println!("")

    println!("20hz == {}%", Hz(20.0).perc() * 100.0)
    println!("20_000hz == {}%", Hz(20_000.0).perc() * 100.0)
    println!("10_010hz == {}%", Hz(10_010.0).perc() * 100.0)

    println!("")

    let weight = 3.0;
    for i in range(0u, 10) {
        // ScaledPerc represents ( Percentage / 100.0 ).powf(weight)
        let perc = ScaledPerc(i as f64 / 10.0, weight);
        println!("{}% == {}hz is closest to {}", perc.perc(), perc.hz(), perc.letter_octave())
    }

    println!("Great success!");

}

