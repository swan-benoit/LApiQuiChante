#[cfg(test)]
mod notes_test {
    use crate::keys::keys::Key;
    use crate::keys::keys::Key::{A, B, C, D, E, F, G};
    use crate::notes::notes::{get_notes_from_score, Note};
    use crate::notes::notes::Alteration::{DoubleSharp, Flat, Natural};

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

}
