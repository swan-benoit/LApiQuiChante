pub mod chords {
    use crate::domain::music_theory::intervals::intervals::intervals::Interval;
    use crate::domain::music_theory::intervals::intervals::intervals::IntervalType::{Fifth, Seventh, Third};
    use crate::domain::music_theory::intervals::intervals::intervals::Quality::{Major, Minor, Perfect};
    use crate::domain::music_theory::notes::notes::notes::{Alteration, Note};

    pub struct Chord {
        pub(crate) root: Note,
        pub(crate) chord_type: ChordType,
    }

    impl Chord {
        pub fn new(root: Note, chord_type: ChordType) -> Option<Chord> {
            match root.alteration {
                Alteration::DoubleSharp
                | Alteration::DoubleFlat => None,
                _ => Some(Chord { root, chord_type })
            }
        }

        pub fn get_notes(&self) -> Vec<Note> {
            let accumulator = Vec::from([self.root]);

            self.get_formula().iter()
                .fold(accumulator, |mut accumulator, interval| {
                    let next_note = self.root.to(*interval);

                    // TODO refactor me
                    if next_note.is_some() {
                        accumulator.push(next_note.unwrap());
                    }
                    accumulator
                })
        }

        pub fn get_formula(&self) -> Vec<Interval> {
            let p5 = Interval::new(Fifth, Perfect);
            let maj3 = Interval::new(Third, Major);
            let m3 = Interval::new(Third, Minor);
            let maj7 = Interval::new(Seventh, Major);
            let m7 = Interval::new(Seventh, Minor);

            match self.chord_type {
                ChordType::Min7 => Vec::from([m3, p5, m7]),
                ChordType::Min => Vec::from([m3, p5]),
                ChordType::Maj7 => Vec::from([maj3, p5, maj7]),
                ChordType::Dom7 => Vec::from([maj3, p5, m7]),
                ChordType::Maj => Vec::from([maj3, p5])
            }
        }
    }


    pub enum ChordType {
        Min7,
        Maj7,
        Dom7,
        Maj,
        Min,
    }
}
