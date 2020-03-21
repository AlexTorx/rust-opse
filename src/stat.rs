#[derive(Debug, PartialEq)]
struct ValueRange {
	start: f64,
	end: f64,
}

impl ValueRange {
	fn new (start: f64, end: f64) -> ValueRange {

		if start > end {
			panic!("ValueRange : start value ({}) should not be greater than end value ({}).", start, end);
		}

		if start != start.floor() {
			panic!("ValueRange : start value should be a 0-decimal f64 number. Found {}", start);
		}

		if end != end.floor() {
			panic!("ValueRange : end value should be a 0-decimal f64 number. Found {}", end);
		}

		ValueRange { start: start, end: end }
	}

	fn size (&self) -> f64 {
		// This function is aimed at returning the number of values
		// in the current ValueRange object
		self.end - self.start + 1.0
	}

	fn contains (&self, number: &f64) -> bool {
		self.start <= *number && *number <= self.end
	}
}

#[cfg(test)]
mod tests {

	mod test_ValueRange {

		use super::super::ValueRange;

		fn create_value_range (start: f64, end: f64) -> ValueRange {
			ValueRange::new(start, end)
		}

		#[test]
		fn test_print_debug () {
			let range: ValueRange = create_value_range(0.0_f64, 100.0_f64);
			assert_eq!(format!("{:?}", range), "ValueRange { start: 0.0, end: 100.0 }");
		}

		#[test]
		fn test_equal () {
			let range_1: ValueRange = create_value_range(0.0_f64, 100.0_f64);
			let range_2: ValueRange = create_value_range(0.0_f64, 100.0_f64);
			assert_eq!(range_1, range_2);

			let range_3: ValueRange = create_value_range(1.0_f64, 100.0_f64);
			assert!(range_1 != range_3);
		}

		#[test]
		fn test_size () {
			let range: ValueRange = create_value_range(0.0_f64, 100.0_f64);
			assert_eq!(range.size(), 101.0);

			let range: ValueRange = create_value_range(100.0_f64, 100.0_f64);
			assert_eq!(range.size(), 1.0);
		}

		#[test]
		fn test_contains () {
			let range: ValueRange = create_value_range(0.0_f64, 100.0_f64);

			assert_eq!(range.contains(&0.0_f64), true);
			assert_eq!(range.contains(&100.0_f64), true);
			assert_eq!(range.contains(&50.0_f64), true);
			assert_eq!(range.contains(&101.0_f64), false);
			assert_eq!(range.contains(&-1.0_f64), false);
		}
	}
}