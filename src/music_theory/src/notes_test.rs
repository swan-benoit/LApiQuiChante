#[cfg(test)]
mod notes_test {
    use std::collections::hash_map::Keys;

    use crate::notes::notes::{Alteration, Key, Note};

    #[test]
    fn get_score() {
        assert_eq!(Note {
            key: Key::B,
            alteration: Alteration::Natural,
            octave: 0,
        }.get_score(), 11);
        assert_eq!(Note {
            key: Key::C,
            alteration: Alteration::Natural,
            octave: 1,
        }.get_score(), 12);
        assert_eq!(Note {
            key: Key::E,
            alteration: Alteration::Flat,
            octave: 4,
        }.get_score(), 4 * 12 + 4 - 1);
        assert_eq!(Note {
            key: Key::B,
            alteration: Alteration::DoubleSharp,
            octave: 3,
        }.get_score(), 3 * 12 + 11 + 2)
    }
}
