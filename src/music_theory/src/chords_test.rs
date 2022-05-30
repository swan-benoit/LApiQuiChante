#[cfg(test)]
mod chords_test {
    use crate::chords::chords::{Chord, ChordType};
    use crate::chords::chords::ChordType::{Dom7, Maj, Maj7, Min, Min7};
    use crate::keys::keys::Key;
    use crate::keys::keys::Key::{A, B, C, D, E, F, G};
    use crate::notes::notes::{Alteration, Note};
    use crate::notes::notes::Alteration::{DoubleFlat, DoubleSharp, Flat, Natural, Sharp};

    #[test]
    fn test_get_maj7_notes() {
        let mut expected_chord = Vec::from([
            Note::new(C, Natural, 0),
            Note::new(E, Natural, 0),
            Note::new(G, Natural, 0),
            Note::new(B, Natural, 0)
        ]);
        assert_chord_equal(C, Natural, 0, Maj7, expected_chord);

        expected_chord = Vec::from([
            Note::new(F, Flat, 0),
            Note::new(A, Flat, 0),
            Note::new(C, Flat, 1),
            Note::new(E, Flat, 1)
        ]);
        assert_chord_equal(F, Flat, 0, Maj7, expected_chord);

        expected_chord = Vec::from([
            Note::new(B, Sharp, 0),
            Note::new(D, DoubleSharp, 1),
            Note::new(F, DoubleSharp, 1),
            Note::new(A, DoubleSharp, 1)
        ]);
        assert_chord_equal(B, Sharp, 0, Maj7, expected_chord);
    }


    #[test]
    fn test_double_alteration_chord_are_not_allowed() {
        let chord = Chord::new(
            Note::new(Key::D, DoubleSharp, 0),
            ChordType::Maj7,
        );

        assert!(chord.is_none());

        let chord = Chord::new(
            Note::new(Key::A, DoubleFlat, 0),
            ChordType::Maj7,
        );

        assert!(chord.is_none());
    }

    #[test]
    fn test_get_min7_notes() {
        let mut expected_chord = Vec::from([
            Note::new(B, Flat, 1),
            Note::new(D, Flat, 2),
            Note::new(F, Natural, 2),
            Note::new(A, Flat, 2)
        ]);
        assert_chord_equal(Key::B, Flat, 1, Min7, expected_chord);

        expected_chord = Vec::from([
            Note { key: G, alteration: Natural, octave: 1 },
            Note { key: B, alteration: Flat, octave: 1 },
            Note { key: D, alteration: Natural, octave: 2 },
            Note { key: F, alteration: Natural, octave: 2 }
        ]);
        assert_chord_equal(Key::G, Natural, 1, Min7, expected_chord);

        expected_chord = Vec::from([
            Note { key: E, alteration: Sharp, octave: 1 },
            Note { key: G, alteration: Sharp, octave: 1 },
            Note { key: B, alteration: Sharp, octave: 1 },
            Note { key: D, alteration: Sharp, octave: 2 }
        ]);
        assert_chord_equal(Key::E, Sharp, 1, Min, expected_chord);
    }

    #[test]
    fn test_get_min_notes() {
        let mut expected_chord = Vec::from([
            Note::new(A, Flat, 1),
            Note::new(C, Flat, 2),
            Note::new(E, Flat, 2),
        ]);
        assert_chord_equal(Key::A, Flat, 1, Min, expected_chord);

        expected_chord = Vec::from([
            Note { key: C, alteration: Natural, octave: 1 },
            Note { key: E, alteration: Flat, octave: 1 },
            Note { key: G, alteration: Natural, octave: 1 },
        ]);
        assert_chord_equal(Key::C, Natural, 1, Min, expected_chord);

        expected_chord = Vec::from([
            Note { key: E, alteration: Sharp, octave: 1 },
            Note { key: G, alteration: Sharp, octave: 1 },
            Note { key: B, alteration: Sharp, octave: 1 },
        ]);
        assert_chord_equal(Key::E, Sharp, 1, Min, expected_chord);
    }

    #[test]
    fn test_get_maj_notes() {
        let mut expected_chord = Vec::from([
            Note::new(E, Flat, 1),
            Note::new(G, Natural, 1),
            Note::new(B, Flat, 1),
        ]);
        assert_chord_equal(Key::E, Flat, 1, Maj, expected_chord);

        expected_chord = Vec::from([
            Note { key: G, alteration: Natural, octave: 1 },
            Note { key: B, alteration: Natural, octave: 1 },
            Note { key: D, alteration: Natural, octave: 2 },
        ]);
        assert_chord_equal(Key::G, Natural, 1, Maj, expected_chord);

        expected_chord = Vec::from([
            Note { key: C, alteration: Sharp, octave: 1 },
            Note { key: E, alteration: Sharp, octave: 1 },
            Note { key: G, alteration: Sharp, octave: 1 },
        ]);
        assert_chord_equal(Key::C, Sharp, 1, Maj, expected_chord);
    }

    #[test]
    fn test_get_dom7_notes() {
        let mut expected_chord = Vec::from([
            Note::new(E, Flat, 1),
            Note::new(G, Natural, 1),
            Note::new(B, Flat, 1),
            Note::new(D, Flat, 2),
        ]);
        assert_chord_equal(Key::E, Flat, 1, Dom7, expected_chord);

        expected_chord = Vec::from([
            Note { key: G, alteration: Natural, octave: 1 },
            Note { key: B, alteration: Natural, octave: 1 },
            Note { key: D, alteration: Natural, octave: 2 },
            Note::new(F, Natural, 2),
        ]);
        assert_chord_equal(Key::G, Natural, 1, Dom7, expected_chord);

        expected_chord = Vec::from([
            Note { key: C, alteration: Sharp, octave: 1 },
            Note { key: E, alteration: Sharp, octave: 1 },
            Note { key: G, alteration: Sharp, octave: 1 },
            Note::new(B, Natural, 1),
        ]);
        assert_chord_equal(Key::C, Sharp, 1, Dom7, expected_chord);
    }

    fn assert_chord_equal(key: Key, alteration: Alteration, octave: i32, chord_type: ChordType, expected_chord: Vec<Note>) {
        let chord: Chord;

        chord = Chord::new(
            Note::new(key, alteration, octave),
            chord_type,
        ).unwrap();
        assert!(chord.get_notes().iter().all(|item| expected_chord.contains(item)));
    }
}
