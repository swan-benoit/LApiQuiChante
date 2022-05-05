#[cfg(test)]
mod notes_test {
    use Alteration::{DoubleSharp, Flat};

    use crate::intervals::intervals::{Interval, IntervalType, Quality};
    use crate::keys::keys::Key;
    use crate::keys::keys::Key::{A, B, C, D, E, F, G};
    use crate::notes::notes::{Alteration, get_notes_from_score, Note, PossibleNotes};
    use crate::notes::notes::Alteration::{DoubleFlat, Natural, Sharp};

    #[test]
    fn get_score() {
        assert_eq!(Note { key: Key::B, alteration: Natural, octave: 0 }.get_score(), 11);
        assert_eq!(Note { key: Key::C, alteration: Natural, octave: 1 }.get_score(), 12);
        assert_eq!(Note { key: Key::E, alteration: Flat, octave: 4 }.get_score(), 4 * 12 + 4 - 1);
        assert_eq!(Note { key: Key::B, alteration: DoubleSharp, octave: 3 }.get_score(), 3 * 12 + 11 + 2)
    }

    #[test]
    fn get_note_from_score_return_right_note() {
        assert_eq!(get_notes_from_score(11).get(B).unwrap().key, Key::B);
        assert_eq!(get_notes_from_score(11).get(B).unwrap().alteration, Natural);
        assert_eq!(get_notes_from_score(11).get(A).unwrap().key, A);
        assert_eq!(get_notes_from_score(11).get(A).unwrap().alteration, DoubleSharp);
        assert_eq!(get_notes_from_score(13).get(A).is_none(), true);
        assert_eq!(get_notes_from_score(13).get(D).unwrap().key, D);
        assert_eq!(get_notes_from_score(13).get(D).unwrap().alteration, Flat);
        assert_eq!(get_notes_from_score(29).get(F).unwrap().key, F);
        assert_eq!(get_notes_from_score(29).get(F).unwrap().alteration, Natural);
        assert_eq!(get_notes_from_score(28).get(E).unwrap().key, E);
        assert_eq!(get_notes_from_score(28).get(E).unwrap().alteration, Natural);
        assert_eq!(get_notes_from_score(28).get(F).unwrap().key, F);
        assert_eq!(get_notes_from_score(28).get(F).unwrap().alteration, Flat);
        assert_eq!(get_notes_from_score(28).get(C).is_none(), true);
        assert_eq!(get_notes_from_score(9).get(G).is_some(), true);
    }

    #[test]
    fn test_get_note_for_interval() {
        let lower_note = Note { key: Key::C, alteration: Natural, octave: 1 };
        let upper_note = lower_note.to(Interval { quality: Quality::Minor, interval_type: IntervalType::Sixth });
        assert_eq!(upper_note.unwrap(), Note {
            key: A,
            alteration: Flat,
            octave: 1,
        });

        let upper_note = lower_note.to(Interval { quality: Quality::Augmented, interval_type: IntervalType::Fifth });
        assert_eq!(upper_note.unwrap(), Note {
            key: G,
            alteration: Sharp,
            octave: 1,
        });

        let upper_note = lower_note.to(Interval { quality: Quality::Perfect, interval_type: IntervalType::Fifth });
        assert_eq!(upper_note.unwrap(), Note {
            key: G,
            alteration: Natural,
            octave: 1,
        });


        let lower_note = Note { key: Key::D, alteration: Natural, octave: 1 };
        let upper_note = lower_note.to(Interval { quality: Quality::Minor, interval_type: IntervalType::Third });
        assert_eq!(upper_note.unwrap(), Note {
            key: F,
            alteration: Natural,
            octave: 1,
        });
    }
}
