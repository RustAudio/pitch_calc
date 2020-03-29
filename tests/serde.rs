#![cfg(features = "serde")]

use pitch_calc::hz::Hz;
use pitch_calc::letter::Letter;
use pitch_calc::letter_octave::LetterOctave;
use pitch_calc::mel::Mel;
use pitch_calc::perc::Perc;
use pitch_calc::scaled_perc::ScaledPerc;
use pitch_calc::step::Step;

#[test]
fn test_hz() {
    let hz = Hz(440.0);
    let serialized = serde_json::to_string(&hz).unwrap();

    println!("{}", serialized);
    assert_eq!("440.0", &serialized);

    let deserialized: Hz = serde_json::from_str(&serialized).unwrap();

    println!("{:?}", deserialized);
    assert_eq!(hz, deserialized);
}

#[test]
fn test_letter() {
    let div = Letter::Fsh;
    let serialized = serde_json::to_string(&div).unwrap();

    println!("{}", serialized);
    assert_eq!("\"Fsh\"", &serialized);

    let deserialized: Letter = serde_json::from_str(&serialized).unwrap();

    println!("{:?}", deserialized);
    assert_eq!(div, deserialized);
}

#[test]
fn test_letter_octave() {
    let letter_octave = LetterOctave(Letter::A, 4);
    let serialized = serde_json::to_string(&letter_octave).unwrap();

    println!("{}", serialized);
    assert_eq!("[\"A\",4]", &serialized);

    let deserialized: LetterOctave = serde_json::from_str(&serialized).unwrap();

    println!("{:?}", deserialized);
    assert_eq!(letter_octave, deserialized);
}

#[test]
fn test_mel() {
    let mel = Mel(440.0);
    let serialized = serde_json::to_string(&mel).unwrap();

    println!("{}", serialized);
    assert_eq!("440.0", &serialized);

    let deserialized: Mel = serde_json::from_str(&serialized).unwrap();

    println!("{:?}", deserialized);
    assert_eq!(mel, deserialized);
}

#[test]
fn test_perc() {
    let perc = Perc(440.0);
    let serialized = serde_json::to_string(&perc).unwrap();

    println!("{}", serialized);
    assert_eq!("440.0", &serialized);

    let deserialized: Perc = serde_json::from_str(&serialized).unwrap();

    println!("{:?}", deserialized);
    assert_eq!(perc, deserialized);
}

#[test]
fn test_scaled_perc() {
    let scaled_perc = ScaledPerc(0.5, 0.25);
    let serialized = serde_json::to_string(&scaled_perc).unwrap();

    println!("{}", serialized);
    assert_eq!("[0.5,0.25]", &serialized);

    let deserialized: ScaledPerc = serde_json::from_str(&serialized).unwrap();

    println!("{:?}", deserialized);
    assert_eq!(scaled_perc, deserialized);
}

#[test]
fn test_step() {
    let step = Step(440.0);
    let serialized = serde_json::to_string(&step).unwrap();

    println!("{}", serialized);
    assert_eq!("440.0", &serialized);

    let deserialized: Step = serde_json::from_str(&serialized).unwrap();

    println!("{:?}", deserialized);
    assert_eq!(step, deserialized);
}
