#[cfg(test)]
mod chords_test {
    use crate::chords::chords::{Chord, ChordType};
    use crate::intervals::intervals::Interval;
    use crate::intervals::intervals::IntervalType::{Fifth, Seventh, Third};
    use crate::intervals::intervals::Quality::{Major, Perfect};
    use crate::keys::keys::Key;
    use crate::keys::keys::Key::{A, B, C, D, E, F, G};
    use crate::notes::notes::Alteration::{DoubleFlat, DoubleSharp, Flat, Natural, Sharp};
    use crate::notes::notes::Note;

    #[test]
    fn test_get_formula() {
        let chord = Chord {
            root: Note {
                key: Key::C,
                alteration: Natural,
                octave: 0,
            },
            chord_type: ChordType::Maj7,
        };


        assert_eq!(chord.get_formula(), Vec::from([
            Interval { quality: Major, interval_type: Third },
            Interval { quality: Perfect, interval_type: Fifth },
            Interval { quality: Major, interval_type: Seventh }
        ]))
    }

    #[test]
    fn test_get_notes() {
        let mut chord = Chord {
            root: Note {
                key: Key::C,
                alteration: Natural,
                octave: 0,
            },
            chord_type: ChordType::Maj7,
        };

        let mut expected_chord = Vec::from([
            Note { key: C, alteration: Natural, octave: 0 },
            Note { key: E, alteration: Natural, octave: 0 },
            Note { key: G, alteration: Natural, octave: 0 },
            Note { key: B, alteration: Natural, octave: 0 }
        ]);
        assert!(chord.get_notes().iter().all(|item| expected_chord.contains(item)));

        chord = Chord {
            root: Note {
                key: Key::C,
                alteration: Natural,
                octave: 0,
            },
            chord_type: ChordType::Min7,
        };

        expected_chord = Vec::from([
            Note { key: C, alteration: Natural, octave: 0 },
            Note { key: E, alteration: Flat, octave: 0 },
            Note { key: G, alteration: Natural, octave: 0 },
            Note { key: B, alteration: Flat, octave: 0 }
        ]);
        assert!(chord.get_notes().iter().all(|item| expected_chord.contains(item)));

        chord = Chord {
            root: Note {
                key: Key::D,
                alteration: Natural,
                octave: 0,
            },
            chord_type: ChordType::Min7,
        };

        expected_chord = Vec::from([
            Note { key: D, alteration: Natural, octave: 0 },
            Note { key: F, alteration: Natural, octave: 0 },
            Note { key: A, alteration: Natural, octave: 0 },
            Note { key: C, alteration: Natural, octave: 1 }
        ]);
        assert!(chord.get_notes().iter().all(|item| expected_chord.contains(item)));

        chord = Chord {
            root: Note {
                key: Key::B,
                alteration: Flat,
                octave: 1,
            },
            chord_type: ChordType::Min7,
        };

        expected_chord = Vec::from([
            Note { key: B, alteration: Flat, octave: 1 },
            Note { key: D, alteration: Flat, octave: 2 },
            Note { key: F, alteration: Natural, octave: 2 },
            Note { key: A, alteration: Flat, octave: 2 }
        ]);
        assert!(chord.get_notes().iter().all(|item| expected_chord.contains(item)));

        // TODO ce cas ne passe pas
        //     let chord = Chord {
        //         root: Note {
        //             key: Key::F,
        //             alteration: Flat,
        //             octave: 1
        //         },
        //         chord_type: ChordType::Min7
        //     };
        //
        //     let expected_chord = Vec::from([
        //         Note { key: F, alteration: Flat, octave: 1 },
        //         Note { key: A, alteration: DoubleFlat, octave: 1 },
        //         Note { key: C, alteration: Flat, octave: 2 },
        //         Note { key: E, alteration: DoubleFlat, octave: 2 }
        //     ]);
        //     assert_eq!(chord.get_notes(), expected_chord);
        //     assert!(chord.get_notes().iter().all(|item| expected_chord.contains(item)));
    }
}
