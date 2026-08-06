use std::fmt;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

/// Carries a pico second value
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Duration(u128);

impl Duration {
    pub const ZERO: Duration = Duration(0);
    pub const MAX: Duration = Duration(u128::MAX);

    pub fn ps(ps: u128) -> Self {
        Self(ps)
    }

    pub fn ns(ns: u64) -> Self {
        Self(ns as u128 * 1_000)
    }

    pub fn us(us: u64) -> Self {
        Self(us as u128 * 1_000_000)
    }

    pub fn ms(ms: u64) -> Self {
        Self(ms as u128 * 1_000_000_000)
    }

    pub fn sec(sec: u32) -> Self {
        Self(sec as u128 * 1_000_000_000_000)
    }

    pub fn min(min: u32) -> Self {
        Self(min as u128 * 60_000_000_000_000)
    }

    pub fn hour(hour: u32) -> Self {
        Self(hour as u128 * 3_600_000_000_000_000)
    }

    pub fn as_ps(&self) -> u128 {
        self.0
    }

    pub fn as_ns(&self) -> u128 {
        self.0 / 1_000
    }

    pub fn as_us(&self) -> u128 {
        self.0 / 1_000_000
    }

    pub fn as_ms(&self) -> u128 {
        self.0 / 1_000_000_000
    }

    pub fn as_sec(&self) -> u128 {
        self.0 / 1_000_000_000_000
    }

    /// Seconds as a float, keeping the sub second part.
    pub fn as_sec_f64(&self) -> f64 {
        self.0 as f64 / 1_000_000_000_000.0
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn checked_add(self, other: Duration) -> Option<Duration> {
        self.0.checked_add(other.0).map(Duration)
    }

    pub fn checked_sub(self, other: Duration) -> Option<Duration> {
        self.0.checked_sub(other.0).map(Duration)
    }

    pub fn checked_mul(self, factor: u128) -> Option<Duration> {
        self.0.checked_mul(factor).map(Duration)
    }

    pub fn saturating_add(self, other: Duration) -> Duration {
        Duration(self.0.saturating_add(other.0))
    }

    /// Clamps at [`Duration::ZERO`] instead of underflowing.
    pub fn saturating_sub(self, other: Duration) -> Duration {
        Duration(self.0.saturating_sub(other.0))
    }

    pub fn saturating_mul(self, factor: u128) -> Duration {
        Duration(self.0.saturating_mul(factor))
    }

    /// Scales by an arbitrary factor, rounding down. Useful for rates and
    /// derating (e.g. `d.scale(0.5)` to halve a delay). Loses precision above
    /// what an `f64` can hold exactly, so prefer [`Duration::checked_mul`] and
    /// [`Div`] for exact ratios.
    pub fn scale(self, factor: f64) -> Duration {
        Duration((self.0 as f64 * factor) as u128)
    }
}

impl Add for Duration {
    type Output = Duration;

    fn add(self, other: Duration) -> Duration {
        Duration(self.0 + other.0)
    }
}

impl AddAssign for Duration {
    fn add_assign(&mut self, other: Duration) {
        self.0 += other.0;
    }
}

impl Sub for Duration {
    type Output = Duration;

    fn sub(self, other: Duration) -> Duration {
        Duration(self.0 - other.0)
    }
}

impl SubAssign for Duration {
    fn sub_assign(&mut self, other: Duration) {
        self.0 -= other.0;
    }
}

impl Mul<u128> for Duration {
    type Output = Duration;

    fn mul(self, factor: u128) -> Duration {
        Duration(self.0 * factor)
    }
}

impl Mul<Duration> for u128 {
    type Output = Duration;

    fn mul(self, duration: Duration) -> Duration {
        Duration(self * duration.0)
    }
}

impl Div<u128> for Duration {
    type Output = Duration;

    fn div(self, divisor: u128) -> Duration {
        Duration(self.0 / divisor)
    }
}

/// Ratio of two durations, truncating.
impl Div<Duration> for Duration {
    type Output = u128;

    fn div(self, other: Duration) -> u128 {
        self.0 / other.0
    }
}

impl Sum for Duration {
    fn sum<I: Iterator<Item = Duration>>(iter: I) -> Duration {
        iter.fold(Duration::ZERO, |acc, d| acc + d)
    }
}

impl<'a> Sum<&'a Duration> for Duration {
    fn sum<I: Iterator<Item = &'a Duration>>(iter: I) -> Duration {
        iter.fold(Duration::ZERO, |acc, d| acc + *d)
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ps = self.0;
        if ps < 1_000 {
            write!(f, "{}ps", ps)
        } else if ps < 1_000_000 {
            write!(f, "{:.3}ns", ps as f64 / 1_000.0)
        } else if ps < 1_000_000_000 {
            write!(f, "{:.3}us", ps as f64 / 1_000_000.0)
        } else if ps < 1_000_000_000_000 {
            write!(f, "{:.3}ms", ps as f64 / 1_000_000_000.0)
        } else {
            write!(f, "{:.3}s", ps as f64 / 1_000_000_000_000.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_unit_is_a_thousandfold_of_the_next() {
        assert_eq!(Duration::ns(1), Duration::ps(1_000));
        assert_eq!(Duration::us(1), Duration::ns(1_000));
        assert_eq!(Duration::ms(1), Duration::us(1_000));
        assert_eq!(Duration::sec(1), Duration::ms(1_000));
        assert_eq!(Duration::min(1), Duration::sec(60));
        assert_eq!(Duration::hour(1), Duration::min(60));
        assert_eq!(Duration::sec(1).as_ps(), 1_000_000_000_000);
    }

    /// A `u64` of pico seconds tops out at ~5124 hours, so the wide
    /// constructors are what the `u128` field buys.
    #[test]
    fn constructors_span_past_the_u64_pico_second_range() {
        assert_eq!(Duration::hour(1_000_000).as_sec(), 3_600_000_000);
        assert_eq!(Duration::ns(u64::MAX).as_ps(), u64::MAX as u128 * 1_000);
    }

    #[test]
    fn accessors_truncate_the_remainder() {
        let d = Duration::sec(1) + Duration::ps(1);
        assert_eq!(d.as_ps(), 1_000_000_000_001);
        assert_eq!(d.as_ns(), 1_000_000_000);
        assert_eq!(d.as_us(), 1_000_000);
        assert_eq!(d.as_ms(), 1_000);
        assert_eq!(d.as_sec(), 1);
        assert_eq!(Duration::ps(999).as_ns(), 0);
    }

    #[test]
    fn as_sec_f64_keeps_the_sub_second_part() {
        assert_eq!(Duration::ms(1_500).as_sec_f64(), 1.5);
        assert_eq!(Duration::ps(1).as_sec_f64(), 1e-12);
    }

    #[test]
    fn zero_and_max() {
        assert!(Duration::ZERO.is_zero());
        assert!(!Duration::ps(1).is_zero());
        assert_eq!(Duration::MAX.as_ps(), u128::MAX);
        assert!(Duration::MAX > Duration::hour(u32::MAX));
    }

    #[test]
    fn arithmetic() {
        let mut d = Duration::us(1);
        d += Duration::ns(500);
        assert_eq!(d, Duration::ps(1_500_000));
        d -= Duration::ns(500);
        assert_eq!(d, Duration::us(1));
        assert_eq!(Duration::ms(3) - Duration::ms(1), Duration::ms(2));
        assert_eq!(Duration::ms(2) * 3, Duration::ms(6));
        assert_eq!(3 * Duration::ms(2), Duration::ms(6));
        assert_eq!(Duration::ms(6) / 4, Duration::ms(1) + Duration::us(500));
        assert_eq!(Duration::ms(7) / Duration::ms(2), 3);
    }

    #[test]
    fn checked_and_saturating_edges() {
        assert_eq!(Duration::MAX.checked_add(Duration::ps(1)), None);
        assert_eq!(Duration::ZERO.checked_sub(Duration::ps(1)), None);
        assert_eq!(Duration::MAX.checked_mul(2), None);
        assert_eq!(Duration::ps(2).checked_mul(3), Some(Duration::ps(6)));
        assert_eq!(Duration::MAX.saturating_add(Duration::ps(1)), Duration::MAX);
        assert_eq!(
            Duration::ZERO.saturating_sub(Duration::ps(1)),
            Duration::ZERO
        );
        assert_eq!(Duration::MAX.saturating_mul(2), Duration::MAX);
    }

    #[test]
    fn scale_rounds_down() {
        assert_eq!(Duration::sec(1).scale(0.5), Duration::ms(500));
        assert_eq!(Duration::ps(3).scale(0.5), Duration::ps(1));
        assert_eq!(Duration::ms(1).scale(2.0), Duration::ms(2));
    }

    #[test]
    fn sums_owned_and_borrowed() {
        let ds = [Duration::ns(1), Duration::ps(500), Duration::ps(500)];
        assert_eq!(ds.iter().sum::<Duration>(), Duration::ns(2));
        assert_eq!(ds.into_iter().sum::<Duration>(), Duration::ns(2));
        assert_eq!(
            std::iter::empty::<Duration>().sum::<Duration>(),
            Duration::ZERO
        );
    }

    #[test]
    fn display_picks_the_largest_fitting_unit() {
        assert_eq!(Duration::ZERO.to_string(), "0ps");
        assert_eq!(Duration::ps(999).to_string(), "999ps");
        assert_eq!(Duration::ns(1).to_string(), "1.000ns");
        assert_eq!(Duration::us(1500).to_string(), "1.500ms");
        assert_eq!(Duration::ms(2).to_string(), "2.000ms");
        assert_eq!(Duration::sec(90).to_string(), "90.000s");
    }
}
