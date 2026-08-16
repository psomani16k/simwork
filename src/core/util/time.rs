use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::core::util::duration::Duration;

/// Carries a pico second value
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct SimTime(u128);

impl SimTime {
    pub const EPOCH: SimTime = SimTime(0);

    pub fn from_ps(ps: u128) -> Self {
        Self(ps)
    }

    pub fn from_ns(ns: u64) -> Self {
        Self::from_ps(ns as u128 * 1_000)
    }

    pub fn from_us(us: u64) -> Self {
        Self::from_ps(us as u128 * 1_000_000)
    }

    pub fn from_ms(ms: u64) -> Self {
        Self::from_ps(ms as u128 * 1_000_000_000)
    }

    pub fn from_sec(sec: u32) -> Self {
        Self::from_ps(sec as u128 * 1_000_000_000_000)
    }

    pub fn from_min(min: u32) -> Self {
        Self::from_ps(min as u128 * 60_000_000_000_000)
    }

    pub fn from_hour(hour: u32) -> Self {
        Self::from_ps(hour as u128 * 3_600_000_000_000_000)
    }

    /// Exact pico second count since [`SimTime::EPOCH`].
    pub fn as_ps(&self) -> u128 {
        self.0
    }

    /// Whole nano seconds since [`SimTime::EPOCH`], truncating any remainder.
    pub fn as_ns(&self) -> u128 {
        self.0 / 1_000
    }

    /// Whole micro seconds since [`SimTime::EPOCH`], truncating any remainder.
    pub fn as_us(&self) -> u128 {
        self.0 / 1_000_000
    }

    /// Whole milli seconds since [`SimTime::EPOCH`], truncating any remainder.
    pub fn as_ms(&self) -> u128 {
        self.0 / 1_000_000_000
    }

    /// Whole seconds since [`SimTime::EPOCH`], truncating any remainder.
    pub fn as_sec(&self) -> u128 {
        self.0 / 1_000_000_000_000
    }

    /// Time elapsed since [`SimTime::EPOCH`].
    pub fn since_epoch(&self) -> Duration {
        Duration::ps(self.0)
    }

    /// Time elapsed since `earlier`, or `None` if `earlier` is later than `self`.
    pub fn checked_duration_since(&self, earlier: SimTime) -> Option<Duration> {
        self.0.checked_sub(earlier.0).map(Duration::ps)
    }

    /// Time elapsed since `earlier`, clamped to [`Duration::ZERO`] if `earlier` is later than `self`.
    pub fn saturating_duration_since(&self, earlier: SimTime) -> Duration {
        Duration::ps(self.0.saturating_sub(earlier.0))
    }

    pub fn checked_add(self, duration: Duration) -> Option<SimTime> {
        self.0.checked_add(duration.as_ps()).map(SimTime)
    }

    pub fn checked_sub(self, duration: Duration) -> Option<SimTime> {
        self.0.checked_sub(duration.as_ps()).map(SimTime)
    }

    pub fn saturating_add(self, duration: Duration) -> SimTime {
        SimTime(self.0.saturating_add(duration.as_ps()))
    }

    /// Clamps at [`SimTime::EPOCH`] instead of underflowing.
    pub fn saturating_sub(self, duration: Duration) -> SimTime {
        SimTime(self.0.saturating_sub(duration.as_ps()))
    }

    pub fn is_after(&self, ts: Self) -> bool {
        *self > ts
    }

    pub fn is_after_or_at(&self, ts: Self) -> bool {
        *self >= ts
    }
    
    pub fn is_before(&self, ts: Self) -> bool {
        *self < ts
    }

    pub fn is_before_or_at(&self, ts: Self) -> bool {
        *self <= ts
    }

}

impl Add<Duration> for SimTime {
    type Output = SimTime;

    fn add(self, duration: Duration) -> SimTime {
        SimTime(self.0 + duration.as_ps())
    }
}

impl AddAssign<Duration> for SimTime {
    fn add_assign(&mut self, duration: Duration) {
        self.0 += duration.as_ps();
    }
}

/// Steps back in time by `duration`.
impl Sub<Duration> for SimTime {
    type Output = SimTime;

    fn sub(self, duration: Duration) -> SimTime {
        SimTime(self.0 - duration.as_ps())
    }
}

impl SubAssign<Duration> for SimTime {
    fn sub_assign(&mut self, duration: Duration) {
        self.0 -= duration.as_ps();
    }
}

/// Elapsed time between two points, panicking on underflow. Prefer
/// [`SimTime::checked_duration_since`] or [`SimTime::saturating_duration_since`]
/// when `other` might not precede `self`.
impl Sub<SimTime> for SimTime {
    type Output = Duration;

    fn sub(self, other: SimTime) -> Duration {
        Duration::ps(self.0 - other.0)
    }
}

impl fmt::Display for SimTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} since epoch", self.since_epoch())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_unit_is_a_thousandfold_of_the_next() {
        assert_eq!(SimTime::from_ns(1), SimTime::from_ps(1_000));
        assert_eq!(SimTime::from_us(1), SimTime::from_ns(1_000));
        assert_eq!(SimTime::from_ms(1), SimTime::from_us(1_000));
        assert_eq!(SimTime::from_sec(1), SimTime::from_ms(1_000));
        assert_eq!(SimTime::from_min(1), SimTime::from_sec(60));
        assert_eq!(SimTime::from_hour(1), SimTime::from_min(60));
        assert_eq!(SimTime::EPOCH, SimTime::from_ps(0));
        assert_eq!(SimTime::default(), SimTime::EPOCH);
    }

    /// A `u64` of pico seconds tops out at ~5124 hours, so the wide
    /// constructors are what the `u128` field buys.
    #[test]
    fn constructors_span_past_the_u64_pico_second_range() {
        assert_eq!(SimTime::from_hour(1_000_000).as_sec(), 3_600_000_000);
        assert_eq!(SimTime::from_ns(u64::MAX).as_ps(), u64::MAX as u128 * 1_000);
    }

    #[test]
    fn accessors_truncate_the_remainder() {
        let t = SimTime::from_sec(1) + Duration::ps(1);
        assert_eq!(t.as_ps(), 1_000_000_000_001);
        assert_eq!(t.as_ns(), 1_000_000_000);
        assert_eq!(t.as_us(), 1_000_000);
        assert_eq!(t.as_ms(), 1_000);
        assert_eq!(t.as_sec(), 1);
        assert_eq!(SimTime::from_ps(999).as_ns(), 0);
    }

    /// Both types count pico seconds, so nothing is lost crossing between them.
    #[test]
    fn round_trips_through_duration_exactly() {
        let t = SimTime::from_ps(1_500_000_000_001);
        assert_eq!(t.since_epoch(), Duration::ps(1_500_000_000_001));
        assert_eq!(SimTime::EPOCH + t.since_epoch(), t);
        assert_eq!(t - SimTime::from_sec(1), Duration::ps(500_000_000_001));
    }

    #[test]
    fn arithmetic_steps_forward_and_back() {
        let mut t = SimTime::from_us(1);
        t += Duration::ns(500);
        assert_eq!(t, SimTime::from_ps(1_500_000));
        t -= Duration::ns(500);
        assert_eq!(t, SimTime::from_us(1));
        assert_eq!(t - Duration::us(1), SimTime::EPOCH);
    }

    #[test]
    fn duration_since_handles_a_later_earlier() {
        let earlier = SimTime::from_ms(1);
        let later = SimTime::from_ms(3);
        assert_eq!(later.checked_duration_since(earlier), Some(Duration::ms(2)));
        assert_eq!(earlier.checked_duration_since(later), None);
        assert_eq!(later.saturating_duration_since(earlier), Duration::ms(2));
        assert_eq!(earlier.saturating_duration_since(later), Duration::ZERO);
        assert_eq!(later.saturating_duration_since(later), Duration::ZERO);
    }

    #[test]
    fn checked_and_saturating_edges() {
        let max = SimTime::from_ps(u128::MAX);
        assert_eq!(max.checked_add(Duration::ps(1)), None);
        assert_eq!(SimTime::EPOCH.checked_sub(Duration::ps(1)), None);
        assert_eq!(
            SimTime::EPOCH.checked_add(Duration::ms(1)),
            Some(SimTime::from_ms(1))
        );
        assert_eq!(
            SimTime::from_ms(1).checked_sub(Duration::ms(1)),
            Some(SimTime::EPOCH)
        );
        assert_eq!(max.saturating_add(Duration::MAX), max);
        assert_eq!(SimTime::EPOCH.saturating_sub(Duration::MAX), SimTime::EPOCH);
    }

    #[test]
    fn orders_chronologically() {
        let mut times = [SimTime::from_sec(1), SimTime::EPOCH, SimTime::from_ps(1)];
        times.sort();
        assert_eq!(
            times,
            [SimTime::EPOCH, SimTime::from_ps(1), SimTime::from_sec(1)]
        );
    }

    #[test]
    fn display_reads_as_a_span_since_epoch() {
        assert_eq!(SimTime::EPOCH.to_string(), "0ps since epoch");
        assert_eq!(SimTime::from_ms(2).to_string(), "2.000ms since epoch");
    }
}
