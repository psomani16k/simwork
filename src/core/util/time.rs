use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::core::util::duration::Duration;

/// Carries a nano second value
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct SimTime(u64);

impl SimTime {
    pub const EPOCH: SimTime = SimTime(0);

    pub fn from_ns(ns: u64) -> Self {
        Self(ns)
    }

    pub fn from_us(us: u64) -> Self {
        Self(us * 1_000)
    }

    pub fn from_ms(ms: u64) -> Self {
        Self(ms * 1_000_000)
    }

    pub fn from_sec(sec: u32) -> Self {
        Self(sec as u64 * 1_000_000_000)
    }

    pub fn from_min(min: u32) -> Self {
        Self(min as u64 * 60_000_000_000)
    }

    pub fn from_hour(hour: u32) -> Self {
        Self(hour as u64 * 3_600_000_000_000)
    }

    /// Exact nano second count since [`SimTime::EPOCH`].
    pub fn as_ns(&self) -> u64 {
        self.0
    }

    /// Whole micro seconds since [`SimTime::EPOCH`], truncating any remainder.
    pub fn as_us(&self) -> u64 {
        self.0 / 1_000
    }

    /// Whole milli seconds since [`SimTime::EPOCH`], truncating any remainder.
    pub fn as_ms(&self) -> u64 {
        self.0 / 1_000_000
    }

    /// Whole seconds since [`SimTime::EPOCH`], truncating any remainder.
    pub fn as_sec(&self) -> u64 {
        self.0 / 1_000_000_000
    }

    /// Time elapsed since [`SimTime::EPOCH`].
    pub fn since_epoch(&self) -> Duration {
        Duration::ns(self.0)
    }

    /// Time elapsed since `earlier`, or `None` if `earlier` is later than `self`.
    pub fn checked_duration_since(&self, earlier: SimTime) -> Option<Duration> {
        self.0.checked_sub(earlier.0).map(Duration::ns)
    }

    /// Time elapsed since `earlier`, clamped to [`Duration::ZERO`] if `earlier` is later than `self`.
    pub fn saturating_duration_since(&self, earlier: SimTime) -> Duration {
        Duration::ns(self.0.saturating_sub(earlier.0))
    }

    pub fn checked_add(self, duration: Duration) -> Option<SimTime> {
        self.0.checked_add(duration.as_ns()).map(SimTime)
    }

    pub fn checked_sub(self, duration: Duration) -> Option<SimTime> {
        self.0.checked_sub(duration.as_ns()).map(SimTime)
    }

    pub fn saturating_add(self, duration: Duration) -> SimTime {
        SimTime(self.0.saturating_add(duration.as_ns()))
    }

    /// Clamps at [`SimTime::EPOCH`] instead of underflowing.
    pub fn saturating_sub(self, duration: Duration) -> SimTime {
        SimTime(self.0.saturating_sub(duration.as_ns()))
    }
}

impl Add<Duration> for SimTime {
    type Output = SimTime;

    fn add(self, duration: Duration) -> SimTime {
        SimTime(self.0 + duration.as_ns())
    }
}

impl AddAssign<Duration> for SimTime {
    fn add_assign(&mut self, duration: Duration) {
        self.0 += duration.as_ns();
    }
}

/// Steps back in time by `duration`.
impl Sub<Duration> for SimTime {
    type Output = SimTime;

    fn sub(self, duration: Duration) -> SimTime {
        SimTime(self.0 - duration.as_ns())
    }
}

impl SubAssign<Duration> for SimTime {
    fn sub_assign(&mut self, duration: Duration) {
        self.0 -= duration.as_ns();
    }
}

/// Elapsed time between two points, panicking on underflow. Prefer
/// [`SimTime::checked_duration_since`] or [`SimTime::saturating_duration_since`]
/// when `other` might not precede `self`.
impl Sub<SimTime> for SimTime {
    type Output = Duration;

    fn sub(self, other: SimTime) -> Duration {
        Duration::ns(self.0 - other.0)
    }
}

impl fmt::Display for SimTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} since epoch", self.since_epoch())
    }
}
