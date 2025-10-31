#[derive(Debug)]
pub struct HighScores {
	list:Vec<u32>,
}

impl HighScores {
	pub fn new( scores: &[u32]) -> Self {
		HighScores {
			list: scores.to_vec(),
		}
	}

	pub fn scores(&self) -> &[u32] {
		&self.list
	}

	pub fn latest(&self) -> Option<u32> {
		self.list.last().copied()
	}

	pub fn personal_best(&self) -> Option<u32> {
		self.list.clone()
			.into_iter()
			.max()
	}

	pub fn personal_top_three(&self) -> Vec<u32> {
		let mut top_three = self.list.clone();
		// top_three.sort_by(|a, b| b.cmp(a));
		top_three.sort();
		top_three.reverse();
		top_three.truncate(3);
		top_three
	}
}
