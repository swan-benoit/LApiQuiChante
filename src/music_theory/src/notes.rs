pub mod notes {
    use std::collections::HashMap;

    #[derive(Eq, PartialEq, Hash, Debug)]
    pub struct Note {
        pub(crate) key: Key,
        pub(crate) alteration: Alteration,
        pub(crate) octave: i32,

    }

    impl Note {
        pub fn get_score(&self) -> i32 {
            let note_score = match self.key {
                Key::C => 0,
                Key::D => 2,
                Key::E => 4,
                Key::F => 5,
                Key::G => 7,
                Key::A => 9,
                Key::B => 11
            };

            let alteration_score = match self.alteration {
                Alteration::DoubleSharp => 2,
                Alteration::Sharp => 1,
                Alteration::Natural => 0,
                Alteration::Flat => -1,
                Alteration::DoubleFlat => -2
            };

            note_score + alteration_score + self.octave * 12
        }
    }

    pub fn get_notes_from_score(score: i32) -> HashMap<Alteration, Note> {
        let octave = score / 12;
        let note_position = score % 12;

        let natural_note = get_key(note_position).map(|key| Note {
            key,
            alteration: Alteration::Natural,
            octave,
        });

        let sharp_note = get_key(note_position - 1).map(|key| Note {
            key,
            alteration: Alteration::Sharp,
            octave,
        });

        let double_sharp_note = get_key(note_position - 2).map(|key| Note {
            key,
            alteration: Alteration::DoubleSharp,
            octave,
        });

        let flat_note = get_key(note_position + 1).map(|key| Note {
            key,
            alteration: Alteration::Flat,
            octave,
        });

        let double_flat_note = get_key(note_position + 2).map(|key| Note {
            key,
            alteration: Alteration::DoubleFlat,
            octave,
        });

        HashMap::from([
            (Alteration::Natural, natural_note),
            (Alteration::DoubleSharp, double_sharp_note),
            (Alteration::Sharp, sharp_note),
            (Alteration::Flat, flat_note),
            (Alteration::DoubleFlat, double_flat_note),
        ]).into_iter().filter(|(_, note)| note.is_ok())
            .map(|x| (x.0, x.1.unwrap()))
            .collect()
    }

    #[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
    pub enum Alteration {
        DoubleSharp,
        Sharp,
        Natural,
        Flat,
        DoubleFlat,
    }

    #[derive(Eq, PartialEq, Hash, Debug)]
    pub enum Key {
        C,
        D,
        E,
        F,
        G,
        A,
        B,
    }

    fn get_key(posistion: i32) -> Result<Key, &'static str> {
        match posistion {
            0 => Ok(Key::C),
            2 => Ok(Key::D),
            4 => Ok(Key::E),
            5 => Ok(Key::F),
            7 => Ok(Key::G),
            9 => Ok(Key::A),
            11 => Ok(Key::B),
            _ => Err("No notes match the provided position")
        }
    }
}
