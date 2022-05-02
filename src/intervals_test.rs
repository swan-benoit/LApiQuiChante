#[cfg(test)]
mod tests {
    use crate::intervals::intervals::{Interval, IntervalType, Quality};

    #[test]
    fn get_distance() {
        assert_eq!(Interval { interval_type: IntervalType::Second, quality: Quality::Major }.get_distance(), 2);
        assert_eq!(Interval { interval_type: IntervalType::Second, quality: Quality::Minor }.get_distance(), 1);
        assert_eq!(Interval { interval_type: IntervalType::Seventh, quality: Quality::Major }.get_distance(), 11);
        assert_eq!(Interval { interval_type: IntervalType::Fifth, quality: Quality::Perfect }.get_distance(), 7);
        assert_eq!(Interval { interval_type: IntervalType::Sixth, quality: Quality::Augmented }.get_distance(), 10);
        assert_eq!(Interval { interval_type: IntervalType::Seventh, quality: Quality::Minor }.get_distance(), 10);
    }
}
