#[cfg(test)]
mod scales_test {
    use crate::keys::keys::Key;
    use crate::keys::keys::Key::{A, B, C, D, E, F, G};
    use crate::notes::notes::{Alteration, Note};
    use crate::notes::notes::Alteration::{DoubleFlat, Flat, Natural, Sharp};
    use crate::scales::scales::{Scale, ScaleType};

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
}
