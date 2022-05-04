pub mod scales {
    use std::collections::HashMap;

    use crate::intervals::intervals::{Interval, IntervalType, Quality};
    use crate::notes::notes::{Alteration, Key, Note};

    pub struct Scale {
        pub(crate) scale_type: ScaleType,
        pub(crate) note: Note,
    }


    impl Scale {
        pub fn get_notes(&self) -> Vec<Note> {
            let accumulator = Vec::from([self.note]);

            self.get_steps()
                .iter()
                .enumerate()
                .fold(accumulator, |mut vec, (index, interval)| {
                    let current_note = vec.last().unwrap();
                    let key = current_note.key.into_iter().cycle().next().unwrap();
                    let next_note = current_note.to(*interval).get(key);
                    if next_note.is_some() {
                        vec.push(next_note.unwrap());
                    }
                    vec
                })
        }

        pub fn get_steps(&self) -> Vec<Interval> {
            let m2 = Interval {
                quality: Quality::Minor,
                interval_type: IntervalType::Second,
            };
            let M2 = Interval {
                quality: Quality::Major,
                interval_type: IntervalType::Second,
            };
            match self.scale_type {
                ScaleType::Major => Vec::from([M2, M2, m2, M2, M2, M2, m2]),
                ScaleType::MinorHarmonic => Vec::from([M2, m2, M2, M2, m2, M2, M2]),
                ScaleType::MinorMelodic => Vec::from([M2, m2, M2, M2, m2, M2, m2])
            }
        }
    }

    #[derive(Copy, Clone)]
    pub enum ScaleType {
        Major,
        MinorHarmonic,
        MinorMelodic,
    }
}
