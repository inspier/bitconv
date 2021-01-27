use core::{convert::TryInto, mem};
use endian::*;

pub mod endian {
    pub enum Endian {
        LE,
        BE,
        NE,
    }

    pub struct Little;
    pub struct Big;
    pub struct Native;
}

pub trait BitConvEndian {
    fn as_endian() -> Endian { Endian::NE }
}

impl BitConvEndian for Little {
    fn as_endian() -> Endian { Endian::LE }
}

impl BitConvEndian for Big {
    fn as_endian() -> Endian { Endian::BE }
}

macro_rules! BitConvImpl {
    ($type:ty, $generic:ty, $data:tt, $start:tt, $error_message:expr) => {{
        let f = match <$generic>::as_endian() {
            Endian::LE => <$type>::from_le_bytes,
            Endian::BE => <$type>::from_be_bytes,
            Endian::NE => <$type>::from_ne_bytes,
        };
        $data
            .get($start..)
            .and_then(|bytes| bytes.get(..mem::size_of::<$type>()))
            .map(|bytes| f(bytes.try_into().unwrap()))
            .expect($error_message)
    }};
}

// Not elegant compared to using format! and stringify!,
// but this way inlines correctly.
const ERROR_MESSAGES: [&str; 6] = [
    "Failed to read i16. Invalid buffer provided.",
    "Failed to read i32. Invalid buffer provided.",
    "Failed to read i64. Invalid buffer provided.",
    "Failed to read u16. Invalid buffer provided.",
    "Failed to read u32. Invalid buffer provided.",
    "Failed to read u64. Invalid buffer provided.",
];

/// Returns a 16-bit signed integer converted from two bytes at a specified
/// position in a byte array.
///
/// The `to_int16` function converts the bytes from index start_index to
/// start_index + 1 to a `i16` value.
/// # Example
///
/// ```
/// use bitconv::{
///     endian::{Big, Little}, to_int16
/// };
///
/// let buffer = [15, 0, 0, 255, 3, 16, 39, 255, 255, 127];
/// assert_eq!(-256, to_int16::<Little>(&buffer, 2));
/// assert_eq!(255, to_int16::<Big>(&buffer, 2));
/// ```
#[inline]
pub fn to_int16<T: BitConvEndian>(data: &[u8], start_index: usize) -> i16 {
    BitConvImpl!(i16, T, data, start_index, ERROR_MESSAGES[0])
}

/// Returns a 32-bit signed integer converted from four bytes at a specified
/// position in a byte array.
///
/// The `to_int32` function converts the bytes from index start_index to
/// start_index + 3 to a `i32` value.
/// # Example
///
/// ```
/// use bitconv::{
///     endian::{Big, Little}, to_int32
/// };
///
/// let buffer = [15, 0, 0, 128, 16, 39, 240, 216, 241, 255, 127];
/// assert_eq!(-265875328, to_int32::<Little>(&buffer, 3));
/// assert_eq!(-2146424848, to_int32::<Big>(&buffer, 3));
/// ```
#[inline]
pub fn to_int32<T: BitConvEndian>(data: &[u8], start_index: usize) -> i32 {
    BitConvImpl!(i32, T, data, start_index, ERROR_MESSAGES[1])
}

/// Returns a 64-bit signed integer converted from eight bytes at a specified
/// position in a byte array.
///
/// The `to_int64` function converts the bytes from index start_index to
/// start_index + 7 to a `i64` value.
/// # Example
///
/// ```
/// use bitconv::{
///     endian::{Big, Little}, to_int64
/// };
///
/// let buffer = [15, 0, 0, 128, 16, 39, 240, 216, 241, 255, 127];
/// assert_eq!(-1019801265028202496, to_int64::<Little>(&buffer, 1));
/// assert_eq!(140806877927665, to_int64::<Big>(&buffer, 1));
/// ```
#[inline]
pub fn to_int64<T: BitConvEndian>(data: &[u8], start_index: usize) -> i64 {
    BitConvImpl!(i64, T, data, start_index, ERROR_MESSAGES[2])
}

/// Returns a 16-bit unsigned integer converted from two bytes at a specified
/// position in a byte array.
///
/// The `to_uint16` function converts the bytes from index start_index to
/// start_index + 1 to a `u16` value.
/// # Example
///
/// ```
/// use bitconv::{
///     endian::{Big, Little}, to_uint16
/// };
///
/// let buffer = [15, 0, 0, 255, 3, 16, 39, 255, 255, 127];
/// assert_eq!(65280, to_uint16::<Little>(&buffer, 2));
/// assert_eq!(255, to_uint16::<Big>(&buffer, 2));
/// ```
#[inline]
pub fn to_uint16<T: BitConvEndian>(data: &[u8], start_index: usize) -> u16 {
    BitConvImpl!(u16, T, data, start_index, ERROR_MESSAGES[3])
}

/// Returns a 32-bit unsigned integer converted from four bytes at a specified
/// position in a byte array.
///
/// The `to_uint32` function converts the bytes from index start_index to
/// start_index + 3 to a `u32` value.
/// # Example
///
/// ```
/// use bitconv::{
///     endian::{Big, Little}, to_uint32
/// };
///
/// let buffer = [15, 0, 0, 0, 0, 16, 0, 255, 3, 0, 0, 202, 19];
/// assert_eq!(261888, to_uint32::<Little>(&buffer, 6));
/// assert_eq!(16712448, to_uint32::<Big>(&buffer, 6));
/// ```
#[inline]
pub fn to_uint32<T: BitConvEndian>(data: &[u8], start_index: usize) -> u32 {
    BitConvImpl!(u32, T, data, start_index, ERROR_MESSAGES[4])
}

/// Returns a 64-bit unsigned integer converted from eight bytes at a specified
/// position in a byte array.
///
/// The `to_uint64` function converts the bytes from index start_index to
/// start_index + 7 to a `u64` value.
/// # Example
///
/// ```
/// use bitconv::{
///     endian::{Big, Little}, to_uint64
/// };
///
/// let buffer = [255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 170, 170, 170, 170, 170];
/// assert_eq!(255, to_uint64::<Little>(&buffer, 2));
/// assert_eq!(18374686479671623680, to_uint64::<Big>(&buffer, 2));
/// ```
#[inline]
pub fn to_uint64<T: BitConvEndian>(data: &[u8], start_index: usize) -> u64 {
    BitConvImpl!(u64, T, data, start_index, ERROR_MESSAGES[5])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_int16_test_le() {
        let buffer = [15, 0, 0, 128, 16, 39, 240, 216, 241, 255, 127];
        assert_eq!(15, to_int16::<Little>(&buffer, 0));
        assert_eq!(0, to_int16::<Little>(&buffer, 1));
        assert_eq!(-32768, to_int16::<Little>(&buffer, 2));
        assert_eq!(10000, to_int16::<Little>(&buffer, 4));
        assert_eq!(-10000, to_int16::<Little>(&buffer, 6));
        assert_eq!(-15, to_int16::<Little>(&buffer, 8));
        assert_eq!(32767, to_int16::<Little>(&buffer, 9));
    }

    #[test]
    #[should_panic]
    fn to_int16_le_panic_test() {
        let buffer = [15, 0, 0, 128, 16, 39, 240, 216, 241, 255, 127];
        to_uint16::<Little>(&buffer, 11);
    }

    #[test]
    fn to_int32_test_le() {
        let buffer = [15, 0, 0, 0, 0, 16, 0, 255, 3, 0, 0, 202, 154, 59, 255, 255, 255, 255, 127];
        assert_eq!(15, to_int32::<Little>(&buffer, 0));
        assert_eq!(268435456, to_int32::<Little>(&buffer, 2));
        assert_eq!(-16773120, to_int32::<Little>(&buffer, 4));
        assert_eq!(67043344, to_int32::<Little>(&buffer, 5));
        assert_eq!(-905969661, to_int32::<Little>(&buffer, 8));
        assert_eq!(-12870966, to_int32::<Little>(&buffer, 11));
        assert_eq!(-50278, to_int32::<Little>(&buffer, 12));
    }

    #[test]
    #[should_panic]
    fn to_int32_le_panic_test() {
        let buffer = [15, 0, 0, 0, 0, 16, 0, 255, 3, 0, 0, 202, 154, 59, 255, 255, 255, 255, 127];
        to_int32::<Little>(&buffer, 16);
    }

    #[test]
    fn to_int64_test_le() {
        let buffer = [
            0, 54, 101, 196, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 202, 154, 59, 0,
            0, 0, 0, 1, 0, 0, 0, 0, 255, 255, 255, 255, 1, 0, 0, 255, 255, 255, 255, 255, 255, 255,
            127, 86, 85, 85, 85, 85, 85, 255, 255, 170, 170, 170, 170, 170, 170, 0, 0, 100, 167,
            179, 182, 224, 13, 0, 0, 156, 88, 76, 73, 31, 242,
        ];
        assert_eq!(-1000000000, to_int64::<Little>(&buffer, 0));
        assert_eq!(16777215, to_int64::<Little>(&buffer, 5));
        assert_eq!(0, to_int64::<Little>(&buffer, 8));
        assert_eq!(-9223372036854775808, to_int64::<Little>(&buffer, 9));
        assert_eq!(1000000000, to_int64::<Little>(&buffer, 17));
        assert_eq!(4294967296, to_int64::<Little>(&buffer, 21));
        assert_eq!(-4294967296, to_int64::<Little>(&buffer, 26));
        assert_eq!(-16777215, to_int64::<Little>(&buffer, 34));
        assert_eq!(-187649984473770, to_int64::<Little>(&buffer, 45));
        assert_eq!(187649984473770, to_int64::<Little>(&buffer, 53));
        assert_eq!(1000000000000000000, to_int64::<Little>(&buffer, 59));
    }

    #[test]
    #[should_panic]
    fn to_int64_le_panic_test() {
        let buffer = [
            0, 54, 101, 196, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 202, 154, 59, 0,
            0, 0, 0, 1, 0, 0, 0, 0, 255, 255, 255, 255, 1, 0, 0, 255, 255, 255, 255, 255, 255, 255,
            127, 86, 85, 85, 85, 85, 85, 255, 255, 170, 170, 170, 170, 170, 170, 0, 0, 100, 167,
            179, 182, 224, 13, 0, 0, 156, 88, 76, 73, 31, 242,
        ];
        to_int64::<Little>(&buffer, 68);
    }

    #[test]
    fn to_uint16_test_le() {
        let buffer = [15, 0, 0, 255, 3, 16, 39, 255, 255, 127];
        assert_eq!(15, to_uint16::<Little>(&buffer, 0));
        assert_eq!(0, to_uint16::<Little>(&buffer, 1));
        assert_eq!(1023, to_uint16::<Little>(&buffer, 3));
        assert_eq!(10000, to_uint16::<Little>(&buffer, 5));
        assert_eq!(32767, to_uint16::<Little>(&buffer, 8));
        assert_eq!(65535, to_uint16::<Little>(&buffer, 7));
    }

    #[test]
    #[should_panic]
    fn to_uint16_le_panic_test() {
        let buffer = [15, 0, 0, 255, 3, 16, 39, 255, 255, 127];
        to_uint16::<Little>(&buffer, 9);
    }

    #[test]
    fn to_uint32_test_le() {
        let buffer = [15, 0, 0, 0, 0, 16, 0, 255, 3, 0, 0, 202, 154, 59, 255, 255, 255, 255, 127];
        assert_eq!(15, to_uint32::<Little>(&buffer, 0));
        assert_eq!(0, to_uint32::<Little>(&buffer, 1));
        assert_eq!(1048576, to_uint32::<Little>(&buffer, 3));
        assert_eq!(1023, to_uint32::<Little>(&buffer, 7));
        assert_eq!(1000000000, to_uint32::<Little>(&buffer, 10));
        assert_eq!(4294967295, to_uint32::<Little>(&buffer, 14));
        assert_eq!(2147483647, to_uint32::<Little>(&buffer, 15));
    }

    #[test]
    #[should_panic]
    fn to_uint32_le_panic_test() {
        let buffer = [15, 0, 0, 0, 0, 16, 0, 255, 3, 0, 0, 202, 154, 59, 255, 255, 255, 255, 127];
        to_uint32::<Little>(&buffer, 16);
    }

    #[test]
    fn to_uint64_test_le() {
        let buffer = [
            255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 100, 167, 179, 182, 224, 13, 0, 202,
            154, 59, 0, 0, 0, 0, 170, 170, 170, 170, 170, 170, 0, 0, 232, 137, 4, 35, 199, 138,
            255, 255, 255, 255, 255, 255, 255, 255, 127,
        ];
        assert_eq!(16777215, to_uint64::<Little>(&buffer, 0));
        assert_eq!(0, to_uint64::<Little>(&buffer, 3));
        assert_eq!(4294967296, to_uint64::<Little>(&buffer, 7));
        assert_eq!(1000000000000000000, to_uint64::<Little>(&buffer, 13));
        assert_eq!(1000000000, to_uint64::<Little>(&buffer, 21));
        assert_eq!(187649984473770, to_uint64::<Little>(&buffer, 29));
        assert_eq!(10000000000000000000, to_uint64::<Little>(&buffer, 35));
        assert_eq!(18446744073709551615, to_uint64::<Little>(&buffer, 43));
        assert_eq!(9223372036854775807, to_uint64::<Little>(&buffer, 44));
    }

    #[test]
    #[should_panic]
    fn to_uint64_le_panic_test() {
        let buffer = [
            255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 100, 167, 179, 182, 224, 13, 0, 202,
            154, 59, 0, 0, 0, 0, 170, 170, 170, 170, 170, 170, 0, 0, 232, 137, 4, 35, 199, 138,
            255, 255, 255, 255, 255, 255, 255, 255, 127,
        ];
        to_uint64::<Little>(&buffer, 45);
    }
    #[test]
    fn to_int16_test_be() {
        let buffer = [15, 0, 0, 128, 16, 39, 240, 216, 241, 255, 127];
        assert_eq!(3840, to_int16::<Big>(&buffer, 0));
        assert_eq!(0, to_int16::<Big>(&buffer, 1));
        assert_eq!(128, to_int16::<Big>(&buffer, 2));
        assert_eq!(4135, to_int16::<Big>(&buffer, 4));
        assert_eq!(-3880, to_int16::<Big>(&buffer, 6));
        assert_eq!(-3585, to_int16::<Big>(&buffer, 8));
        assert_eq!(-129, to_int16::<Big>(&buffer, 9));
    }

    #[test]
    #[should_panic]
    fn to_int16_be_panic_test() {
        let buffer = [15, 0, 0, 128, 16, 39, 240, 216, 241, 255, 127];
        to_uint16::<Big>(&buffer, 11);
    }

    #[test]
    fn to_int32_test_be() {
        let buffer = [15, 0, 0, 0, 0, 16, 0, 255, 3, 0, 0, 202, 154, 59, 255, 255, 255, 255, 127];
        assert_eq!(251658240, to_int32::<Big>(&buffer, 0));
        assert_eq!(16, to_int32::<Big>(&buffer, 2));
        assert_eq!(1048831, to_int32::<Big>(&buffer, 4));
        assert_eq!(268500739, to_int32::<Big>(&buffer, 5));
        assert_eq!(50331850, to_int32::<Big>(&buffer, 8));
        assert_eq!(-895861761, to_int32::<Big>(&buffer, 11));
        assert_eq!(-1707343873, to_int32::<Big>(&buffer, 12));
    }

    #[test]
    #[should_panic]
    fn to_int32_be_panic_test() {
        let buffer = [15, 0, 0, 0, 0, 16, 0, 255, 3, 0, 0, 202, 154, 59, 255, 255, 255, 255, 127];
        to_int32::<Big>(&buffer, 16);
    }

    #[test]
    fn to_int64_test_be() {
        let buffer = [
            0, 54, 101, 196, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 202, 154, 59, 0,
            0, 0, 0, 1, 0, 0, 0, 0, 255, 255, 255, 255, 1, 0, 0, 255, 255, 255, 255, 255, 255, 255,
            127, 86, 85, 85, 85, 85, 85, 255, 255, 170, 170, 170, 170, 170, 170, 0, 0, 100, 167,
            179, 182, 224, 13, 0, 0, 156, 88, 76, 73, 31, 242,
        ];
        assert_eq!(15311545525338111, to_int64::<Big>(&buffer, 0));
        assert_eq!(-1099511627776, to_int64::<Big>(&buffer, 5));
        assert_eq!(0, to_int64::<Big>(&buffer, 8));
        assert_eq!(128, to_int64::<Big>(&buffer, 9));
        assert_eq!(57027523489300480, to_int64::<Big>(&buffer, 17));
        assert_eq!(16777216, to_int64::<Big>(&buffer, 21));
        assert_eq!(4294967295, to_int64::<Big>(&buffer, 26));
        assert_eq!(72058693549555711, to_int64::<Big>(&buffer, 34));
        assert_eq!(6220972285274488831, to_int64::<Big>(&buffer, 45));
        assert_eq!(-6148914691236560896, to_int64::<Big>(&buffer, 53));
        assert_eq!(110671437422605, to_int64::<Big>(&buffer, 59));
    }

    #[test]
    #[should_panic]
    fn to_int64_be_panic_test() {
        let buffer = [
            0, 54, 101, 196, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 202, 154, 59, 0,
            0, 0, 0, 1, 0, 0, 0, 0, 255, 255, 255, 255, 1, 0, 0, 255, 255, 255, 255, 255, 255, 255,
            127, 86, 85, 85, 85, 85, 85, 255, 255, 170, 170, 170, 170, 170, 170, 0, 0, 100, 167,
            179, 182, 224, 13, 0, 0, 156, 88, 76, 73, 31, 242,
        ];
        to_int64::<Big>(&buffer, 68);
    }

    #[test]
    fn to_uint16_test_be() {
        let buffer = [15, 0, 0, 255, 3, 16, 39, 255, 255, 127];
        assert_eq!(3840, to_uint16::<Big>(&buffer, 0));
        assert_eq!(0, to_uint16::<Big>(&buffer, 1));
        assert_eq!(65283, to_uint16::<Big>(&buffer, 3));
        assert_eq!(4135, to_uint16::<Big>(&buffer, 5));
        assert_eq!(65407, to_uint16::<Big>(&buffer, 8));
        assert_eq!(65535, to_uint16::<Big>(&buffer, 7));
    }

    #[test]
    #[should_panic]
    fn to_uint16_be_panic_test() {
        let buffer = [15, 0, 0, 255, 3, 16, 39, 255, 255, 127];
        to_uint16::<Big>(&buffer, 9);
    }

    #[test]
    fn to_uint32_test_be() {
        let buffer = [15, 0, 0, 0, 0, 16, 0, 255, 3, 0, 0, 202, 154, 59, 255, 255, 255, 255, 127];
        assert_eq!(251658240, to_uint32::<Big>(&buffer, 0));
        assert_eq!(0, to_uint32::<Big>(&buffer, 1));
        assert_eq!(4096, to_uint32::<Big>(&buffer, 3));
        assert_eq!(4278386688, to_uint32::<Big>(&buffer, 7));
        assert_eq!(13277755, to_uint32::<Big>(&buffer, 10));
        assert_eq!(4294967295, to_uint32::<Big>(&buffer, 14));
        assert_eq!(4294967167, to_uint32::<Big>(&buffer, 15));
    }

    #[test]
    #[should_panic]
    fn to_uint32_be_panic_test() {
        let buffer = [15, 0, 0, 0, 0, 16, 0, 255, 3, 0, 0, 202, 154, 59, 255, 255, 255, 255, 127];
        to_uint32::<Big>(&buffer, 16);
    }

    #[test]
    fn to_uint64_test_be() {
        let buffer = [
            255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 100, 167, 179, 182, 224, 13, 0, 202,
            154, 59, 0, 0, 0, 0, 170, 170, 170, 170, 170, 170, 0, 0, 232, 137, 4, 35, 199, 138,
            255, 255, 255, 255, 255, 255, 255, 255, 127,
        ];
        assert_eq!(18446742974197923840, to_uint64::<Big>(&buffer, 0));
        assert_eq!(0, to_uint64::<Big>(&buffer, 3));
        assert_eq!(16777216, to_uint64::<Big>(&buffer, 7));
        assert_eq!(110671437422605, to_uint64::<Big>(&buffer, 13));
        assert_eq!(57027523489300480, to_uint64::<Big>(&buffer, 21));
        assert_eq!(12297829382472990720, to_uint64::<Big>(&buffer, 29));
        assert_eq!(255675177617290, to_uint64::<Big>(&buffer, 35));
        assert_eq!(18446744073709551615, to_uint64::<Big>(&buffer, 43));
        assert_eq!(18446744073709551487, to_uint64::<Big>(&buffer, 44));
    }

    #[test]
    #[should_panic]
    fn to_uint64_be_panic_test() {
        let buffer = [
            255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 100, 167, 179, 182, 224, 13, 0, 202,
            154, 59, 0, 0, 0, 0, 170, 170, 170, 170, 170, 170, 0, 0, 232, 137, 4, 35, 199, 138,
            255, 255, 255, 255, 255, 255, 255, 255, 127,
        ];
        to_uint64::<Big>(&buffer, 45);
    }
}
