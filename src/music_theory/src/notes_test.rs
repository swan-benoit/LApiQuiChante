#[cfg(test)]
mod notes_test {
    use Alteration::{DoubleSharp, Flat};

    use crate::chords::chords::ChordType::Min;
    use crate::intervals::intervals::{Interval, IntervalType, Quality};
    use crate::intervals::intervals::IntervalType::{Fifth, Sixth};
    use crate::intervals::intervals::Quality::{Augmented, Minor};
    use crate::keys::keys::Key;
    use crate::keys::keys::Key::{A, B, C, D, E, F, G};
    use crate::notes::notes::{Alteration, get_notes_from_score, Note, PossibleNotes};
    use crate::notes::notes::Alteration::{DoubleFlat, Natural, Sharp};

    #[test]
    fn get_score() {
        assert_eq!(Note::new(Key::B, Natural, 0).get_score(), 11);
        assert_eq!(Note::new(Key::C, Natural, 1).get_score(), 12);
        assert_eq!(Note::new(Key::E, Flat, 4).get_score(), 4 * 12 + 4 - 1);
        assert_eq!(Note::new(Key::B, DoubleSharp, 3).get_score(), 3 * 12 + 11 + 2)
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
        let lower_note = Note::new(C, Natural, 1);
        let upper_note = lower_note.to(Interval::new(Sixth, Minor));
        assert_eq!(upper_note.unwrap(), Note::new(A, Flat, 1));

        let upper_note = lower_note.to(Interval::new(Fifth, Augmented));
        assert_eq!(upper_note.unwrap(), Note::new(G, Sharp, 1));

        // let upper_note = lower_note.to(Interval::new(Quality::Perfect, interval_type: IntervalType::Fifth });
        // assert_eq!(upper_note.unwrap(), Note::new(G, Natural, 1));


        let lower_note = Note::new(D, Natural, 1);
        // let upper_note = lower_note.to(Interval::new(Quality::Minor, interval_type: IntervalType::Third });
        // assert_eq!(upper_note.unwrap(), Note::new(F, Natural, 1));
    }
}
