pub mod scales {
    use crate::domain::music_theory::intervals::intervals::intervals::Interval;
    use crate::domain::music_theory::intervals::intervals::intervals::IntervalType::{Second, Third};
    use crate::domain::music_theory::intervals::intervals::intervals::Quality::{Augmented, Major, Minor};
    use crate::domain::music_theory::notes::notes::notes::Note;

    #[derive()]
    pub struct Scale {
        pub(crate) scale_type: ScaleType,
        pub(crate) note: Note,
    }


    impl Scale {
        pub fn new(scale_type: ScaleType, note: Note) -> Scale {
            Scale { scale_type, note }
        }

        pub fn get_notes(&self) -> Vec<Note> {
            let accumulator = Vec::from([self.note]);

            self.get_steps()
                .iter()
                .fold(accumulator, |mut vec, interval| {
                    let current_note = vec.last().unwrap();
                    let next_note = current_note.to(*interval);
                    // TODO refactor me
                    if next_note.is_some() {
                        vec.push(next_note.unwrap());
                    }
                    vec
                })
        }

        pub fn get_steps(&self) -> Vec<Interval> {
            let m2 = Interval::new(Second, Minor);
            let maj2 = Interval::new(Second, Major);
            let aug2 = Interval::new(Second, Augmented);
            let m3 = Interval::new(Third, Minor);

            match self.scale_type {
                ScaleType::Major => Vec::from([maj2, maj2, m2, maj2, maj2, maj2, m2]),
                ScaleType::HarmonicMinor => Vec::from([maj2, m2, maj2, maj2, m2, aug2, m2]),
                ScaleType::NaturalMinor => Vec::from([maj2, m2, maj2, maj2, m2, maj2, maj2]),
                ScaleType::MinorPentatonic => Vec::from([m3, maj2, maj2, m3, maj2])
            }
        }
    }

    pub enum ScaleType {
        Major,
        HarmonicMinor,
        NaturalMinor,
        MinorPentatonic
    }
}
