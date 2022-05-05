pub mod scales {
    use crate::intervals::intervals::{Interval, IntervalType, Quality};
    use crate::intervals::intervals::IntervalType::Second;
    use crate::intervals::intervals::Quality::{Major, Minor};
    use crate::notes::notes::Note;

    pub struct Scale {
        pub(crate) scale_type: ScaleType,
        pub(crate) note: Note,
    }


    impl Scale {
        pub fn get_notes(&self) -> Vec<Note> {
            let accumulator = Vec::from([self.note]);

            self.get_steps()
                .iter()
                .fold(accumulator, |mut vec, interval| {
                    let current_note = vec.last().unwrap();
                    let next_note = current_note.to(*interval);
                    if next_note.is_some() {
                        vec.push(next_note.unwrap());
                    }
                    vec
                })
        }

        pub fn get_steps(&self) -> Vec<Interval> {
            let m2 = Interval::new(Second, Minor);
            let maj2 = Interval::new(Second, Major);
            match self.scale_type {
                ScaleType::Major => Vec::from([maj2, maj2, m2, maj2, maj2, maj2, m2]),
                ScaleType::MinorHarmonic => Vec::from([maj2, m2, maj2, maj2, m2, maj2, maj2]),
                ScaleType::MinorMelodic => Vec::from([maj2, m2, maj2, maj2, m2, maj2, m2])
            }
        }
    }

    pub enum ScaleType {
        Major,
        MinorHarmonic,
        MinorMelodic,
    }
}
