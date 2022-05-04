#[cfg(test)]
mod scales_test {
    use Key::C;

    use crate::notes::notes::{Alteration, Key, Note};
    use crate::notes::notes::Alteration::{DoubleFlat, Flat, Natural, Sharp};
    use crate::notes::notes::Key::{A, B, E, F, G};
    use crate::scales::scales::{Scale, ScaleType};

    #[test]
    fn test_get_notes_for_major_scale() {
        let scale = Scale {
            scale_type: ScaleType::Major,
            note: Note {
                key: Key::G,
                alteration: Alteration::Natural,
                octave: 0,
            },
        };
        let c_major_scale_notes = scale.get_notes();
        assert_eq!(c_major_scale_notes[0].key, G);
        assert_eq!(c_major_scale_notes[0].alteration, Natural);
        assert_eq!(c_major_scale_notes[1].key, A);
        assert_eq!(c_major_scale_notes[1].alteration, Natural);
        assert_eq!(c_major_scale_notes[6].key, F);
        assert_eq!(c_major_scale_notes[6].alteration, Sharp);
    }
}
