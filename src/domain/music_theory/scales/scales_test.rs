#[cfg(test)]
mod scales_test {
    use crate::domain::music_theory::keys::keys::keys::Key::{A, B, C, D, E, F, G};
    use crate::domain::music_theory::notes::notes::notes::Alteration::{DoubleFlat, DoubleSharp, Flat, Natural, Sharp};
    use crate::domain::music_theory::notes::notes::notes::Note;
    use crate::domain::music_theory::scales::scales::scales::{Scale, ScaleType};

    #[test]
    fn test_get_notes_for_major_scale() {
        let scale = Scale::new(
            ScaleType::Major,
            Note::new(G, Natural, 0),
        );
        assert_eq!(scale.get_notes(), Vec::from([
            Note::new(G, Natural, 0),
            Note::new(A, Natural, 0),
            Note::new(B, Natural, 0),
            Note::new(C, Natural, 1),
            Note::new(D, Natural, 1),
            Note::new(E, Natural, 1),
            Note::new(F, Sharp, 1),
            Note::new(G, Natural, 1)
        ]));

        let scale = Scale::new(
            ScaleType::Major,
            Note::new(F, Sharp, 0),
        );
        assert_eq!(scale.get_notes(), Vec::from([
            Note::new(F, Sharp, 0),
            Note::new(G, Sharp, 0),
            Note::new(A, Sharp, 0),
            Note::new(B, Natural, 0),
            Note::new(C, Sharp, 1),
            Note::new(D, Sharp, 1),
            Note::new(E, Sharp, 1),
            Note::new(F, Sharp, 1),
        ]));


        let scale = Scale::new(
            ScaleType::Major,
            Note::new(D, Flat, 0),
        );
        assert_eq!(scale.get_notes(), Vec::from([
            Note::new(D, Flat, 0),
            Note::new(E, Flat, 0),
            Note::new(F, Natural, 0),
            Note::new(G, Flat, 0),
            Note::new(A, Flat, 0),
            Note::new(B, Flat, 0),
            Note::new(C, Natural, 1),
            Note::new(D, Flat, 1),
        ]));
    }

    #[test]
    fn test_get_notes_for_minor_harmonic_scale() {
        let mut scale = Scale::new(
            ScaleType::HarmonicMinor,
            Note::new(G, Natural, 0),
        );
        assert_eq!(scale.get_notes(), Vec::from([
            Note::new(G, Natural, 0),
            Note::new(A, Natural, 0),
            Note::new(B, Flat, 0),
            Note::new(C, Natural, 1),
            Note::new(D, Natural, 1),
            Note::new(E, Flat, 1),
            Note::new(F, Sharp, 1),
            Note::new(G, Natural, 1)
        ]));

        scale = Scale::new(
            ScaleType::HarmonicMinor,
            Note::new(D, Sharp, 0),
        );
        assert_eq!(scale.get_notes(), Vec::from([
            Note::new(D, Sharp, 0),
            Note::new(E, Sharp, 0),
            Note::new(F, Sharp, 0),
            Note::new(G, Sharp, 0),
            Note::new(A, Sharp, 0),
            Note::new(B, Natural, 0),
            Note::new(C, DoubleSharp, 1),
            Note::new(D, Sharp, 1),
        ]));


        scale = Scale::new(
            ScaleType::HarmonicMinor,
            Note::new(B, Flat, 0),
        );
        assert_eq!(scale.get_notes(), Vec::from([
            Note::new(B, Flat, 0),
            Note::new(C, Natural, 1),
            Note::new(D, Flat, 1),
            Note::new(E, Flat, 1),
            Note::new(F, Natural, 1),
            Note::new(G, Flat, 1),
            Note::new(A, Natural, 1),
            Note::new(B, Flat, 1),
        ]));
    }

    #[test]
    fn test_get_notes_for_natural_minor_scale() {
        let mut scale = Scale::new(
            ScaleType::NaturalMinor,
            Note::new(A, Natural, 0),
        );
        assert_eq!(scale.get_notes(), Vec::from([
            Note::new(A, Natural, 0),
            Note::new(B, Natural, 0),
            Note::new(C, Natural, 1),
            Note::new(D, Natural, 1),
            Note::new(E, Natural, 1),
            Note::new(F, Natural, 1),
            Note::new(G, Natural, 1),
            Note::new(A, Natural, 1),
        ]));

        scale = Scale::new(
            ScaleType::NaturalMinor,
            Note::new(F, Sharp, 0),
        );
        assert_eq!(scale.get_notes(), Vec::from([
            Note::new(F, Sharp, 0),
            Note::new(G, Sharp, 0),
            Note::new(A, Natural, 0),
            Note::new(B, Natural, 0),
            Note::new(C, Sharp, 1),
            Note::new(D, Natural, 1),
            Note::new(E, Natural, 1),
            Note::new(F, Sharp, 1),
        ]));

        scale = Scale::new(
            ScaleType::NaturalMinor,
            Note::new(D, Flat, 1),
        );
        assert_eq!(scale.get_notes(), Vec::from([
            Note::new(D, Flat, 1),
            Note::new(E, Flat, 1),
            Note::new(F, Flat, 1),
            Note::new(G, Flat, 1),
            Note::new(A, Flat, 1),
            Note::new(B, DoubleFlat, 1),
            Note::new(C, Flat, 2),
            Note::new(D, Flat, 2),
        ]));
    }

    #[test]
    fn test_get_notes_for_pentatonic_minor_scale() {
        let mut scale = Scale::new(
            ScaleType::MinorPentatonic,
            Note::new(C, Natural, 0),
        );
        assert_eq!(scale.get_notes(), Vec::from([
            Note::new(C, Natural, 0),
            Note::new(E, Flat, 0),
            Note::new(F, Natural, 0),
            Note::new(G, Natural, 0),
            Note::new(B, Flat, 0),
            Note::new(C, Natural, 1),
        ]));

        scale = Scale::new(
            ScaleType::MinorPentatonic,
            Note::new(G, Sharp, 0),
        );
        assert_eq!(scale.get_notes(), Vec::from([
            Note::new(G, Sharp, 0),
            Note::new(B, Natural, 0),
            Note::new(C, Sharp, 1),
            Note::new(D, Sharp, 1),
            Note::new(F, Sharp, 1),
            Note::new(G, Sharp, 1),
        ]));

        scale = Scale::new(
            ScaleType::MinorPentatonic,
            Note::new(E, Flat, 0),
        );
        assert_eq!(scale.get_notes(), Vec::from([
            Note::new(E, Flat, 0),
            Note::new(G, Flat, 0),
            Note::new(A, Flat, 0),
            Note::new(B, Flat, 0),
            Note::new(D, Flat, 1),
            Note::new(E, Flat, 1),
        ]));
    }
}
