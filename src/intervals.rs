pub mod intervals {
    pub struct Interval {
        pub(crate) quality: Quality,
        pub(crate) interval_type: IntervalType,
    }

    pub enum IntervalType {
        Unisson,
        Second,
        Third,
        Fourth,
        Fifth,
        Sixth,
        Seventh,
    }

    pub enum Quality {
        Minor,
        Major,
        Diminished,
        Augmented,
        Perfect,
    }

    impl Interval {
        pub fn get_distance(&self) -> i32 {
            let interval = match self.interval_type {
                IntervalType::Unisson => 0,
                IntervalType::Second => 2,
                IntervalType::Third => 4,
                IntervalType::Fourth => 5,
                IntervalType::Fifth => 7,
                IntervalType::Sixth => 9,
                IntervalType::Seventh => 11
            };

            match self.quality {
                Quality::Minor => interval - 1,
                Quality::Major | Quality::Perfect => interval,
                Quality::Diminished => interval - 2,
                Quality::Augmented => interval + 1
            }
        }
    }
}
