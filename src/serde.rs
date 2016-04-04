extern crate serde;

mod hz {
    use hz::Hz;
    use super::serde;

    impl serde::Serialize for Hz {
        fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
            where S: serde::Serializer,
        {
            serializer.serialize_newtype_struct("Hz", self.hz())
        }
    }

    impl serde::Deserialize for Hz {
        fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
            where D: serde::Deserializer,
        {
            struct Visitor;

            impl serde::de::Visitor for Visitor {
                type Value = Hz;

                fn visit_f32<E>(&mut self, v: f32) -> Result<Self::Value, E>
                    where E: serde::de::Error,
                {
                    Ok(Hz(v))
                }

                fn visit_newtype_struct<D>(&mut self, deserializer: &mut D) -> Result<Self::Value, D::Error>
                    where D: serde::Deserializer,
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
        assert_eq!("440", &serialized);
        
        let deserialized: Hz = serde_json::from_str(&serialized).unwrap();

        println!("{:?}", deserialized);
        assert_eq!(hz, deserialized);
    }
}

mod letter {
    use letter::Letter;
    use super::serde;

    impl serde::Serialize for Letter {
        fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
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

    impl serde::Deserialize for Letter {
        fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
            where D: serde::Deserializer,
        {
            enum Field {
                C, Csh, Db, D, Dsh, Eb, E, F, Fsh, Gb, G, Gsh, Ab, A, Ash, Bb, B
            }

            impl serde::de::Deserialize for Field {
                fn deserialize<D>(deserializer: &mut D) -> Result<Field, D::Error>
                    where D: serde::Deserializer,
                {
                    struct FieldVisitor;

                    impl serde::de::Visitor for FieldVisitor {
                        type Value = Field;

                        fn visit_usize<E>(&mut self, value: usize) -> Result<Field, E>
                            where E: serde::de::Error,
                        {
                            let div = match value {
                                0  => Field::C,
                                1  => Field::Csh,
                                2  => Field::Db,
                                3  => Field::D,
                                4  => Field::Dsh,
                                5  => Field::Eb,
                                6  => Field::E,
                                7  => Field::F,
                                8  => Field::Fsh,
                                9  => Field::Gb,
                                10 => Field::G,
                                11 => Field::Gsh,
                                12 => Field::Ab,
                                13 => Field::A,
                                14 => Field::Ash,
                                15 => Field::Bb,
                                16 => Field::B,
                                _ => return Err(serde::de::Error::unknown_field(&value.to_string())),
                            };
                            Ok(div)
                        }

                        fn visit_str<E>(&mut self, value: &str) -> Result<Field, E>
                            where E: serde::de::Error,
                        {
                            match value {
                                "C"   => Ok(Field::C),
                                "Csh" => Ok(Field::Csh),
                                "Db"  => Ok(Field::Db),
                                "D"   => Ok(Field::D),
                                "Dsh" => Ok(Field::Dsh),
                                "Eb"  => Ok(Field::Eb),
                                "E"   => Ok(Field::E),
                                "F"   => Ok(Field::F),
                                "Fsh" => Ok(Field::Fsh),
                                "Gb"  => Ok(Field::Gb),
                                "G"   => Ok(Field::G),
                                "Gsh" => Ok(Field::Gsh),
                                "Ab"  => Ok(Field::Ab),
                                "A"   => Ok(Field::A),
                                "Ash" => Ok(Field::Ash),
                                "Bb"  => Ok(Field::Bb),
                                "B"   => Ok(Field::B),
                                _ => return Err(serde::de::Error::unknown_field(value)),
                            }
                        }
                    }

                    deserializer.deserialize(FieldVisitor)
                }
            }

            struct Visitor;

            impl serde::de::EnumVisitor for Visitor {
                type Value = Letter;

                fn visit<V>(&mut self, mut visitor: V) -> Result<Letter, V::Error>
                    where V: serde::de::VariantVisitor,
                {
                    let div = match try!(visitor.visit_variant()) {
                        Field::C   => Letter::C,
                        Field::Csh => Letter::Csh,
                        Field::Db  => Letter::Db,
                        Field::D   => Letter::D,
                        Field::Dsh => Letter::Dsh,
                        Field::Eb  => Letter::Eb,
                        Field::E   => Letter::E,
                        Field::F   => Letter::F,
                        Field::Fsh => Letter::Fsh,
                        Field::Gb  => Letter::Gb,
                        Field::G   => Letter::G,
                        Field::Gsh => Letter::Gsh,
                        Field::Ab  => Letter::Ab,
                        Field::A   => Letter::A,
                        Field::Ash => Letter::Ash,
                        Field::Bb  => Letter::Bb,
                        Field::B   => Letter::B,
                    };
                    try!(visitor.visit_unit());
                    Ok(div)
                }
            }

            const VARIANTS: &'static [&'static str] = &[
                "C", "Csh", "Db", "D", "Dsh", "Eb", "E", "F", "Fsh", "Gb", "G", "Gsh", "Ab", "A", "Ash",
                "Bb", "B",
            ];

            deserializer.deserialize_enum("Letter", VARIANTS, Visitor)
        }
    }

    #[test]
    fn test() {
        extern crate serde_json;

        let div = Letter::Fsh;
        let serialized = serde_json::to_string(&div).unwrap();

        println!("{}", serialized);
        assert_eq!("{\"Fsh\":[]}", &serialized);
        
        let deserialized: Letter = serde_json::from_str(&serialized).unwrap();

        println!("{:?}", deserialized);
        assert_eq!(div, deserialized);
    }
}

mod letter_octave {
    use letter_octave::LetterOctave;
    use super::serde;

    impl serde::Serialize for LetterOctave {
        fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
            where S: serde::Serializer,
        {
            struct Visitor<'a> {
                t: &'a LetterOctave,
                field_idx: u8,
            }

            impl<'a> serde::ser::SeqVisitor for Visitor<'a> {
                fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
                    where S: serde::Serializer,
                {
                    match self.field_idx {
                        0 => {
                            self.field_idx += 1;
                            Ok(Some(try!(serializer.serialize_tuple_struct_elt(self.t.0))))
                        },
                        1 => {
                            self.field_idx += 1;
                            Ok(Some(try!(serializer.serialize_tuple_struct_elt(self.t.1))))
                        },
                        _ => Ok(None),
                    }
                }

                fn len(&self) -> Option<usize> {
                    Some(2)
                }
            }

            serializer.serialize_tuple_struct("LetterOctave", Visitor { t: self, field_idx: 0 })
        }
    }

    impl serde::Deserialize for LetterOctave {
        fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
            where D: serde::Deserializer,
        {
            struct Visitor;

            impl serde::de::Visitor for Visitor {
                type Value = LetterOctave;

                fn visit_seq<V>(&mut self, mut visitor: V) -> Result<LetterOctave, V::Error>
                    where V: serde::de::SeqVisitor,
                {
                    let letter = try!(visitor.visit());
                    let octave = try!(visitor.visit());

                    let letter = match letter {
                        Some(letter) => letter,
                        None => return Err(serde::de::Error::missing_field("letter")),
                    };

                    let octave = match octave {
                        Some(octave) => octave,
                        None => return Err(serde::de::Error::missing_field("octave")),
                    };

                    try!(visitor.end());

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
        assert_eq!("[{\"A\":[]},4]", &serialized);
        
        let deserialized: LetterOctave = serde_json::from_str(&serialized).unwrap();

        println!("{:?}", deserialized);
        assert_eq!(letter_octave, deserialized);
    }
}

mod mel {
    use mel::Mel;
    use super::serde;

    impl serde::Serialize for Mel {
        fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
            where S: serde::Serializer,
        {
            serializer.serialize_newtype_struct("Mel", self.mel())
        }
    }

    impl serde::Deserialize for Mel {
        fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
            where D: serde::Deserializer,
        {
            struct Visitor;

            impl serde::de::Visitor for Visitor {
                type Value = Mel;

                fn visit_f32<E>(&mut self, v: f32) -> Result<Self::Value, E>
                    where E: serde::de::Error,
                {
                    Ok(Mel(v))
                }

                fn visit_newtype_struct<D>(&mut self, deserializer: &mut D) -> Result<Self::Value, D::Error>
                    where D: serde::Deserializer,
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
        assert_eq!("440", &serialized);
        
        let deserialized: Mel = serde_json::from_str(&serialized).unwrap();

        println!("{:?}", deserialized);
        assert_eq!(mel, deserialized);
    }
}

mod perc {
    use perc::Perc;
    use super::serde;

    impl serde::Serialize for Perc {
        fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
            where S: serde::Serializer,
        {
            serializer.serialize_newtype_struct("Perc", self.perc())
        }
    }

    impl serde::Deserialize for Perc {
        fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
            where D: serde::Deserializer,
        {
            struct Visitor;

            impl serde::de::Visitor for Visitor {
                type Value = Perc;

                fn visit_f64<E>(&mut self, v: f64) -> Result<Self::Value, E>
                    where E: serde::de::Error,
                {
                    Ok(Perc(v))
                }

                fn visit_newtype_struct<D>(&mut self, deserializer: &mut D) -> Result<Self::Value, D::Error>
                    where D: serde::Deserializer,
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
        assert_eq!("440", &serialized);
        
        let deserialized: Perc = serde_json::from_str(&serialized).unwrap();

        println!("{:?}", deserialized);
        assert_eq!(perc, deserialized);
    }
}

mod scaled_perc {
    use scaled_perc::ScaledPerc;
    use super::serde;

    impl serde::Serialize for ScaledPerc {
        fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
            where S: serde::Serializer,
        {
            struct Visitor<'a> {
                t: &'a ScaledPerc,
                field_idx: u8,
            }

            impl<'a> serde::ser::SeqVisitor for Visitor<'a> {
                fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
                    where S: serde::Serializer,
                {
                    match self.field_idx {
                        0 => {
                            self.field_idx += 1;
                            Ok(Some(try!(serializer.serialize_tuple_struct_elt(self.t.0))))
                        },
                        1 => {
                            self.field_idx += 1;
                            Ok(Some(try!(serializer.serialize_tuple_struct_elt(self.t.1))))
                        },
                        _ => Ok(None),
                    }
                }

                fn len(&self) -> Option<usize> {
                    Some(2)
                }
            }

            serializer.serialize_tuple_struct("ScaledPerc", Visitor { t: self, field_idx: 0 })
        }
    }

    impl serde::Deserialize for ScaledPerc {
        fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
            where D: serde::Deserializer,
        {
            struct Visitor;

            impl serde::de::Visitor for Visitor {
                type Value = ScaledPerc;

                fn visit_seq<V>(&mut self, mut visitor: V) -> Result<ScaledPerc, V::Error>
                    where V: serde::de::SeqVisitor,
                {
                    let perc = try!(visitor.visit());
                    let weight = try!(visitor.visit());

                    let perc = match perc {
                        Some(perc) => perc,
                        None => return Err(serde::de::Error::missing_field("perc")),
                    };

                    let weight = match weight {
                        Some(weight) => weight,
                        None => return Err(serde::de::Error::missing_field("weight")),
                    };

                    try!(visitor.end());

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
    use step::Step;
    use super::serde;

    impl serde::Serialize for Step {
        fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
            where S: serde::Serializer,
        {
            serializer.serialize_newtype_struct("Step", self.step())
        }
    }

    impl serde::Deserialize for Step {
        fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
            where D: serde::Deserializer,
        {
            struct Visitor;

            impl serde::de::Visitor for Visitor {
                type Value = Step;

                fn visit_f32<E>(&mut self, v: f32) -> Result<Self::Value, E>
                    where E: serde::de::Error,
                {
                    Ok(Step(v))
                }

                fn visit_newtype_struct<D>(&mut self, deserializer: &mut D) -> Result<Self::Value, D::Error>
                    where D: serde::Deserializer,
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
        assert_eq!("440", &serialized);
        
        let deserialized: Step = serde_json::from_str(&serialized).unwrap();

        println!("{:?}", deserialized);
        assert_eq!(step, deserialized);
    }
}
