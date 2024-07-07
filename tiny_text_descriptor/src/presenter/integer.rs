use super::signed_integer::SignedInteger;
use super::unsigned_integer::UnsignedInteger;

pub enum Integer {
	Signed(SignedInteger),
	Unsigned(UnsignedInteger),
}

impl Integer {
	pub fn is_negative(&self) -> bool {
		todo!()
	}

	pub fn thousand_separator(&self) -> String {
		todo!()
	}
}

impl From<i8> for Integer {
	fn from(value: i8) -> Self {
		todo!()
	}
}

impl From<i16> for Integer {
	fn from(value: i16) -> Self {
		todo!()
	}
}

impl From<i32> for Integer {
	fn from(value: i32) -> Self {
		todo!()
	}
}

impl From<i64> for Integer {
	fn from(value: i64) -> Self {
		todo!()
	}
}

impl From<i128> for Integer {
	fn from(value: i128) -> Self {
		todo!()
	}
}

impl From<isize> for Integer {
	fn from(value: isize) -> Self {
		todo!()
	}
}

impl From<u8> for Integer {
	fn from(value: u8) -> Self {
		todo!()
	}
}

impl From<u16> for Integer {
	fn from(value: u16) -> Self {
		todo!()
	}
}

impl From<u32> for Integer {
	fn from(value: u32) -> Self {
		todo!()
	}
}

impl From<u64> for Integer {
	fn from(value: u64) -> Self {
		todo!()
	}
}

impl From<u128> for Integer {
	fn from(value: u128) -> Self {
		todo!()
	}
}

impl From<usize> for Integer {
	fn from(value: usize) -> Self {
		todo!()
	}
}
