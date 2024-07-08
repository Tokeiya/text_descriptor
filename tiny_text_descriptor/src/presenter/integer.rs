use std::collections::VecDeque;
use super::contents_presenter::ContentsPresenter;

pub enum Integer {
	I8(i8),
	I16(i16),
	I32(i32),
	I64(i64),
	I128(i128),
	ISize(isize),
	U8(u8),
	U16(u16),
	U32(u32),
	U64(u64),
	U128(u128),
	USize(usize)
}


impl Integer {
	pub fn is_negative(&self) -> bool {
		match self {
			Integer::I8(x) => x.is_negative(),
			Integer::I16(x) => x.is_negative(),
			Integer::I32(x) => x.is_negative(),
			Integer::I64(x) => x.is_negative(),
			Integer::I128(x) => x.is_negative(),
			Integer::ISize(x) => x.is_negative(),
			Integer::U8(_) => false,
			Integer::U16(_) => false,
			Integer::U32(_) => false,
			Integer::U64(_) => false,
			Integer::U128(_) => false,
			Integer::USize(_) => false
		}
	}

	fn get_text(&self)->String{
		match self {
			Integer::I8(x) => x.abs().to_string(),
			Integer::I16(x) => x.abs().to_string(),
			Integer::I32(x) => x.abs().to_string(),
			Integer::I64(x) => x.abs().to_string(),
			Integer::I128(x) => x.abs().to_string(),
			Integer::ISize(x) => x.abs().to_string(),
			Integer::U8(x) => x.to_string(),
			Integer::U16(x) => x.to_string(),
			Integer::U32(x) => x.to_string(),
			Integer::U64(x) => x.to_string(),
			Integer::U128(x) => x.to_string(),
			Integer::USize(x) => x.to_string()
		}
	}

	pub fn thousand_separator(&self,separator:&str) -> String {
		let mut buff=VecDeque::<char>::new();
		let str=self.get_text();
		for (idx, char) in str.chars().rev().enumerate() {
			if idx%3==0 && idx!=0{
				for elem in separator.chars() {
					buff.push_front(elem)
				}
			}
			buff.push_front(char);
		}

		if self.is_negative(){
			buff.push_front('-');
		}

		buff.iter().collect()

	}
}

impl From<i8> for Integer {
	fn from(value: i8) -> Self {
		Integer::I8(value)
	}
}

impl From<i16> for Integer {
	fn from(value: i16) -> Self {
		Integer::I16(value)
	}
}

impl From<i32> for Integer {
	fn from(value: i32) -> Self {
		Integer::I32(value)
	}
}

impl From<i64> for Integer {
	fn from(value: i64) -> Self {
		Integer::I64(value)
	}
}

impl From<i128> for Integer {
	fn from(value: i128) -> Self {
		Integer::I128(value)
	}
}

impl From<isize> for Integer {
	fn from(value: isize) -> Self {
		Integer::ISize(value)
	}
}

impl From<u8> for Integer {
	fn from(value: u8) -> Self {
		Integer::U8(value)
	}
}

impl From<u16> for Integer {
	fn from(value: u16) -> Self {
		Integer::U16(value)
	}
}

impl From<u32> for Integer {
	fn from(value: u32) -> Self {
		Integer::U32(value)
	}
}

impl From<u64> for Integer {
	fn from(value: u64) -> Self {
		Integer::U64(value)
	}
}

impl From<u128> for Integer {
	fn from(value: u128) -> Self {
		Integer::U128(value)
	}
}

impl From<usize> for Integer {
	fn from(value: usize) -> Self {
		Integer::USize(value)
	}
}

#[cfg(test)]
pub mod test_helper {
	use super::*;

	impl Integer {
		pub fn assert_sign(&self, expected: bool) {
			assert_eq!(self.is_negative(), expected)
		}

		pub fn assert_thousand_separator(&self, expected: &str) {
			assert_eq!(self.thousand_separator(","), expected)
		}

		pub fn eq_i8(&self, expected: i8) {
			assert!(matches!(self, Integer::I8(value) if value == &expected))
		}

		pub fn eq_i16(&self, expected: i16) {
			assert!(matches!(self, Integer::I16(value) if value == &expected))
		}

		pub fn eq_i32(&self, expected: i32) {
			assert!(matches!(self, Integer::I32(value) if value == &expected))
		}

		pub fn eq_i64(&self, expected: i64) {
			assert!(matches!(self, Integer::I64(value) if value == &expected))
		}

		pub fn eq_i128(&self, expected: i128) {
			assert!(matches!(self, Integer::I128(value) if value == &expected))
		}

		pub fn eq_isize(&self, expected: isize) {
			assert!(matches!(self, Integer::ISize(value) if value == &expected))
		}

		pub fn eq_u8(&self, expected: u8) {
			assert!(matches!(self, Integer::U8(value) if value == &expected))
		}

		pub fn eq_u16(&self, expected: u16) {
			assert!(matches!(self, Integer::U16(value) if value == &expected))
		}

		pub fn eq_u32(&self, expected: u32) {
			assert!(matches!(self, Integer::U32(value) if value == &expected))
		}

		pub fn eq_u64(&self, expected: u64) {
			assert!(matches!(self, Integer::U64(value) if value == &expected))
		}

		pub fn eq_u128(&self, expected: u128) {
			assert!(matches!(self, Integer::U128(value) if value == &expected))
		}

		pub fn eq_usize(&self, expected: usize) {
			assert!(matches!(self, Integer::USize(value) if value == &expected))
		}
	}

	#[test]
	fn eq_test() {
		let i8 = Integer::I8(1);
		i8.eq_i8(1);
		let i16 = Integer::I16(1);
		i16.eq_i16(1);
		let i32 = Integer::I32(1);
		i32.eq_i32(1);
		let i64 = Integer::I64(1);
		i64.eq_i64(1);
		let i128 = Integer::I128(1);
		i128.eq_i128(1);
		let isize = Integer::ISize(1);
		isize.eq_isize(1);
		let u8 = Integer::U8(1);
		u8.eq_u8(1);
		let u16 = Integer::U16(1);
		u16.eq_u16(1);
		let u32 = Integer::U32(1);
		u32.eq_u32(1);
		let u64 = Integer::U64(1);
		u64.eq_u64(1);
		let u128 = Integer::U128(1);
		u128.eq_u128(1);
		let usize = Integer::USize(1);
		usize.eq_usize(1);
	}

	#[test]
	#[should_panic]
	fn not_eq_i8_test() {
		let i8 = Integer::I8(1);
		i8.eq_i8(2);
	}

	#[test]
	#[should_panic]
	fn type_mismatch_i8_test() {
		let i8 = Integer::I8(1);
		i8.eq_i16(1);
	}

	#[test]
	#[should_panic]
	fn not_eq_i16_test() {
		let i16 = Integer::I16(1);
		i16.eq_i16(2);
	}

	#[test]
	#[should_panic]
	fn type_mismatch_i16_test() {
		let i16 = Integer::I16(1);
		i16.eq_i32(1);
	}

	#[test]
	#[should_panic]
	fn not_eq_i32_test() {
		let i32 = Integer::I32(1);
		i32.eq_i32(2);
	}

	#[test]
	#[should_panic]
	fn type_mismatch_i32_test() {
		let i32 = Integer::I32(1);
		i32.eq_i64(1);
	}

	#[test]
	#[should_panic]
	fn not_eq_i64_test() {
		let i64 = Integer::I64(1);
		i64.eq_i64(2);
	}

	#[test]
	#[should_panic]
	fn type_mismatch_i64_test() {
		let i64 = Integer::I64(1);
		i64.eq_i128(1);
	}

	#[test]
	#[should_panic]
	fn not_eq_i128_test() {
		let i128 = Integer::I128(1);
		i128.eq_i128(2);
	}

	#[test]
	#[should_panic]
	fn type_mismatch_i128_test() {
		let i128 = Integer::I128(1);
		i128.eq_isize(1);
	}

	#[test]
	#[should_panic]
	fn not_eq_isize_test() {
		let isize = Integer::ISize(1);
		isize.eq_isize(2);
	}

	#[test]
	#[should_panic]
	fn type_mismatch_isize_test() {
		let isize = Integer::ISize(1);
		isize.eq_u8(1);
	}

	#[test]
	#[should_panic]
	fn not_eq_u8_test() {
		let u8 = Integer::U8(1);
		u8.eq_u8(2);
	}

	#[test]
	#[should_panic]
	fn type_mismatch_u8_test() {
		let u8 = Integer::U8(1);
		u8.eq_u16(1);
	}

	#[test]
	#[should_panic]
	fn not_eq_u16_test() {
		let u16 = Integer::U16(1);
		u16.eq_u16(2);
	}

	#[test]
	#[should_panic]
	fn type_mismatch_u16_test() {
		let u16 = Integer::U16(1);
		u16.eq_u32(1);
	}

	#[test]
	#[should_panic]
	fn not_eq_u32_test() {
		let u32 = Integer::U32(1);
		u32.eq_u32(2);
	}

	#[test]
	#[should_panic]
	fn type_mismatch_u32_test() {
		let u32 = Integer::U32(1);
		u32.eq_u64(1);
	}

	#[test]
	#[should_panic]
	fn not_eq_u64_test() {
		let u64 = Integer::U64(1);
		u64.eq_u64(2);
	}

	#[test]
	#[should_panic]
	fn type_mismatch_u64_test() {
		let u64 = Integer::U64(1);
		u64.eq_u128(1);
	}

	#[test]
	#[should_panic]
	fn not_eq_u128_test() {
		let u128 = Integer::U128(1);
		u128.eq_u128(2);
	}

	#[test]
	#[should_panic]
	fn type_mismatch_u128_test() {
		let u128 = Integer::U128(1);
		u128.eq_usize(1);
	}

	#[test]
	#[should_panic]
	fn not_eq_usize_test() {
		let usize = Integer::USize(1);
		usize.eq_usize(2);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn thousand_separator_test() {
		let integer = Integer::I8(100);
		integer.assert_thousand_separator("100");

		let integer = Integer::I8(-100);
		integer.assert_thousand_separator("-100");

		let integer = Integer::I16(1_000);
		integer.assert_thousand_separator("1,000");

		let integer = Integer::I16(-1_000);
		integer.assert_thousand_separator("-1,000");

		let integer = Integer::I32(1_000_000);
		integer.assert_thousand_separator("1,000,000");

		let integer = Integer::I32(-1_000_000);
		integer.assert_thousand_separator("-1,000,000");

		let integer = Integer::I64(1_000_000_000);
		integer.assert_thousand_separator("1,000,000,000");

		let integer = Integer::I64(-1_000_000_000);
		integer.assert_thousand_separator("-1,000,000,000");

		let integer = Integer::I128(1_000_000_000_000);
		integer.assert_thousand_separator("1,000,000,000,000");

		let integer = Integer::I128(-1_000_000_000_000);
		integer.assert_thousand_separator("-1,000,000,000,000");

		let integer = Integer::ISize(1_000);
		integer.assert_thousand_separator("1,000");

		let integer = Integer::ISize(-1_000);
		integer.assert_thousand_separator("-1,000");

		let integer = Integer::U8(100);
		integer.assert_thousand_separator("100");

		let integer = Integer::U16(1_000);
		integer.assert_thousand_separator("1,000");

		let integer = Integer::U32(1_000_000);
		integer.assert_thousand_separator("1,000,000");

		let integer = Integer::U64(1_000_000_000);
		integer.assert_thousand_separator("1,000,000,000");

		let integer = Integer::U128(1_000_000_000_000);
		integer.assert_thousand_separator("1,000,000,000,000");

		let integer = Integer::USize(1_000);
		integer.assert_thousand_separator("1,000");
	}

	#[test]
	fn is_negative_test() {
		let positive = Integer::I8(1);
		positive.assert_sign(false);
		let negative = Integer::I8(-1);
		negative.assert_sign(true);
	}

	#[test]
	fn from_i8_test() {
		let value = 1i8;
		let integer = Integer::from(value);
		integer.eq_i8(value);
	}

	#[test]
	#[should_panic]
	fn from_i8_not_eq_test() {
		let value = 1i8;
		let integer = Integer::from(value);
		integer.eq_i8(2);
	}

	#[test]
	#[should_panic]
	fn from_i8_type_mismatch_test() {
		let value = 1i16;
		let integer = Integer::from(value);
		integer.eq_i8(1);
	}

	#[test]
	fn from_i16_test() {
		let value = 1i16;
		let integer = Integer::from(value);
		integer.eq_i16(value);
	}

	#[test]
	#[should_panic]
	fn from_i16_not_eq_test() {
		let value = 1i16;
		let integer = Integer::from(value);
		integer.eq_i16(2);
	}

	#[test]
	#[should_panic]
	fn from_i16_type_mismatch_test() {
		let value = 1i32;
		let integer = Integer::from(value);
		integer.eq_i16(1);
	}

	#[test]
	fn from_i32_test() {
		let value = 1i32;
		let integer = Integer::from(value);
		integer.eq_i32(value);
	}

	#[test]
	#[should_panic]
	fn from_i32_not_eq_test() {
		let value = 1i32;
		let integer = Integer::from(value);
		integer.eq_i32(2);
	}

	#[test]
	#[should_panic]
	fn from_i32_type_mismatch_test() {
		let value = 1i64;
		let integer = Integer::from(value);
		integer.eq_i32(1);
	}

	#[test]
	fn from_i64_test() {
		let value = 1i64;
		let integer = Integer::from(value);
		integer.eq_i64(value);
	}

	#[test]
	#[should_panic]
	fn from_i64_not_eq_test() {
		let value = 1i64;
		let integer = Integer::from(value);
		integer.eq_i64(2);
	}

	#[test]
	#[should_panic]
	fn from_i64_type_mismatch_test() {
		let value = 1i128;
		let integer = Integer::from(value);
		integer.eq_i64(1);
	}

	#[test]
	fn from_i128_test() {
		let value = 1i128;
		let integer = Integer::from(value);
		integer.eq_i128(value);
	}

	#[test]
	#[should_panic]
	fn from_i128_not_eq_test() {
		let value = 1i128;
		let integer = Integer::from(value);
		integer.eq_i128(2);
	}

	#[test]
	#[should_panic]
	fn from_i128_type_mismatch_test() {
		let value = 1isize;
		let integer = Integer::from(value);
		integer.eq_i128(1);
	}

	#[test]
	fn from_isize_test() {
		let value = 1isize;
		let integer = Integer::from(value);
		integer.eq_isize(value);
	}

	#[test]
	#[should_panic]
	fn from_isize_not_eq_test() {
		let value = 1isize;
		let integer = Integer::from(value);
		integer.eq_isize(2);
	}

	#[test]
	#[should_panic]
	fn from_isize_type_mismatch_test() {
		let value = 1i8;
		let integer = Integer::from(value);
		integer.eq_isize(1);
	}

	#[test]
	fn from_u8_test() {
		let value = 1u8;
		let integer = Integer::from(value);
		integer.eq_u8(value);
	}

	#[test]
	#[should_panic]
	fn from_u8_not_eq_test() {
		let value = 1u8;
		let integer = Integer::from(value);
		integer.eq_u8(2);
	}

	#[test]
	#[should_panic]
	fn from_u8_type_mismatch_test() {
		let value = 1u16;
		let integer = Integer::from(value);
		integer.eq_u8(1);
	}

	#[test]
	fn from_u16_test() {
		let value = 1u16;
		let integer = Integer::from(value);
		integer.eq_u16(value);
	}

	#[test]
	#[should_panic]
	fn from_u16_not_eq_test() {
		let value = 1u16;
		let integer = Integer::from(value);
		integer.eq_u16(2);
	}

	#[test]
	#[should_panic]
	fn from_u16_type_mismatch_test() {
		let value = 1u32;
		let integer = Integer::from(value);
		integer.eq_u16(1);
	}

	#[test]
	fn from_u32_test() {
		let value = 1u32;
		let integer = Integer::from(value);
		integer.eq_u32(value);
	}

	#[test]
	#[should_panic]
	fn from_u32_not_eq_test() {
		let value = 1u32;
		let integer = Integer::from(value);
		integer.eq_u32(2);
	}

	#[test]
	#[should_panic]
	fn from_u32_type_mismatch_test() {
		let value = 1u64;
		let integer = Integer::from(value);
		integer.eq_u32(1);
	}

	#[test]
	fn from_u64_test() {
		let value = 1u64;
		let integer = Integer::from(value);
		integer.eq_u64(value);
	}

	#[test]
	#[should_panic]
	fn from_u64_not_eq_test() {
		let value = 1u64;
		let integer = Integer::from(value);
		integer.eq_u64(2);
	}

	#[test]
	#[should_panic]
	fn from_u64_type_mismatch_test() {
		let value = 1u128;
		let integer = Integer::from(value);
		integer.eq_u64(1);
	}

	#[test]
	fn from_u128_test() {
		let value = 1u128;
		let integer = Integer::from(value);
		integer.eq_u128(value);
	}

	#[test]
	#[should_panic]
	fn from_u128_not_eq_test() {
		let value = 1u128;
		let integer = Integer::from(value);
		integer.eq_u128(2);
	}

	#[test]
	#[should_panic]
	fn from_u128_type_mismatch_test() {
		let value = 1usize;
		let integer = Integer::from(value);
		integer.eq_u128(1);
	}

	#[test]
	fn from_usize_test() {
		let value = 1usize;
		let integer = Integer::from(value);
		integer.eq_usize(value);
	}

	#[test]
	#[should_panic]
	fn from_usize_not_eq_test() {
		let value = 1usize;
		let integer = Integer::from(value);
		integer.eq_usize(2);
	}

	#[test]
	#[should_panic]
	fn from_usize_type_mismatch_test() {
		let value = 1u8;
		let integer = Integer::from(value);
		integer.eq_usize(1);
	}
}