pub fn annotate(minefield: &[&str]) -> Vec<String> {
	let row_count = minefield.len();
	if row_count == 0 {
		return vec![];
	}
	let col_count = minefield[0].len();
	if col_count == 0 {
		return vec![String::new()];
	}
	// 2d u8 vector, 0 is space, 9 is a mine, 1-8 is number of mines
	let mut two_d_array: Vec<Vec<u8>> = vec![vec![0; col_count]; row_count];

	for (row_index, row) in minefield.iter().enumerate() {
		row.as_bytes().iter().enumerate().for_each(|(col_index, cell)| {
			if *cell == b'*' {
				two_d_array[row_index][col_index] = 9;
				// increment all surrounding cells
				for i in -1..=1 {
					for j in -1..=1 {
						if i == 0 && j == 0 {
							// ...unless it's the origin
							continue;
						}
						let row = row_index as i32 + i;
						let col = col_index as i32 + j;
						if row >= 0 && row < row_count as i32 && col >= 0 && col < col_count as i32 {
							let cell_val = two_d_array[row as usize][col as usize];
							match cell_val {
								9 => continue, // don't increment mines
								_ => two_d_array[row as usize][col as usize] += 1,
							}
						}
					}
				}
			}
		});
	};

	two_d_array.iter().map(|row| {
		row.iter().map(|cell| {
			// convert back to char
			match cell {
				9 => '*',
				0 => ' ',
				_ => (cell + 48) as char, // 48 is ascii for 0, adding the cell value gives the ascii value for the number
			}
		}).collect()
	}).collect()
}
