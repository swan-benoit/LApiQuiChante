#[cfg(test)]
mod chords_test {
    use crate::chords::chords::{Chord, ChordType};
    use crate::chords::chords::ChordType::Maj;
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
            root: Note::new(C, Natural, 0),
            chord_type: ChordType::Maj7,
        };


        assert_eq!(chord.get_formula(), Vec::from([
            Interval::new(Third, Major),
            Interval::new(Fifth, Perfect),
            Interval::new(Seventh, Major)
        ]))
    }

    #[test]
    fn test_get_notes() {
        let mut chord = Chord {
            root: Note::new(C, Natural, 0),
            chord_type: ChordType::Maj7,
        };

        let mut expected_chord = Vec::from([
            Note::new(C, Natural, 0),
            Note::new(E, Natural, 0),
            Note::new(G, Natural, 0),
            Note::new(B, Natural, 0)
        ]);
        assert!(chord.get_notes().iter().all(|item| expected_chord.contains(item)));

        chord = Chord {
            root: Note::new(C, Natural, 0),
            chord_type: ChordType::Min7,
        };

        expected_chord = Vec::from([
            Note::new(C, Natural, 0),
            Note::new(E, Flat, 0),
            Note::new(G, Natural, 0),
            Note::new(B, Flat, 0)
        ]);
        assert!(chord.get_notes().iter().all(|item| expected_chord.contains(item)));

        chord = Chord {
            root: Note::new(Key::D, Natural, 0),
            chord_type: ChordType::Min7,
        };

        expected_chord = Vec::from([
            Note::new(D, Natural, 0),
            Note::new(F, Natural, 0),
            Note::new(A, Natural, 0),
            Note::new(C, Natural, 1)
        ]);
        assert!(chord.get_notes().iter().all(|item| expected_chord.contains(item)));

        chord = Chord {
            root: Note::new(Key::B, Flat, 1),
            chord_type: ChordType::Min7,
        };

        expected_chord = Vec::from([
            Note::new(B, Flat, 1),
            Note::new(D, Flat, 2),
            Note::new(F, Natural, 2),
            Note::new(A, Flat, 2)
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
