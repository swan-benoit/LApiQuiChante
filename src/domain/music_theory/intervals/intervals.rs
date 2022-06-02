pub mod intervals {
    #[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
    pub struct Interval {
        pub(crate) quality: Quality,
        pub(crate) interval_type: IntervalType,
    }

    #[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
    pub enum IntervalType {
        Unisson,
        Second,
        Third,
        Fourth,
        Fifth,
        Sixth,
        Seventh,
    }

    #[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
    pub enum Quality {
        Minor,
        Major,
        Diminished,
        Augmented,
        Perfect,
    }

    impl Interval {
        pub fn new(interval_type: IntervalType, quality: Quality) -> Interval {
            Interval { quality, interval_type }
        }

        pub fn get_score(&self) -> i32 {
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

        pub fn get_degree_value(&self) -> usize {
            match self.interval_type {
                IntervalType::Unisson => 0,
                IntervalType::Second => 1,
                IntervalType::Third => 2,
                IntervalType::Fourth => 3,
                IntervalType::Fifth => 4,
                IntervalType::Sixth => 5,
                IntervalType::Seventh => 6
            }
        }
    }
}
