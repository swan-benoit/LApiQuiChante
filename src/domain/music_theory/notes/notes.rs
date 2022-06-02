pub mod notes {
    use std::collections::HashSet;
    use std::hash::Hash;

    use crate::domain::music_theory::intervals::intervals::intervals::Interval;
    use crate::domain::music_theory::keys::keys::keys::{get_key, Key};

    #[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
    pub struct Note {
        pub(crate) key: Key,
        pub(crate) alteration: Alteration,
        pub(crate) octave: i32,

    }

    impl Note {
        pub(crate) fn to(&self, interval: Interval) -> Option<Note> {
            let i = interval.get_degree_value();
            let target = self.key.into_iter().nth(i).unwrap();
            let score = self.get_score() + interval.get_score();
            get_notes_from_score(score)
                .get(target)
        }

        pub(crate) fn new(key: Key, alteration: Alteration, octave: i32) -> Note {
            Note { key, alteration, octave }
        }
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

    pub fn get_notes_from_score(score: i32) -> PossibleNotes {
        let natural_note = get_note(score, Alteration::Natural);
        let sharp_note = get_note(score - 1, Alteration::Sharp);
        let double_sharp_note = get_note(score - 2, Alteration::DoubleSharp);
        let flat_note = get_note(score + 1, Alteration::Flat);
        let double_flat_note = get_note(score + 2, Alteration::DoubleFlat);

        let all_notes = HashSet::from([
            natural_note,
            double_sharp_note,
            sharp_note,
            flat_note,
            double_flat_note,
        ]);

        PossibleNotes::new(all_notes
            .iter()
            .filter(|note| note.is_ok())
            .map(|note| (note.unwrap()))
            .collect()
        )
    }

    fn get_note(score: i32, alteration: Alteration) -> Result<Note, &'static str> {
        let octave = score / 12;
        let note_position = score % 12;

        get_key(note_position)
            .map(|key| Note::new(key, alteration, octave))
    }


    #[derive(Eq, PartialEq, Hash, Debug)]
    pub struct PossibleNotes {
        pub(crate) notes: Vec<Note>,
    }

    impl PossibleNotes {
        pub fn new(notes: Vec<Note>) -> PossibleNotes {
            PossibleNotes { notes }
        }

        pub fn get(&self, target: Key) -> Option<Note> {
            self.notes
                .iter()
                .filter(|note| note.key == target)
                .map(|note| note.clone())
                .last()
        }
    }


    #[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
    pub enum Alteration {
        DoubleSharp,
        Sharp,
        Natural,
        Flat,
        DoubleFlat,
    }

}
