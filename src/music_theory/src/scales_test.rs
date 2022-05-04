#[cfg(test)]
mod scales_test {
    use crate::notes::notes::{Alteration, Key, Note};
    use crate::scales::scales::{Scale, ScaleType};

    #[test]
    fn test_get_notes() {
        let scale = Scale {
            scale_type: ScaleType::Minor,
            note: Note {
                key: Key::A,
                alteration: Alteration::Natural,
                octave: 0,
            },
        };
        let c_major_scale_notes = scale.get_notes();

        println!("{:?}", c_major_scale_notes);
        // assert_eq!(x, HashMap::new())
    }
}
