#[cfg(test)]
mod intervals_tests {
    use crate::intervals::intervals::{Interval, IntervalType, Quality};

    #[test]
    fn get_distance() {
        assert_eq!(Interval::new(IntervalType::Second, Quality::Major).get_score(), 2);
        assert_eq!(Interval::new(IntervalType::Second, Quality::Minor).get_score(), 1);
        assert_eq!(Interval::new(IntervalType::Seventh, Quality::Major).get_score(), 11);
        assert_eq!(Interval::new(IntervalType::Fifth, Quality::Perfect).get_score(), 7);
        assert_eq!(Interval::new(IntervalType::Sixth, Quality::Augmented).get_score(), 10);
        assert_eq!(Interval::new(IntervalType::Seventh, Quality::Minor).get_score(), 10);
        assert_eq!(Interval::new(IntervalType::Seventh, Quality::Minor).get_score(), 10);
        assert_eq!(Interval::new(IntervalType::Fourth, Quality::Diminished).get_score(), 3);
        assert_eq!(Interval::new(IntervalType::Unisson, Quality::Perfect).get_score(), 0);
    }
}
