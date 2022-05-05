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
    }
}
