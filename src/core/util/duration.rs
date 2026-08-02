use std::fmt;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Duration(u64);

impl Duration {
    pub const ZERO: Duration = Duration(0);
    pub const MAX: Duration = Duration(u64::MAX);

    pub fn ns(ns: u64) -> Self {
        Self(ns)
    }

    pub fn us(us: u64) -> Self {
        Self(us * 1_000)
    }

    pub fn ms(ms: u64) -> Self {
        Self(ms * 1_000_000)
    }

    pub fn sec(sec: u32) -> Self {
        Self(sec as u64 * 1_000_000_000)
    }

    pub fn min(min: u32) -> Self {
        Self(min as u64 * 60_000_000_000)
    }

    pub fn hour(hour: u32) -> Self {
        Self(hour as u64 * 3_600_000_000_000)
    }

    /// Exact nano second count.
    pub fn as_ns(&self) -> u64 {
        self.0
    }

    /// Whole micro seconds, truncating any remainder.
    pub fn as_us(&self) -> u64 {
        self.0 / 1_000
    }

    /// Whole milli seconds, truncating any remainder.
    pub fn as_ms(&self) -> u64 {
        self.0 / 1_000_000
    }

    /// Whole seconds, truncating any remainder.
    pub fn as_sec(&self) -> u64 {
        self.0 / 1_000_000_000
    }

    /// Seconds as a float, keeping the sub second part.
    pub fn as_sec_f64(&self) -> f64 {
        self.0 as f64 / 1_000_000_000.0
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

    pub fn checked_mul(self, factor: u64) -> Option<Duration> {
        self.0.checked_mul(factor).map(Duration)
    }

    pub fn saturating_add(self, other: Duration) -> Duration {
        Duration(self.0.saturating_add(other.0))
    }

    /// Clamps at [`Duration::ZERO`] instead of underflowing.
    pub fn saturating_sub(self, other: Duration) -> Duration {
        Duration(self.0.saturating_sub(other.0))
    }

    pub fn saturating_mul(self, factor: u64) -> Duration {
        Duration(self.0.saturating_mul(factor))
    }

    /// Scales by an arbitrary factor, rounding down. Useful for rates and
    /// derating (e.g. `d.scale(0.5)` to halve a delay).
    pub fn scale(self, factor: f64) -> Duration {
        Duration((self.0 as f64 * factor) as u64)
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

impl Mul<u64> for Duration {
    type Output = Duration;

    fn mul(self, factor: u64) -> Duration {
        Duration(self.0 * factor)
    }
}

impl Mul<Duration> for u64 {
    type Output = Duration;

    fn mul(self, duration: Duration) -> Duration {
        Duration(self * duration.0)
    }
}

impl Div<u64> for Duration {
    type Output = Duration;

    fn div(self, divisor: u64) -> Duration {
        Duration(self.0 / divisor)
    }
}

/// Ratio of two durations, truncating.
impl Div<Duration> for Duration {
    type Output = u64;

    fn div(self, other: Duration) -> u64 {
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
        let ns = self.0;
        if ns < 1_000 {
            write!(f, "{}ns", ns)
        } else if ns < 1_000_000 {
            write!(f, "{:.3}us", ns as f64 / 1_000.0)
        } else if ns < 1_000_000_000 {
            write!(f, "{:.3}ms", ns as f64 / 1_000_000.0)
        } else {
            write!(f, "{:.3}s", ns as f64 / 1_000_000_000.0)
        }
    }
}
