#[derive(Debug, Copy, Clone)]
pub struct Page {
	pub current: i32,
	pub total: i32,
}
impl Page {
	pub fn new(current: i32, total: i32) -> Self {
		Self {
			current: if current < 1 { 1 } else { current },
			total: total,
		}
	}

	pub fn calc_total(&mut self, item_count: i32, limit: i32) -> i32 {
		let mut t: i32 = item_count / limit;
		if item_count % limit != 0 {
			t += 1;
		}
		self.total = t;
		t
	}

	pub fn range(self, limit: i32) -> (i32, i32) {
		((self.current - 1) * limit, self.current * limit)
	}
}
impl Default for Page {
	fn default() -> Self {
		Page::new(1, 1)
	}
}
