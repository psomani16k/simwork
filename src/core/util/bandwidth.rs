use std::fmt;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

use crate::core::util::duration::Duration;

/// Carries bits/sec
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Bandwidth(u64);

impl Bandwidth {
    pub const ZERO: Bandwidth = Bandwidth(0);
    pub const MAX: Bandwidth = Bandwidth(u64::MAX);

    pub fn from_bits_per_sec(bw: u64) -> Self {
        Self(bw)
    }

    pub fn from_kilo_bits_per_sec(bw: u32) -> Self {
        Self::from_bits_per_sec(bw as u64) * 1_000
    }

    pub fn from_mega_bits_per_sec(bw: u32) -> Self {
        Self::from_kilo_bits_per_sec(bw) * 1_000
    }

    pub fn from_giga_bits_per_sec(bw: u32) -> Self {
        Self::from_mega_bits_per_sec(bw) * 1_000
    }

    pub fn from_tera_bits_per_sec(bw: u32) -> Self {
        Self::from_giga_bits_per_sec(bw) * 1_000
    }

    pub fn from_bytes_per_sec(bw: u64) -> Self {
        Self::from_bits_per_sec(bw) * 8
    }

    pub fn from_kilo_bytes_per_sec(bw: u32) -> Self {
        Self::from_bytes_per_sec(bw as u64) * 1_000
    }

    pub fn from_mega_bytes_per_sec(bw: u32) -> Self {
        Self::from_kilo_bytes_per_sec(bw) * 1_000
    }

    pub fn from_giga_bytes_per_sec(bw: u32) -> Self {
        Self::from_mega_bytes_per_sec(bw) * 1_000
    }

    pub fn from_tera_bytes_per_sec(bw: u32) -> Self {
        Self::from_giga_bytes_per_sec(bw) * 1_000
    }

    pub fn from_kibi_bits_per_sec(bw: u32) -> Self {
        Self::from_bits_per_sec(bw as u64) * 1_024
    }

    pub fn from_mebi_bits_per_sec(bw: u32) -> Self {
        Self::from_kibi_bits_per_sec(bw) * 1_024
    }

    pub fn from_gibi_bits_per_sec(bw: u32) -> Self {
        Self::from_mebi_bits_per_sec(bw) * 1_024
    }

    pub fn from_tebi_bits_per_sec(bw: u32) -> Self {
        Self::from_gibi_bits_per_sec(bw) * 1_024
    }

    pub fn from_kibi_bytes_per_sec(bw: u32) -> Self {
        Self::from_bytes_per_sec(bw as u64) * 1_024
    }

    pub fn from_mebi_bytes_per_sec(bw: u32) -> Self {
        Self::from_kibi_bytes_per_sec(bw) * 1_024
    }

    pub fn from_gibi_bytes_per_sec(bw: u32) -> Self {
        Self::from_mebi_bytes_per_sec(bw) * 1_024
    }

    pub fn from_tebi_bytes_per_sec(bw: u32) -> Self {
        Self::from_gibi_bytes_per_sec(bw) * 1_024
    }

    pub fn as_bits_per_sec(&self) -> u64 {
        self.0
    }

    pub fn as_kilo_bits_per_sec(&self) -> u64 {
        self.as_bits_per_sec() / 1_000
    }

    pub fn as_mega_bits_per_sec(&self) -> u64 {
        self.as_kilo_bits_per_sec() / 1_000
    }

    pub fn as_giga_bits_per_sec(&self) -> u64 {
        self.as_mega_bits_per_sec() / 1_000
    }

    pub fn as_tera_bits_per_sec(&self) -> u64 {
        self.as_giga_bits_per_sec() / 1_000
    }

    /// Whole bytes/sec, truncating any sub byte remainder.
    pub fn as_bytes_per_sec(&self) -> u64 {
        self.as_bits_per_sec() / 8
    }

    pub fn as_kilo_bytes_per_sec(&self) -> u64 {
        self.as_bytes_per_sec() / 1_000
    }

    pub fn as_mega_bytes_per_sec(&self) -> u64 {
        self.as_kilo_bytes_per_sec() / 1_000
    }

    pub fn as_giga_bytes_per_sec(&self) -> u64 {
        self.as_mega_bytes_per_sec() / 1_000
    }

    pub fn as_tera_bytes_per_sec(&self) -> u64 {
        self.as_giga_bytes_per_sec() / 1_000
    }

    pub fn as_kibi_bits_per_sec(&self) -> u64 {
        self.as_bits_per_sec() / 1_024
    }

    pub fn as_mebi_bits_per_sec(&self) -> u64 {
        self.as_kibi_bits_per_sec() / 1_024
    }

    pub fn as_gibi_bits_per_sec(&self) -> u64 {
        self.as_mebi_bits_per_sec() / 1_024
    }

    pub fn as_tebi_bits_per_sec(&self) -> u64 {
        self.as_gibi_bits_per_sec() / 1_024
    }

    pub fn as_kibi_bytes_per_sec(&self) -> u64 {
        self.as_bytes_per_sec() / 1_024
    }

    pub fn as_mebi_bytes_per_sec(&self) -> u64 {
        self.as_kibi_bytes_per_sec() / 1_024
    }

    pub fn as_gibi_bytes_per_sec(&self) -> u64 {
        self.as_mebi_bytes_per_sec() / 1_024
    }

    pub fn as_tebi_bytes_per_sec(&self) -> u64 {
        self.as_gibi_bytes_per_sec() / 1_024
    }

    /// Mega bits/sec as a float, keeping the sub unit part.
    pub fn as_mega_bits_per_sec_f64(&self) -> f64 {
        self.0 as f64 / 1_000_000.0
    }

    /// Giga bits/sec as a float, keeping the sub unit part.
    pub fn as_giga_bits_per_sec_f64(&self) -> f64 {
        self.0 as f64 / 1_000_000_000.0
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    /// Time needed to serialise `bits` onto a link of this rate, truncating any
    /// sub pico second remainder. `None` at [`Bandwidth::ZERO`], where nothing
    /// ever finishes.
    pub fn transmission_time(&self, bits: u64) -> Option<Duration> {
        (bits as u128 * 1_000_000_000_000)
            .checked_div(self.0 as u128)
            .map(Duration::ps)
    }

    /// Same as [`Bandwidth::transmission_time`], panicking at
    /// [`Bandwidth::ZERO`] instead of reporting it.
    pub fn unchecked_transmission_time(&self, bits: u64) -> Duration {
        self.transmission_time(bits)
            .expect("nothing ever serialises onto a zero bandwidth link")
    }

    /// Time needed to serialise `bytes` bytes, truncating any sub pico second
    /// remainder. `None` at [`Bandwidth::ZERO`], or if `bytes` is too wide to
    /// count in bits.
    pub fn transmission_time_bytes(&self, bytes: u64) -> Option<Duration> {
        bytes
            .checked_mul(8)
            .and_then(|bits| self.transmission_time(bits))
    }

    /// Same as [`Bandwidth::transmission_time_bytes`], panicking instead of
    /// reporting either edge.
    pub fn unchecked_transmission_time_bytes(&self, bytes: u64) -> Duration {
        self.transmission_time_bytes(bytes)
            .expect("nothing ever serialises onto a zero bandwidth link")
    }

    /// Bits carried over `duration` at this rate, truncating any partial bit.
    /// `None` if the count overflows a `u128`.
    pub fn bits_in(&self, duration: Duration) -> Option<u128> {
        (self.0 as u128)
            .checked_mul(duration.as_ps())
            .map(|bits| bits / 1_000_000_000_000)
    }

    /// Same as [`Bandwidth::bits_in`], panicking on overflow instead of
    /// reporting it.
    pub fn unchecked_bits_in(&self, duration: Duration) -> u128 {
        self.bits_in(duration)
            .expect("bits carried over the span overflow a u128")
    }

    /// Whole bytes carried over `duration` at this rate, truncating. `None` if
    /// the count overflows a `u128`.
    pub fn bytes_in(&self, duration: Duration) -> Option<u128> {
        self.bits_in(duration).map(|bits| bits / 8)
    }

    /// Same as [`Bandwidth::bytes_in`], panicking on overflow instead of
    /// reporting it.
    pub fn unchecked_bytes_in(&self, duration: Duration) -> u128 {
        self.bytes_in(duration)
            .expect("bytes carried over the span overflow a u128")
    }

    pub fn checked_add(self, other: Bandwidth) -> Option<Bandwidth> {
        self.0.checked_add(other.0).map(Bandwidth)
    }

    pub fn checked_sub(self, other: Bandwidth) -> Option<Bandwidth> {
        self.0.checked_sub(other.0).map(Bandwidth)
    }

    pub fn checked_mul(self, factor: u64) -> Option<Bandwidth> {
        self.0.checked_mul(factor).map(Bandwidth)
    }

    pub fn checked_div(self, divisor: u64) -> Option<Bandwidth> {
        self.0.checked_div(divisor).map(Bandwidth)
    }

    pub fn saturating_add(self, other: Bandwidth) -> Bandwidth {
        Bandwidth(self.0.saturating_add(other.0))
    }

    /// Clamps at [`Bandwidth::ZERO`] instead of underflowing.
    pub fn saturating_sub(self, other: Bandwidth) -> Bandwidth {
        Bandwidth(self.0.saturating_sub(other.0))
    }

    pub fn saturating_mul(self, factor: u64) -> Bandwidth {
        Bandwidth(self.0.saturating_mul(factor))
    }

    /// Scales by an arbitrary factor, rounding down. Useful for derating a link
    /// (e.g. `bw.scale(0.9)` for protocol overhead). Loses precision above what
    /// an `f64` can hold exactly, so prefer [`Bandwidth::checked_mul`] and
    /// [`Div`] for exact ratios.
    pub fn scale(self, factor: f64) -> Bandwidth {
        Bandwidth((self.0 as f64 * factor) as u64)
    }
}

impl Add for Bandwidth {
    type Output = Bandwidth;

    fn add(self, other: Bandwidth) -> Bandwidth {
        Bandwidth(self.0 + other.0)
    }
}

impl AddAssign for Bandwidth {
    fn add_assign(&mut self, other: Bandwidth) {
        self.0 += other.0;
    }
}

impl Sub for Bandwidth {
    type Output = Bandwidth;

    fn sub(self, other: Bandwidth) -> Bandwidth {
        Bandwidth(self.0 - other.0)
    }
}

impl SubAssign for Bandwidth {
    fn sub_assign(&mut self, other: Bandwidth) {
        self.0 -= other.0;
    }
}

impl Mul<u64> for Bandwidth {
    type Output = Bandwidth;

    fn mul(self, factor: u64) -> Bandwidth {
        Bandwidth(self.0 * factor)
    }
}

impl Mul<Bandwidth> for u64 {
    type Output = Bandwidth;

    fn mul(self, bandwidth: Bandwidth) -> Bandwidth {
        Bandwidth(self * bandwidth.0)
    }
}

impl Div<u64> for Bandwidth {
    type Output = Bandwidth;

    fn div(self, divisor: u64) -> Bandwidth {
        Bandwidth(self.0 / divisor)
    }
}

/// Ratio of two rates, truncating.
impl Div<Bandwidth> for Bandwidth {
    type Output = u64;

    fn div(self, other: Bandwidth) -> u64 {
        self.0 / other.0
    }
}

impl Sum for Bandwidth {
    fn sum<I: Iterator<Item = Bandwidth>>(iter: I) -> Bandwidth {
        iter.fold(Bandwidth::ZERO, |acc, bw| acc + bw)
    }
}

impl<'a> Sum<&'a Bandwidth> for Bandwidth {
    fn sum<I: Iterator<Item = &'a Bandwidth>>(iter: I) -> Bandwidth {
        iter.fold(Bandwidth::ZERO, |acc, bw| acc + *bw)
    }
}

impl fmt::Display for Bandwidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bps = self.0;
        if bps < 1_000 {
            write!(f, "{}bps", bps)
        } else if bps < 1_000_000 {
            write!(f, "{:.3}kbps", bps as f64 / 1_000.0)
        } else if bps < 1_000_000_000 {
            write!(f, "{:.3}Mbps", bps as f64 / 1_000_000.0)
        } else if bps < 1_000_000_000_000 {
            write!(f, "{:.3}Gbps", bps as f64 / 1_000_000_000.0)
        } else {
            write!(f, "{:.3}Tbps", bps as f64 / 1_000_000_000_000.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_unit_is_a_thousandfold_of_the_next() {
        assert_eq!(
            Bandwidth::from_kilo_bits_per_sec(1),
            Bandwidth::from_bits_per_sec(1_000)
        );
        assert_eq!(
            Bandwidth::from_mega_bits_per_sec(1),
            Bandwidth::from_kilo_bits_per_sec(1_000)
        );
        assert_eq!(
            Bandwidth::from_giga_bits_per_sec(1),
            Bandwidth::from_mega_bits_per_sec(1_000)
        );
        assert_eq!(
            Bandwidth::from_tera_bits_per_sec(1),
            Bandwidth::from_giga_bits_per_sec(1_000)
        );
        assert_eq!(
            Bandwidth::from_kilo_bytes_per_sec(1),
            Bandwidth::from_bytes_per_sec(1_000)
        );
        assert_eq!(
            Bandwidth::from_mega_bytes_per_sec(1),
            Bandwidth::from_kilo_bytes_per_sec(1_000)
        );
        assert_eq!(
            Bandwidth::from_giga_bytes_per_sec(1),
            Bandwidth::from_mega_bytes_per_sec(1_000)
        );
        assert_eq!(
            Bandwidth::from_tera_bytes_per_sec(1),
            Bandwidth::from_giga_bytes_per_sec(1_000)
        );
    }

    #[test]
    fn a_byte_is_eight_bits() {
        assert_eq!(
            Bandwidth::from_bytes_per_sec(1),
            Bandwidth::from_bits_per_sec(8)
        );
        assert_eq!(
            Bandwidth::from_mega_bytes_per_sec(1),
            Bandwidth::from_mega_bits_per_sec(8)
        );
        assert_eq!(Bandwidth::from_bits_per_sec(15).as_bytes_per_sec(), 1);
    }

    /// Each unit steps up the rate rather than the `u32` argument, so values
    /// past `u32::MAX` bits/sec survive the chain down to the base unit.
    #[test]
    fn wide_constructors_do_not_overflow_on_the_way_down() {
        assert_eq!(
            Bandwidth::from_tera_bits_per_sec(5).as_bits_per_sec(),
            5_000_000_000_000
        );
        assert_eq!(
            Bandwidth::from_giga_bytes_per_sec(1_000_000).as_bits_per_sec(),
            8_000_000_000_000_000
        );
        assert_eq!(
            Bandwidth::from_tebi_bits_per_sec(5).as_bits_per_sec(),
            5 * 1_099_511_627_776
        );
    }

    #[test]
    fn each_binary_unit_is_a_1024_fold_of_the_next() {
        assert_eq!(
            Bandwidth::from_kibi_bits_per_sec(1),
            Bandwidth::from_bits_per_sec(1_024)
        );
        assert_eq!(
            Bandwidth::from_mebi_bits_per_sec(1),
            Bandwidth::from_kibi_bits_per_sec(1_024)
        );
        assert_eq!(
            Bandwidth::from_gibi_bits_per_sec(1),
            Bandwidth::from_mebi_bits_per_sec(1_024)
        );
        assert_eq!(
            Bandwidth::from_tebi_bits_per_sec(1),
            Bandwidth::from_gibi_bits_per_sec(1_024)
        );
        assert_eq!(
            Bandwidth::from_kibi_bytes_per_sec(1),
            Bandwidth::from_bytes_per_sec(1_024)
        );
        assert_eq!(
            Bandwidth::from_mebi_bytes_per_sec(1),
            Bandwidth::from_kibi_bytes_per_sec(1_024)
        );
        assert_eq!(
            Bandwidth::from_gibi_bytes_per_sec(1),
            Bandwidth::from_mebi_bytes_per_sec(1_024)
        );
        assert_eq!(
            Bandwidth::from_tebi_bytes_per_sec(1),
            Bandwidth::from_gibi_bytes_per_sec(1_024)
        );
        assert_eq!(
            Bandwidth::from_gibi_bytes_per_sec(1),
            Bandwidth::from_gibi_bits_per_sec(8)
        );
    }

    /// The binary units are the larger of the two ladders at every step, which
    /// is exactly why mixing them up misreports a link.
    #[test]
    fn binary_units_run_ahead_of_the_decimal_ones() {
        assert_eq!(
            Bandwidth::from_kibi_bits_per_sec(1).as_bits_per_sec(),
            1_024
        );
        assert_eq!(
            Bandwidth::from_gibi_bits_per_sec(1).as_bits_per_sec(),
            1_073_741_824
        );
        assert!(Bandwidth::from_gibi_bits_per_sec(1) > Bandwidth::from_giga_bits_per_sec(1));
        // 1Gbps is just under 1Gibps, so the whole count truncates to zero.
        assert_eq!(
            Bandwidth::from_giga_bits_per_sec(1).as_gibi_bits_per_sec(),
            0
        );
        assert_eq!(
            Bandwidth::from_giga_bits_per_sec(2).as_gibi_bits_per_sec(),
            1
        );
    }

    #[test]
    fn binary_accessors_truncate_the_remainder() {
        let bw = Bandwidth::from_gibi_bits_per_sec(1) + Bandwidth::from_bits_per_sec(1);
        assert_eq!(bw.as_bits_per_sec(), 1_073_741_825);
        assert_eq!(bw.as_kibi_bits_per_sec(), 1_048_576);
        assert_eq!(bw.as_mebi_bits_per_sec(), 1_024);
        assert_eq!(bw.as_gibi_bits_per_sec(), 1);
        assert_eq!(bw.as_tebi_bits_per_sec(), 0);
        assert_eq!(bw.as_kibi_bytes_per_sec(), 131_072);
        assert_eq!(bw.as_mebi_bytes_per_sec(), 128);
        assert_eq!(bw.as_gibi_bytes_per_sec(), 0);
        assert_eq!(bw.as_tebi_bytes_per_sec(), 0);
    }

    #[test]
    fn constructors_and_accessors_round_trip() {
        assert_eq!(
            Bandwidth::from_mebi_bits_per_sec(7).as_mebi_bits_per_sec(),
            7
        );
        assert_eq!(
            Bandwidth::from_tebi_bytes_per_sec(3).as_tebi_bytes_per_sec(),
            3
        );
        assert_eq!(
            Bandwidth::from_mega_bits_per_sec(7).as_mega_bits_per_sec(),
            7
        );
        assert_eq!(
            Bandwidth::from_tera_bytes_per_sec(3).as_tera_bytes_per_sec(),
            3
        );
    }

    #[test]
    fn accessors_truncate_the_remainder() {
        let bw = Bandwidth::from_giga_bits_per_sec(1) + Bandwidth::from_bits_per_sec(1);
        assert_eq!(bw.as_bits_per_sec(), 1_000_000_001);
        assert_eq!(bw.as_kilo_bits_per_sec(), 1_000_000);
        assert_eq!(bw.as_mega_bits_per_sec(), 1_000);
        assert_eq!(bw.as_giga_bits_per_sec(), 1);
        assert_eq!(bw.as_tera_bits_per_sec(), 0);
        assert_eq!(bw.as_bytes_per_sec(), 125_000_000);
        assert_eq!(bw.as_kilo_bytes_per_sec(), 125_000);
        assert_eq!(bw.as_mega_bytes_per_sec(), 125);
        assert_eq!(bw.as_giga_bytes_per_sec(), 0);
        assert_eq!(bw.as_tera_bytes_per_sec(), 0);
    }

    #[test]
    fn float_accessors_keep_the_sub_unit_part() {
        let bw = Bandwidth::from_kilo_bits_per_sec(1_500);
        assert_eq!(bw.as_mega_bits_per_sec_f64(), 1.5);
        assert_eq!(bw.as_giga_bits_per_sec_f64(), 0.0015);
    }

    #[test]
    fn zero_and_max() {
        assert!(Bandwidth::ZERO.is_zero());
        assert!(!Bandwidth::from_bits_per_sec(1).is_zero());
        assert_eq!(Bandwidth::default(), Bandwidth::ZERO);
        assert_eq!(Bandwidth::MAX.as_bits_per_sec(), u64::MAX);
        assert!(Bandwidth::MAX > Bandwidth::from_tera_bits_per_sec(u32::MAX / 1_000));
    }

    #[test]
    fn transmission_time_is_the_serialisation_delay() {
        let bw = Bandwidth::from_mega_bits_per_sec(1);
        assert_eq!(bw.transmission_time(1_000_000), Some(Duration::sec(1)));
        assert_eq!(bw.transmission_time(1_000), Some(Duration::ms(1)));
        assert_eq!(bw.transmission_time_bytes(125), Some(Duration::ms(1)));
        assert_eq!(bw.transmission_time(0), Some(Duration::ZERO));
        // 1 bit at 1Mbps is 1us, well inside pico second resolution.
        assert_eq!(bw.transmission_time(1), Some(Duration::ns(1_000)));
        assert_eq!(bw.unchecked_transmission_time(1_000_000), Duration::sec(1));
        assert_eq!(bw.unchecked_transmission_time_bytes(125), Duration::ms(1));
    }

    #[test]
    fn transmission_time_truncates_below_a_pico_second() {
        // 1 bit at 2Tbps is half a pico second.
        assert_eq!(
            Bandwidth::from_tera_bits_per_sec(2).transmission_time(1),
            Some(Duration::ZERO)
        );
    }

    #[test]
    fn transmission_time_reports_a_dead_link() {
        let bw = Bandwidth::from_kilo_bits_per_sec(1);
        assert_eq!(bw.transmission_time(1_000), Some(Duration::sec(1)));
        assert_eq!(Bandwidth::ZERO.transmission_time(1), None);
        assert_eq!(Bandwidth::ZERO.transmission_time(0), None);
        assert_eq!(Bandwidth::ZERO.transmission_time_bytes(1), None);
        // A byte count too wide to express in bits is the other None.
        assert_eq!(bw.transmission_time_bytes(u64::MAX), None);
    }

    #[test]
    #[should_panic(expected = "zero bandwidth")]
    fn unchecked_transmission_time_panics_on_a_dead_link() {
        Bandwidth::ZERO.unchecked_transmission_time(1);
    }

    #[test]
    fn volume_carried_over_a_span() {
        let bw = Bandwidth::from_mega_bits_per_sec(1);
        assert_eq!(bw.bits_in(Duration::sec(1)), Some(1_000_000));
        assert_eq!(bw.bits_in(Duration::ms(1)), Some(1_000));
        assert_eq!(bw.bytes_in(Duration::sec(1)), Some(125_000));
        assert_eq!(bw.bits_in(Duration::ZERO), Some(0));
        assert_eq!(Bandwidth::ZERO.bits_in(Duration::hour(1)), Some(0));
        assert_eq!(bw.unchecked_bits_in(Duration::sec(1)), 1_000_000);
        assert_eq!(bw.unchecked_bytes_in(Duration::sec(1)), 125_000);
        assert_eq!(Bandwidth::MAX.bits_in(Duration::MAX), None);
        assert_eq!(Bandwidth::MAX.bytes_in(Duration::MAX), None);
    }

    #[test]
    #[should_panic(expected = "overflow a u128")]
    fn unchecked_bits_in_panics_on_overflow() {
        Bandwidth::MAX.unchecked_bits_in(Duration::MAX);
    }

    /// A rate and a span round trip back to the volume that produced them.
    #[test]
    fn transmission_time_and_bits_in_are_inverses() {
        let bw = Bandwidth::from_giga_bits_per_sec(10);
        let bits = 12_000;
        assert_eq!(
            bw.bits_in(bw.unchecked_transmission_time(bits)),
            Some(bits as u128)
        );
    }

    #[test]
    fn arithmetic() {
        let mut bw = Bandwidth::from_mega_bits_per_sec(1);
        bw += Bandwidth::from_kilo_bits_per_sec(500);
        assert_eq!(bw, Bandwidth::from_bits_per_sec(1_500_000));
        bw -= Bandwidth::from_kilo_bits_per_sec(500);
        assert_eq!(bw, Bandwidth::from_mega_bits_per_sec(1));
        assert_eq!(
            Bandwidth::from_mega_bits_per_sec(3) - Bandwidth::from_mega_bits_per_sec(1),
            Bandwidth::from_mega_bits_per_sec(2)
        );
        assert_eq!(
            Bandwidth::from_mega_bits_per_sec(2) * 3,
            Bandwidth::from_mega_bits_per_sec(6)
        );
        assert_eq!(
            3 * Bandwidth::from_mega_bits_per_sec(2),
            Bandwidth::from_mega_bits_per_sec(6)
        );
        assert_eq!(
            Bandwidth::from_mega_bits_per_sec(6) / 4,
            Bandwidth::from_kilo_bits_per_sec(1_500)
        );
        assert_eq!(
            Bandwidth::from_mega_bits_per_sec(7) / Bandwidth::from_mega_bits_per_sec(2),
            3
        );
    }

    #[test]
    fn checked_and_saturating_edges() {
        let one = Bandwidth::from_bits_per_sec(1);
        assert_eq!(Bandwidth::MAX.checked_add(one), None);
        assert_eq!(Bandwidth::ZERO.checked_sub(one), None);
        assert_eq!(Bandwidth::MAX.checked_mul(2), None);
        assert_eq!(
            Bandwidth::from_bits_per_sec(2).checked_mul(3),
            Some(Bandwidth::from_bits_per_sec(6))
        );
        assert_eq!(Bandwidth::MAX.checked_div(0), None);
        assert_eq!(
            Bandwidth::from_bits_per_sec(6).checked_div(2),
            Some(Bandwidth::from_bits_per_sec(3))
        );
        assert_eq!(Bandwidth::MAX.saturating_add(one), Bandwidth::MAX);
        assert_eq!(Bandwidth::ZERO.saturating_sub(one), Bandwidth::ZERO);
        assert_eq!(Bandwidth::MAX.saturating_mul(2), Bandwidth::MAX);
    }

    #[test]
    fn scale_rounds_down() {
        assert_eq!(
            Bandwidth::from_mega_bits_per_sec(1).scale(0.5),
            Bandwidth::from_kilo_bits_per_sec(500)
        );
        assert_eq!(
            Bandwidth::from_bits_per_sec(3).scale(0.5),
            Bandwidth::from_bits_per_sec(1)
        );
        assert_eq!(
            Bandwidth::from_mega_bits_per_sec(1).scale(2.0),
            Bandwidth::from_mega_bits_per_sec(2)
        );
    }

    #[test]
    fn sums_owned_and_borrowed() {
        let bws = [
            Bandwidth::from_kilo_bits_per_sec(1),
            Bandwidth::from_bits_per_sec(500),
            Bandwidth::from_bits_per_sec(500),
        ];
        assert_eq!(
            bws.iter().sum::<Bandwidth>(),
            Bandwidth::from_kilo_bits_per_sec(2)
        );
        assert_eq!(
            bws.into_iter().sum::<Bandwidth>(),
            Bandwidth::from_kilo_bits_per_sec(2)
        );
        assert_eq!(
            std::iter::empty::<Bandwidth>().sum::<Bandwidth>(),
            Bandwidth::ZERO
        );
    }

    #[test]
    fn orders_by_rate() {
        let mut bws = [
            Bandwidth::from_giga_bits_per_sec(1),
            Bandwidth::ZERO,
            Bandwidth::from_kilo_bits_per_sec(1),
        ];
        bws.sort();
        assert_eq!(
            bws,
            [
                Bandwidth::ZERO,
                Bandwidth::from_kilo_bits_per_sec(1),
                Bandwidth::from_giga_bits_per_sec(1)
            ]
        );
    }

    #[test]
    fn display_picks_the_largest_fitting_unit() {
        assert_eq!(Bandwidth::ZERO.to_string(), "0bps");
        assert_eq!(Bandwidth::from_bits_per_sec(999).to_string(), "999bps");
        assert_eq!(
            Bandwidth::from_kilo_bits_per_sec(1).to_string(),
            "1.000kbps"
        );
        assert_eq!(
            Bandwidth::from_kilo_bits_per_sec(1_500).to_string(),
            "1.500Mbps"
        );
        assert_eq!(
            Bandwidth::from_giga_bits_per_sec(10).to_string(),
            "10.000Gbps"
        );
        assert_eq!(
            Bandwidth::from_tera_bits_per_sec(2).to_string(),
            "2.000Tbps"
        );
    }
}
