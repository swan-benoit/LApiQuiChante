pub mod notes {
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

    pub enum Alteration {
        DoubleSharp,
        Sharp,
        Natural,
        Flat,
        DoubleFlat,
    }

    pub enum Key {
        C,
        D,
        E,
        F,
        G,
        A,
        B,
    }
}
