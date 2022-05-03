#[cfg(test)]
mod notes_test {
    use crate::notes::notes::{Alteration, get_notes_from_score, Key, Note};

    #[test]
    fn get_score() {
        assert_eq!(Note { key: Key::B, alteration: Alteration::Natural, octave: 0 }.get_score(), 11);
        assert_eq!(Note { key: Key::C, alteration: Alteration::Natural, octave: 1 }.get_score(), 12);
        assert_eq!(Note { key: Key::E, alteration: Alteration::Flat, octave: 4 }.get_score(), 4 * 12 + 4 - 1);
        assert_eq!(Note { key: Key::B, alteration: Alteration::DoubleSharp, octave: 3 }.get_score(), 3 * 12 + 11 + 2)
    }

    #[test]
    fn get_note_from_score_return_right_note() {
        assert_eq!(get_notes_from_score(11).get(&Alteration::Natural).unwrap().key, Key::B);
        assert_eq!(get_notes_from_score(11).get(&Alteration::DoubleSharp).unwrap().key, Key::A);
        assert_eq!(get_notes_from_score(13).get(&Alteration::Natural).is_none(), true);
        assert_eq!(get_notes_from_score(13).get(&Alteration::Flat).unwrap().key, Key::D);
        assert_eq!(get_notes_from_score(29).get(&Alteration::Natural).unwrap().key, Key::F);
        assert_eq!(get_notes_from_score(28).get(&Alteration::Natural).unwrap().key, Key::E);
        assert_eq!(get_notes_from_score(28).get(&Alteration::Flat).unwrap().key, Key::F);
    }
}
