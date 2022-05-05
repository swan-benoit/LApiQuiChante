#[cfg(test)]
mod intervals_tests {
    use crate::intervals::intervals::{Interval, IntervalType, Quality};

    #[test]
    fn get_distance() {
        assert_eq!(Interval { interval_type: IntervalType::Second, quality: Quality::Major }.get_score(), 2);
        assert_eq!(Interval { interval_type: IntervalType::Second, quality: Quality::Minor }.get_score(), 1);
        assert_eq!(Interval { interval_type: IntervalType::Seventh, quality: Quality::Major }.get_score(), 11);
        assert_eq!(Interval { interval_type: IntervalType::Fifth, quality: Quality::Perfect }.get_score(), 7);
        assert_eq!(Interval { interval_type: IntervalType::Sixth, quality: Quality::Augmented }.get_score(), 10);
        assert_eq!(Interval { interval_type: IntervalType::Seventh, quality: Quality::Minor }.get_score(), 10);
        assert_eq!(Interval { interval_type: IntervalType::Seventh, quality: Quality::Minor }.get_score(), 10);
    }
}
