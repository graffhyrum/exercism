pub fn is_armstrong_number(num: u32) -> bool {
	let mut digit_count: u32 = 0;
	let mut digits: Vec<u32> = Vec::new();
	for digit in DigitIterator::new(num) {
		digit_count += 1;
		digits.push(digit);
	}
	let mut sum: u32 = 0;
	for digit in digits {
		sum = sum.checked_add(digit.pow(digit_count)).unwrap_or(0);
	}
	sum == num
}

struct DigitIterator<T> {
	value: T,
}

trait Integer {
	fn abs(self) -> Self;
}

impl<T> DigitIterator<T>
	where
		T: Integer,
{
	pub fn new(value: T) -> Self {
		Self { value: value.abs() }
	}
}

impl Integer for u32 {
	#[inline(always)]
	fn abs(self) -> u32 {
		self
	}
}
impl Iterator for DigitIterator<u32> {
	type Item = u32;

	fn next(&mut self) -> Option<Self::Item> {
		if self.value == 0 {
			None
		} else {
			let digit = self.value % 10;
			self.value /= 10;
			Some(digit)
		}
	}
}
