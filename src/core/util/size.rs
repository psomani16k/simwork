use std::fmt;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

use crate::core::util::bandwidth::Bandwidth;
use crate::core::util::duration::Duration;

/// Carries bits
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Size(u64);

impl Size {
    pub const ZERO: Size = Size(0);
    pub const MAX: Size = Size(u64::MAX);

    pub fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    pub fn from_kilo_bits(size: u32) -> Self {
        Self::from_bits(size as u64) * 1_000
    }

    pub fn from_mega_bits(size: u32) -> Self {
        Self::from_kilo_bits(size) * 1_000
    }

    pub fn from_giga_bits(size: u32) -> Self {
        Self::from_mega_bits(size) * 1_000
    }

    pub fn from_tera_bits(size: u32) -> Self {
        Self::from_giga_bits(size) * 1_000
    }

    pub fn from_bytes(size: u32) -> Self {
        Self::from_bits(size as u64) * 8
    }

    pub fn from_kilo_bytes(size: u32) -> Self {
        Self::from_bytes(size) * 1_000
    }

    pub fn from_mega_bytes(size: u32) -> Self {
        Self::from_kilo_bytes(size) * 1_000
    }

    pub fn from_giga_bytes(size: u32) -> Self {
        Self::from_mega_bytes(size) * 1_000
    }

    pub fn from_tera_bytes(size: u32) -> Self {
        Self::from_giga_bytes(size) * 1_000
    }

    pub fn from_kibi_bits(size: u32) -> Self {
        Self::from_bits(size as u64) * 1_024
    }

    pub fn from_mebi_bits(size: u32) -> Self {
        Self::from_kibi_bits(size) * 1_024
    }

    pub fn from_gibi_bits(size: u32) -> Self {
        Self::from_mebi_bits(size) * 1_024
    }

    pub fn from_tebi_bits(size: u32) -> Self {
        Self::from_gibi_bits(size) * 1_024
    }

    pub fn from_kibi_bytes(size: u32) -> Self {
        Self::from_bytes(size) * 1_024
    }

    pub fn from_mebi_bytes(size: u32) -> Self {
        Self::from_kibi_bytes(size) * 1_024
    }

    pub fn from_gibi_bytes(size: u32) -> Self {
        Self::from_mebi_bytes(size) * 1_024
    }

    pub fn from_tebi_bytes(size: u32) -> Self {
        Self::from_gibi_bytes(size) * 1_024
    }

    pub fn as_bits(&self) -> u64 {
        self.0
    }

    pub fn as_kilo_bits(&self) -> u64 {
        self.as_bits() / 1_000
    }

    pub fn as_mega_bits(&self) -> u64 {
        self.as_kilo_bits() / 1_000
    }

    pub fn as_giga_bits(&self) -> u64 {
        self.as_mega_bits() / 1_000
    }

    pub fn as_tera_bits(&self) -> u64 {
        self.as_giga_bits() / 1_000
    }

    /// Whole bytes, truncating any sub byte remainder.
    pub fn as_bytes(&self) -> u64 {
        self.as_bits() / 8
    }

    pub fn as_kilo_bytes(&self) -> u64 {
        self.as_bytes() / 1_000
    }

    pub fn as_mega_bytes(&self) -> u64 {
        self.as_kilo_bytes() / 1_000
    }

    pub fn as_giga_bytes(&self) -> u64 {
        self.as_mega_bytes() / 1_000
    }

    pub fn as_tera_bytes(&self) -> u64 {
        self.as_giga_bytes() / 1_000
    }

    pub fn as_kibi_bits(&self) -> u64 {
        self.as_bits() / 1_024
    }

    pub fn as_mebi_bits(&self) -> u64 {
        self.as_kibi_bits() / 1_024
    }

    pub fn as_gibi_bits(&self) -> u64 {
        self.as_mebi_bits() / 1_024
    }

    pub fn as_tebi_bits(&self) -> u64 {
        self.as_gibi_bits() / 1_024
    }

    pub fn as_kibi_bytes(&self) -> u64 {
        self.as_bytes() / 1_024
    }

    pub fn as_mebi_bytes(&self) -> u64 {
        self.as_kibi_bytes() / 1_024
    }

    pub fn as_gibi_bytes(&self) -> u64 {
        self.as_mebi_bytes() / 1_024
    }

    pub fn as_tebi_bytes(&self) -> u64 {
        self.as_gibi_bytes() / 1_024
    }

    /// Mega bits as a float, keeping the sub unit part.
    pub fn as_mega_bits_f64(&self) -> f64 {
        self.0 as f64 / 1_000_000.0
    }

    /// Giga bits as a float, keeping the sub unit part.
    pub fn as_giga_bits_f64(&self) -> f64 {
        self.0 as f64 / 1_000_000_000.0
    }

    /// Mebi bytes as a float, keeping the sub unit part.
    pub fn as_mebi_bytes_f64(&self) -> f64 {
        self.0 as f64 / 8_388_608.0
    }

    /// Gibi bytes as a float, keeping the sub unit part.
    pub fn as_gibi_bytes_f64(&self) -> f64 {
        self.0 as f64 / 8_589_934_592.0
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    /// Time needed to serialise this much data at `bandwidth`, truncating any
    /// sub pico second remainder. `None` at [`Bandwidth::ZERO`], where nothing
    /// ever finishes.
    pub fn transmission_time(&self, bandwidth: Bandwidth) -> Option<Duration> {
        bandwidth.transmission_time(self.0)
    }

    /// Same as [`Size::transmission_time`], panicking at [`Bandwidth::ZERO`]
    /// instead of reporting it.
    pub fn unchecked_transmission_time(&self, bandwidth: Bandwidth) -> Duration {
        bandwidth.unchecked_transmission_time(self.0)
    }

    /// Volume carried over `duration` at `bandwidth`, truncating any partial
    /// bit. `None` if the count is too wide to hold in a `u64`.
    pub fn carried_in(bandwidth: Bandwidth, duration: Duration) -> Option<Size> {
        bandwidth
            .bits_in(duration)
            .and_then(|bits| u64::try_from(bits).ok())
            .map(Size)
    }

    /// Same as [`Size::carried_in`], panicking instead of reporting a volume
    /// too wide to hold.
    pub fn unchecked_carried_in(bandwidth: Bandwidth, duration: Duration) -> Size {
        Size::carried_in(bandwidth, duration).expect("bits carried over the span overflow a u64")
    }

    pub fn checked_add(self, other: Size) -> Option<Size> {
        self.0.checked_add(other.0).map(Size)
    }

    pub fn checked_sub(self, other: Size) -> Option<Size> {
        self.0.checked_sub(other.0).map(Size)
    }

    pub fn checked_mul(self, factor: u64) -> Option<Size> {
        self.0.checked_mul(factor).map(Size)
    }

    pub fn checked_div(self, divisor: u64) -> Option<Size> {
        self.0.checked_div(divisor).map(Size)
    }

    pub fn saturating_add(self, other: Size) -> Size {
        Size(self.0.saturating_add(other.0))
    }

    /// Clamps at [`Size::ZERO`] instead of underflowing.
    pub fn saturating_sub(self, other: Size) -> Size {
        Size(self.0.saturating_sub(other.0))
    }

    pub fn saturating_mul(self, factor: u64) -> Size {
        Size(self.0.saturating_mul(factor))
    }

    /// Scales by an arbitrary factor, rounding down. Useful for a fractional
    /// share of a payload (e.g. `size.scale(0.5)` for half a frame). Loses
    /// precision above what an `f64` can hold exactly, so prefer
    /// [`Size::checked_mul`] and [`Div`] for exact ratios.
    pub fn scale(self, factor: f64) -> Size {
        Size((self.0 as f64 * factor) as u64)
    }
}

impl Add for Size {
    type Output = Size;

    fn add(self, other: Size) -> Size {
        Size(self.0 + other.0)
    }
}

impl AddAssign for Size {
    fn add_assign(&mut self, other: Size) {
        self.0 += other.0;
    }
}

impl Sub for Size {
    type Output = Size;

    fn sub(self, other: Size) -> Size {
        Size(self.0 - other.0)
    }
}

impl SubAssign for Size {
    fn sub_assign(&mut self, other: Size) {
        self.0 -= other.0;
    }
}

impl Mul<u64> for Size {
    type Output = Size;

    fn mul(self, factor: u64) -> Size {
        Size(self.0 * factor)
    }
}

impl Mul<Size> for u64 {
    type Output = Size;

    fn mul(self, size: Size) -> Size {
        Size(self * size.0)
    }
}

impl Div<u64> for Size {
    type Output = Size;

    fn div(self, divisor: u64) -> Size {
        Size(self.0 / divisor)
    }
}

/// Ratio of two sizes, truncating.
impl Div<Size> for Size {
    type Output = u64;

    fn div(self, other: Size) -> u64 {
        self.0 / other.0
    }
}

impl Sum for Size {
    fn sum<I: Iterator<Item = Size>>(iter: I) -> Size {
        iter.fold(Size::ZERO, |acc, size| acc + size)
    }
}

impl<'a> Sum<&'a Size> for Size {
    fn sum<I: Iterator<Item = &'a Size>>(iter: I) -> Size {
        iter.fold(Size::ZERO, |acc, size| acc + *size)
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bits = self.0;
        if bits < 1_000 {
            write!(f, "{}b", bits)
        } else if bits < 1_000_000 {
            write!(f, "{:.3}kb", bits as f64 / 1_000.0)
        } else if bits < 1_000_000_000 {
            write!(f, "{:.3}Mb", bits as f64 / 1_000_000.0)
        } else if bits < 1_000_000_000_000 {
            write!(f, "{:.3}Gb", bits as f64 / 1_000_000_000.0)
        } else {
            write!(f, "{:.3}Tb", bits as f64 / 1_000_000_000_000.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_unit_is_a_thousandfold_of_the_next() {
        assert_eq!(Size::from_kilo_bits(1), Size::from_bits(1_000));
        assert_eq!(Size::from_mega_bits(1), Size::from_kilo_bits(1_000));
        assert_eq!(Size::from_giga_bits(1), Size::from_mega_bits(1_000));
        assert_eq!(Size::from_tera_bits(1), Size::from_giga_bits(1_000));
        assert_eq!(Size::from_kilo_bytes(1), Size::from_bytes(1_000));
        assert_eq!(Size::from_mega_bytes(1), Size::from_kilo_bytes(1_000));
        assert_eq!(Size::from_giga_bytes(1), Size::from_mega_bytes(1_000));
        assert_eq!(Size::from_tera_bytes(1), Size::from_giga_bytes(1_000));
    }

    #[test]
    fn a_byte_is_eight_bits() {
        assert_eq!(Size::from_bytes(1), Size::from_bits(8));
        assert_eq!(Size::from_mega_bytes(1), Size::from_mega_bits(8));
        assert_eq!(Size::from_bits(15).as_bytes(), 1);
    }

    /// Each unit steps up the size rather than the `u32` argument, so values
    /// past `u32::MAX` bits survive the chain down to the base unit.
    #[test]
    fn wide_constructors_do_not_overflow_on_the_way_down() {
        assert_eq!(Size::from_tera_bits(5).as_bits(), 5_000_000_000_000);
        assert_eq!(
            Size::from_giga_bytes(1_000_000).as_bits(),
            8_000_000_000_000_000
        );
        assert_eq!(Size::from_tebi_bits(5).as_bits(), 5 * 1_099_511_627_776);
    }

    #[test]
    fn each_binary_unit_is_a_1024_fold_of_the_next() {
        assert_eq!(Size::from_kibi_bits(1), Size::from_bits(1_024));
        assert_eq!(Size::from_mebi_bits(1), Size::from_kibi_bits(1_024));
        assert_eq!(Size::from_gibi_bits(1), Size::from_mebi_bits(1_024));
        assert_eq!(Size::from_tebi_bits(1), Size::from_gibi_bits(1_024));
        assert_eq!(Size::from_kibi_bytes(1), Size::from_bytes(1_024));
        assert_eq!(Size::from_mebi_bytes(1), Size::from_kibi_bytes(1_024));
        assert_eq!(Size::from_gibi_bytes(1), Size::from_mebi_bytes(1_024));
        assert_eq!(Size::from_tebi_bytes(1), Size::from_gibi_bytes(1_024));
        assert_eq!(Size::from_gibi_bytes(1), Size::from_gibi_bits(8));
    }

    /// The binary units are the larger of the two ladders at every step, which
    /// is exactly why mixing them up misreports a payload.
    #[test]
    fn binary_units_run_ahead_of_the_decimal_ones() {
        assert_eq!(Size::from_kibi_bits(1).as_bits(), 1_024);
        assert_eq!(Size::from_gibi_bits(1).as_bits(), 1_073_741_824);
        assert!(Size::from_gibi_bits(1) > Size::from_giga_bits(1));
        // 1Gb is just under 1Gib, so the whole count truncates to zero.
        assert_eq!(Size::from_giga_bits(1).as_gibi_bits(), 0);
        assert_eq!(Size::from_giga_bits(2).as_gibi_bits(), 1);
    }

    #[test]
    fn accessors_truncate_the_remainder() {
        let size = Size::from_giga_bits(1) + Size::from_bits(1);
        assert_eq!(size.as_bits(), 1_000_000_001);
        assert_eq!(size.as_kilo_bits(), 1_000_000);
        assert_eq!(size.as_mega_bits(), 1_000);
        assert_eq!(size.as_giga_bits(), 1);
        assert_eq!(size.as_tera_bits(), 0);
        assert_eq!(size.as_bytes(), 125_000_000);
        assert_eq!(size.as_kilo_bytes(), 125_000);
        assert_eq!(size.as_mega_bytes(), 125);
        assert_eq!(size.as_giga_bytes(), 0);
        assert_eq!(size.as_tera_bytes(), 0);
    }

    #[test]
    fn binary_accessors_truncate_the_remainder() {
        let size = Size::from_gibi_bits(1) + Size::from_bits(1);
        assert_eq!(size.as_bits(), 1_073_741_825);
        assert_eq!(size.as_kibi_bits(), 1_048_576);
        assert_eq!(size.as_mebi_bits(), 1_024);
        assert_eq!(size.as_gibi_bits(), 1);
        assert_eq!(size.as_tebi_bits(), 0);
        assert_eq!(size.as_kibi_bytes(), 131_072);
        assert_eq!(size.as_mebi_bytes(), 128);
        assert_eq!(size.as_gibi_bytes(), 0);
        assert_eq!(size.as_tebi_bytes(), 0);
    }

    #[test]
    fn constructors_and_accessors_round_trip() {
        assert_eq!(Size::from_mebi_bits(7).as_mebi_bits(), 7);
        assert_eq!(Size::from_tebi_bytes(3).as_tebi_bytes(), 3);
        assert_eq!(Size::from_mega_bits(7).as_mega_bits(), 7);
        assert_eq!(Size::from_tera_bytes(3).as_tera_bytes(), 3);
    }

    #[test]
    fn float_accessors_keep_the_sub_unit_part() {
        assert_eq!(Size::from_kilo_bits(1_500).as_mega_bits_f64(), 1.5);
        assert_eq!(Size::from_kilo_bits(1_500).as_giga_bits_f64(), 0.0015);
        assert_eq!(Size::from_kibi_bytes(1_536).as_mebi_bytes_f64(), 1.5);
        assert_eq!(Size::from_mebi_bytes(512).as_gibi_bytes_f64(), 0.5);
    }

    #[test]
    fn zero_and_max() {
        assert!(Size::ZERO.is_zero());
        assert!(!Size::from_bits(1).is_zero());
        assert_eq!(Size::default(), Size::ZERO);
        assert_eq!(Size::MAX.as_bits(), u64::MAX);
        assert!(Size::MAX > Size::from_tera_bits(u32::MAX / 1_000));
    }

    #[test]
    fn transmission_time_is_the_serialisation_delay() {
        let bw = Bandwidth::from_mega_bits_per_sec(1);
        assert_eq!(
            Size::from_mega_bits(1).transmission_time(bw),
            Some(Duration::sec(1))
        );
        assert_eq!(
            Size::from_bytes(125).transmission_time(bw),
            Some(Duration::ms(1))
        );
        assert_eq!(Size::ZERO.transmission_time(bw), Some(Duration::ZERO));
        assert_eq!(
            Size::from_mega_bits(1).unchecked_transmission_time(bw),
            Duration::sec(1)
        );
    }

    #[test]
    fn transmission_time_reports_a_dead_link() {
        assert_eq!(Size::from_bits(1).transmission_time(Bandwidth::ZERO), None);
        assert_eq!(Size::ZERO.transmission_time(Bandwidth::ZERO), None);
    }

    #[test]
    #[should_panic(expected = "zero bandwidth")]
    fn unchecked_transmission_time_panics_on_a_dead_link() {
        Size::from_bits(1).unchecked_transmission_time(Bandwidth::ZERO);
    }

    #[test]
    fn carried_in_is_the_volume_over_a_span() {
        let bw = Bandwidth::from_mega_bits_per_sec(1);
        assert_eq!(
            Size::carried_in(bw, Duration::sec(1)),
            Some(Size::from_mega_bits(1))
        );
        assert_eq!(Size::carried_in(bw, Duration::ZERO), Some(Size::ZERO));
        assert_eq!(
            Size::carried_in(Bandwidth::ZERO, Duration::hour(1)),
            Some(Size::ZERO)
        );
        assert_eq!(
            Size::unchecked_carried_in(bw, Duration::ms(1)),
            Size::from_kilo_bits(1)
        );
        // Both the u128 bit count and the narrowing back to a u64 are Nones.
        assert_eq!(Size::carried_in(Bandwidth::MAX, Duration::MAX), None);
        assert_eq!(Size::carried_in(Bandwidth::MAX, Duration::hour(1)), None);
    }

    #[test]
    #[should_panic(expected = "overflow a u64")]
    fn unchecked_carried_in_panics_on_a_volume_too_wide() {
        Size::unchecked_carried_in(Bandwidth::MAX, Duration::hour(1));
    }

    /// A size and the rate that carried it round trip back to the same volume.
    #[test]
    fn transmission_time_and_carried_in_are_inverses() {
        let bw = Bandwidth::from_giga_bits_per_sec(10);
        let size = Size::from_kilo_bits(12);
        assert_eq!(
            Size::carried_in(bw, size.unchecked_transmission_time(bw)),
            Some(size)
        );
    }

    #[test]
    fn arithmetic() {
        let mut size = Size::from_mega_bits(1);
        size += Size::from_kilo_bits(500);
        assert_eq!(size, Size::from_bits(1_500_000));
        size -= Size::from_kilo_bits(500);
        assert_eq!(size, Size::from_mega_bits(1));
        assert_eq!(
            Size::from_mega_bits(3) - Size::from_mega_bits(1),
            Size::from_mega_bits(2)
        );
        assert_eq!(Size::from_mega_bits(2) * 3, Size::from_mega_bits(6));
        assert_eq!(3 * Size::from_mega_bits(2), Size::from_mega_bits(6));
        assert_eq!(Size::from_mega_bits(6) / 4, Size::from_kilo_bits(1_500));
        assert_eq!(Size::from_mega_bits(7) / Size::from_mega_bits(2), 3);
    }

    #[test]
    fn checked_and_saturating_edges() {
        let one = Size::from_bits(1);
        assert_eq!(Size::MAX.checked_add(one), None);
        assert_eq!(Size::ZERO.checked_sub(one), None);
        assert_eq!(Size::MAX.checked_mul(2), None);
        assert_eq!(Size::from_bits(2).checked_mul(3), Some(Size::from_bits(6)));
        assert_eq!(Size::MAX.checked_div(0), None);
        assert_eq!(Size::from_bits(6).checked_div(2), Some(Size::from_bits(3)));
        assert_eq!(Size::MAX.saturating_add(one), Size::MAX);
        assert_eq!(Size::ZERO.saturating_sub(one), Size::ZERO);
        assert_eq!(Size::MAX.saturating_mul(2), Size::MAX);
    }

    #[test]
    fn scale_rounds_down() {
        assert_eq!(
            Size::from_mega_bits(1).scale(0.5),
            Size::from_kilo_bits(500)
        );
        assert_eq!(Size::from_bits(3).scale(0.5), Size::from_bits(1));
        assert_eq!(Size::from_mega_bits(1).scale(2.0), Size::from_mega_bits(2));
    }

    #[test]
    fn sums_owned_and_borrowed() {
        let sizes = [
            Size::from_kilo_bits(1),
            Size::from_bits(500),
            Size::from_bits(500),
        ];
        assert_eq!(sizes.iter().sum::<Size>(), Size::from_kilo_bits(2));
        assert_eq!(sizes.into_iter().sum::<Size>(), Size::from_kilo_bits(2));
        assert_eq!(std::iter::empty::<Size>().sum::<Size>(), Size::ZERO);
    }

    #[test]
    fn orders_by_volume() {
        let mut sizes = [Size::from_giga_bits(1), Size::ZERO, Size::from_kilo_bits(1)];
        sizes.sort();
        assert_eq!(
            sizes,
            [Size::ZERO, Size::from_kilo_bits(1), Size::from_giga_bits(1)]
        );
    }

    #[test]
    fn display_picks_the_largest_fitting_unit() {
        assert_eq!(Size::ZERO.to_string(), "0b");
        assert_eq!(Size::from_bits(999).to_string(), "999b");
        assert_eq!(Size::from_kilo_bits(1).to_string(), "1.000kb");
        assert_eq!(Size::from_kilo_bits(1_500).to_string(), "1.500Mb");
        assert_eq!(Size::from_giga_bits(10).to_string(), "10.000Gb");
        assert_eq!(Size::from_tera_bits(2).to_string(), "2.000Tb");
    }
}

pub trait SizeOf {
    fn size_in_bytes(&self) -> Size;
}
