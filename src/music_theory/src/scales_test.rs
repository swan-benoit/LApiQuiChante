#[cfg(test)]
mod scales_test {
    use crate::keys::keys::Key;
    use crate::keys::keys::Key::{A, B, C, D, E, F, G};
    use crate::notes::notes::{Alteration, Note};
    use crate::notes::notes::Alteration::{DoubleFlat, Flat, Natural, Sharp};
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
        assert_eq!(scale.get_notes(), Vec::from([
            Note { key: G, alteration: Natural, octave: 0 },
            Note { key: A, alteration: Natural, octave: 0 },
            Note { key: B, alteration: Natural, octave: 0 },
            Note { key: C, alteration: Natural, octave: 1 },
            Note { key: D, alteration: Natural, octave: 1 },
            Note { key: E, alteration: Natural, octave: 1 },
            Note { key: F, alteration: Sharp, octave: 1 },
            Note { key: G, alteration: Natural, octave: 1 }
        ]));
    }
}
