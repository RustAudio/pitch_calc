extern crate serde;

mod hz {
    use std::fmt;
    use hz::Hz;
    use super::serde;

    impl serde::Serialize for Hz {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: serde::Serializer,
        {
            serializer.serialize_newtype_struct("Hz", &self.hz())
        }
    }

    impl<'de> serde::Deserialize<'de> for Hz {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where D: serde::Deserializer<'de>,
        {
            struct Visitor;

            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = Hz;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("a hertz integer")
                }

                fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
                    where E: serde::de::Error,
                {
                    Ok(Hz(v))
                }

                fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                    where D: serde::Deserializer<'de>,
                {
                    Ok(Hz(try!(super::serde::de::Deserialize::deserialize(deserializer))))
                }
            }

            deserializer.deserialize_newtype_struct("Hz", Visitor)
        }
    }

    #[test]
    fn test() {
        extern crate serde_json;

        let hz = Hz(440.0);
        let serialized = serde_json::to_string(&hz).unwrap();

        println!("{}", serialized);
        assert_eq!("440.0", &serialized);

        let deserialized: Hz = serde_json::from_str(&serialized).unwrap();

        println!("{:?}", deserialized);
        assert_eq!(hz, deserialized);
    }
}

mod letter {
    use std::fmt;
    use letter::Letter;
    use super::serde;

    impl serde::Serialize for Letter {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: serde::Serializer,
        {
            use Letter::*;
            match *self {
                C   => serializer.serialize_unit_variant("Letter", 0, "C"),
                Csh => serializer.serialize_unit_variant("Letter", 1, "Csh"),
                Db  => serializer.serialize_unit_variant("Letter", 2, "Db"),
                D   => serializer.serialize_unit_variant("Letter", 3, "D"),
                Dsh => serializer.serialize_unit_variant("Letter", 4, "Dsh"),
                Eb  => serializer.serialize_unit_variant("Letter", 5, "Eb"),
                E   => serializer.serialize_unit_variant("Letter", 6, "Eb"),
                F   => serializer.serialize_unit_variant("Letter", 7, "F"),
                Fsh => serializer.serialize_unit_variant("Letter", 8, "Fsh"),
                Gb  => serializer.serialize_unit_variant("Letter", 9, "Gb"),
                G   => serializer.serialize_unit_variant("Letter", 10, "G"),
                Gsh => serializer.serialize_unit_variant("Letter", 11, "Gsh"),
                Ab  => serializer.serialize_unit_variant("Letter", 12, "Ab"),
                A   => serializer.serialize_unit_variant("Letter", 13, "A"),
                Ash => serializer.serialize_unit_variant("Letter", 14, "Ash"),
                Bb  => serializer.serialize_unit_variant("Letter", 15, "Bb"),
                B   => serializer.serialize_unit_variant("Letter", 16, "B"),
            }
        }
    }

    impl<'de> serde::Deserialize<'de> for Letter {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where D: serde::Deserializer<'de>,
        {
            struct Visitor;

            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = Letter;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("a letter as integer or string")
                }

                fn visit_u64<E>(self, value: u64) -> Result<Letter, E>
                    where E: serde::de::Error,
                          {
                              let div = match value {
                                  0  => Letter::C,
                                  1  => Letter::Csh,
                                  2  => Letter::Db,
                                  3  => Letter::D,
                                  4  => Letter::Dsh,
                                  5  => Letter::Eb,
                                  6  => Letter::E,
                                  7  => Letter::F,
                                  8  => Letter::Fsh,
                                  9  => Letter::Gb,
                                  10 => Letter::G,
                                  11 => Letter::Gsh,
                                  12 => Letter::Ab,
                                  13 => Letter::A,
                                  14 => Letter::Ash,
                                  15 => Letter::Bb,
                                  16 => Letter::B,
                                  x => return Err(E::custom(format!("letter '{}' out of range", x))),
                              };
                              Ok(div)
                          }

                fn visit_str<E>(self, value: &str) -> Result<Letter, E>
                    where E: serde::de::Error,
                          {
                              let ltr = match value {
                                  "C"   => Letter::C,
                                  "Csh" => Letter::Csh,
                                  "Db"  => Letter::Db,
                                  "D"   => Letter::D,
                                  "Dsh" => Letter::Dsh,
                                  "Eb"  => Letter::Eb,
                                  "E"   => Letter::E,
                                  "F"   => Letter::F,
                                  "Fsh" => Letter::Fsh,
                                  "Gb"  => Letter::Gb,
                                  "G"   => Letter::G,
                                  "Gsh" => Letter::Gsh,
                                  "Ab"  => Letter::Ab,
                                  "A"   => Letter::A,
                                  "Ash" => Letter::Ash,
                                  "Bb"  => Letter::Bb,
                                  "B"   => Letter::B,
                                  x => return Err(E::custom(format!("wrong letter '{}'", x))),
                              };
                              Ok(ltr)
                          }
            }

            deserializer.deserialize_any(Visitor)
        }
    }

    #[test]
    fn test() {
        extern crate serde_json;

        let div = Letter::Fsh;
        let serialized = serde_json::to_string(&div).unwrap();

        println!("{}", serialized);
        assert_eq!("\"Fsh\"", &serialized);

        let deserialized: Letter = serde_json::from_str(&serialized).unwrap();

        println!("{:?}", deserialized);
        assert_eq!(div, deserialized);
    }
}

mod letter_octave {
    use std::fmt;
    use letter_octave::LetterOctave;
    use super::serde;

    impl serde::Serialize for LetterOctave {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: serde::Serializer,
        {
            use serde::serde::ser::SerializeTuple;
            let mut tup = serializer.serialize_tuple(2)?;
            tup.serialize_element(&self.0)?;
            tup.serialize_element(&self.1)?;
            tup.end()
        }
    }

    impl<'de> serde::Deserialize<'de> for LetterOctave {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where D: serde::Deserializer<'de>,
        {
            struct Visitor;

            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = LetterOctave;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("a letter sequence")
                }

                fn visit_seq<V>(self, mut visitor: V) -> Result<LetterOctave, V::Error>
                    where V: serde::de::SeqAccess<'de>,
                {
                    let letter = try!(visitor.next_element());
                    let octave = try!(visitor.next_element());

                    let letter = match letter {
                        Some(letter) => letter,
                        None => return Err(serde::de::Error::missing_field("letter")),
                    };

                    let octave = match octave {
                        Some(octave) => octave,
                        None => return Err(serde::de::Error::missing_field("octave")),
                    };

                    Ok(LetterOctave(letter, octave))
                }
            }

            deserializer.deserialize_tuple_struct("LetterOctave", 2, Visitor)
        }
    }

    #[test]
    fn test() {
        use letter::Letter;
        extern crate serde_json;

        let letter_octave = LetterOctave(Letter::A, 4);
        let serialized = serde_json::to_string(&letter_octave).unwrap();

        println!("{}", serialized);
        assert_eq!("[\"A\",4]", &serialized);

        let deserialized: LetterOctave = serde_json::from_str(&serialized).unwrap();

        println!("{:?}", deserialized);
        assert_eq!(letter_octave, deserialized);
    }
}

mod mel {
    use std::fmt;
    use mel::Mel;
    use super::serde;

    impl serde::Serialize for Mel {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: serde::Serializer,
        {
            serializer.serialize_newtype_struct("Mel", &self.mel())
        }
    }

    impl<'de> serde::Deserialize<'de> for Mel {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where D: serde::Deserializer<'de>,
        {
            struct Visitor;

            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = Mel;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("a mel float")
                }

                fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
                    where E: serde::de::Error,
                {
                    Ok(Mel(v))
                }

                fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                    where D: serde::Deserializer<'de>,
                {
                    Ok(Mel(try!(super::serde::de::Deserialize::deserialize(deserializer))))
                }
            }

            deserializer.deserialize_newtype_struct("Mel", Visitor)
        }
    }

    #[test]
    fn test() {
        extern crate serde_json;

        let mel = Mel(440.0);
        let serialized = serde_json::to_string(&mel).unwrap();

        println!("{}", serialized);
        assert_eq!("440.0", &serialized);

        let deserialized: Mel = serde_json::from_str(&serialized).unwrap();

        println!("{:?}", deserialized);
        assert_eq!(mel, deserialized);
    }
}

mod perc {
    use std::fmt;
    use perc::Perc;
    use super::serde;

    impl serde::Serialize for Perc {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: serde::Serializer,
        {
            serializer.serialize_newtype_struct("Perc", &self.perc())
        }
    }

    impl<'de> serde::Deserialize<'de> for Perc {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where D: serde::Deserializer<'de>,
        {
            struct Visitor;

            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = Perc;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("a perc float")
                }

                fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
                    where E: serde::de::Error,
                {
                    Ok(Perc(v))
                }

                fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                    where D: serde::Deserializer<'de>,
                {
                    Ok(Perc(try!(super::serde::de::Deserialize::deserialize(deserializer))))
                }
            }

            deserializer.deserialize_newtype_struct("Perc", Visitor)
        }
    }

    #[test]
    fn test() {
        extern crate serde_json;

        let perc = Perc(440.0);
        let serialized = serde_json::to_string(&perc).unwrap();

        println!("{}", serialized);
        assert_eq!("440.0", &serialized);

        let deserialized: Perc = serde_json::from_str(&serialized).unwrap();

        println!("{:?}", deserialized);
        assert_eq!(perc, deserialized);
    }
}

mod scaled_perc {
    use std::fmt;
    use scaled_perc::ScaledPerc;
    use super::serde;

    impl serde::Serialize for ScaledPerc {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: serde::Serializer,
        {
            use serde::serde::ser::SerializeTuple;
            let mut tup = serializer.serialize_tuple(2)?;
            tup.serialize_element(&self.0)?;
            tup.serialize_element(&self.1)?;
            tup.end()
        }
    }

    impl<'de> serde::Deserialize<'de> for ScaledPerc {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where D: serde::Deserializer<'de>,
        {
            struct Visitor;

            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = ScaledPerc;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("a scaled sequence")
                }

                fn visit_seq<V>(self, mut visitor: V) -> Result<ScaledPerc, V::Error>
                    where V: serde::de::SeqAccess<'de>,
                {
                    let perc = try!(visitor.next_element());
                    let weight = try!(visitor.next_element());

                    let perc = match perc {
                        Some(perc) => perc,
                        None => return Err(serde::de::Error::missing_field("perc")),
                    };

                    let weight = match weight {
                        Some(weight) => weight,
                        None => return Err(serde::de::Error::missing_field("weight")),
                    };

                    Ok(ScaledPerc(perc, weight))
                }
            }

            deserializer.deserialize_tuple_struct("ScaledPerc", 2, Visitor)
        }
    }

    #[test]
    fn test() {
        extern crate serde_json;

        let scaled_perc = ScaledPerc(0.5, 0.25);
        let serialized = serde_json::to_string(&scaled_perc).unwrap();

        println!("{}", serialized);
        assert_eq!("[0.5,0.25]", &serialized);

        let deserialized: ScaledPerc = serde_json::from_str(&serialized).unwrap();

        println!("{:?}", deserialized);
        assert_eq!(scaled_perc, deserialized);
    }
}

mod step {
    use std::fmt;
    use step::Step;
    use super::serde;

    impl serde::Serialize for Step {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: serde::Serializer,
        {
            serializer.serialize_newtype_struct("Step", &self.step())
        }
    }

    impl<'de> serde::Deserialize<'de> for Step {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where D: serde::Deserializer<'de>,
        {
            struct Visitor;

            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = Step;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("a step integer")
                }

                fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
                    where E: serde::de::Error,
                {
                    Ok(Step(v))
                }

                fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                    where D: serde::Deserializer<'de>,
                {
                    Ok(Step(try!(super::serde::de::Deserialize::deserialize(deserializer))))
                }
            }

            deserializer.deserialize_newtype_struct("Step", Visitor)
        }
    }

    #[test]
    fn test() {
        extern crate serde_json;

        let step = Step(440.0);
        let serialized = serde_json::to_string(&step).unwrap();

        println!("{}", serialized);
        assert_eq!("440.0", &serialized);

        let deserialized: Step = serde_json::from_str(&serialized).unwrap();

        println!("{:?}", deserialized);
        assert_eq!(step, deserialized);
    }
}
