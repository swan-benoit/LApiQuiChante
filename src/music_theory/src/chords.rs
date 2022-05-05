pub mod chords {
    use std::collections::hash_map;

    use crate::intervals::intervals::Interval;
    use crate::intervals::intervals::IntervalType::{Fifth, Seventh, Third};
    use crate::intervals::intervals::Quality::{Major, Minor, Perfect};
    use crate::notes::notes::Note;

    pub struct Chord {
        pub(crate) root: Note,
        pub(crate) chord_type: ChordType,
    }

    impl Chord {
        pub fn get_notes(&self) -> Vec<Note> {
            let accumulator = Vec::from([self.root]);
            self.get_formula().iter()
                .fold(accumulator, |mut accumulator, interval| {
                    let next_note = self.root.to(*interval);
                    if next_note.is_some() {
                        accumulator.push(next_note.unwrap());
                    }
                    accumulator
                })
        }

        pub fn get_formula(&self) -> Vec<Interval> {
            let p5 = Interval { quality: Perfect, interval_type: Fifth };
            let maj3 = Interval { quality: Major, interval_type: Third };
            let m3 = Interval { quality: Minor, interval_type: Third };
            let maj7 = Interval { quality: Major, interval_type: Seventh };
            let m7 = Interval { quality: Minor, interval_type: Seventh };

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
