pub mod notes {
    use std::cmp::Ordering;
    use std::collections::HashSet;
    use std::hash::Hash;

    use crate::intervals::intervals::Interval;
    use crate::keys::keys::{get_key, Key};

    #[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
    pub struct Note {
        pub(crate) key: Key,
        pub(crate) alteration: Alteration,
        pub(crate) octave: i32,

    }

    impl Note {
        pub(crate) fn to(&self, interval: Interval) -> PossibleNotes {
            get_notes_from_score(self.get_score() + interval.get_distance())
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

        let all_notes = HashSet::from([
            natural_note,
            double_sharp_note,
            sharp_note,
            flat_note,
            double_flat_note,
        ]);

        PossibleNotes {
            notes: all_notes
                .iter()
                .filter(|note| note.is_ok())
                .map(|note| (note.unwrap()))
                .collect()
        }
    }

    #[derive(Eq, PartialEq, Hash, Debug)]
    pub struct PossibleNotes {
        notes: Vec<Note>,
    }

    impl PossibleNotes {
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
