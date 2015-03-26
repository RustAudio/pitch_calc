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
use std::num::Float;

fn main() {

    println!("");

    // You can convert midi-step to letter octave
    assert!(Step(64.0).to_letter_octave() == LetterOctave(Letter::E, 4));
    // Or hz to letter octave.
    assert!(Hz(220.0).letter_octave() == (Letter::A, 3));
    // Or the other way around.
    assert!(LetterOctave(Letter::A, 4).to_hz() == Hz(440.0));

    // This will print a bunch of midi-steps in their musical letter form.
    for i in 0..12 {
        println!("{:?}: {:?}", i, Step(i as f32).letter());
    }

    println!("");

    // This will print A at octaves 0 - 9 next to the equivalent frequency in hz.
    for i in 0..10 {
        println!("A{:?} == {:?}hz", i, LetterOctave(Letter::A, i).hz())
    }

    println!("");

    // The `ToPerc` trait allows us to convert any pitch to a percentage between the human hearing
    // range (20hz - 20_000hz)
    println!("20hz == {:?}%", Hz(20.0).perc() * 100.0);
    println!("20_000hz == {:?}%", Hz(20_000.0).perc() * 100.0);
    println!("10_010hz == {:?}%", Hz(10_010.0).perc() * 100.0);

    println!("");

    // We can also "weight" a particular area of the frequency spectrum using the scaled
    // percentage type.
    // - weight > 1 will weight the low end of the spectrum.
    // - 0 < weight < 1 will weight the high end of the spectrum.
    // More precisely, `ScaledPerc` represents ``` (Percentage / 100.0)^weight ```.
    let weight = 3.0;
    for i in 0..10 {
        let perc = ScaledPerc(i as f64 / 10.0, weight);
        println!("{:?}% == {:?}hz is closest to {:?}", perc.perc(), perc.hz(), perc.letter_octave());
    }

    // "Mels" can be used to represent an evenly distributed range of our pitch perception.
    println!("20hz in Mels == {:?}", Hz(20.0).mel());
    println!("440hz in Mels == {:?}", Hz(440.0).mel());
    println!("2_000hz in Mels == {:?}", Hz(2_000.0).mel());
    println!("10_000hz in Mels == {:?}", Hz(10_000.0).mel());
    println!("20_000hz in Mels == {:?}", Hz(20_000.0).mel());

    // Test a big chain of conversions..
    let a_4 = LetterOctave(Letter::A, 4)
                .to_hz()
                .to_perc()
                .to_mel()
                .to_step()
                .to_perc()
                .to_hz()
                .to_step()
                .to_letter_octave()
                .hz().round() as i32;
    assert!(a_4 == 440, "A4 == {:?}", a_4);

    println!("Great success!");

}


