pub fn annotate(garden: &[&str]) -> Vec<String> {
    let grid: Vec<_> = garden.iter().map(|&line| line.as_bytes()).collect();
    let height = grid.len();
    let width = if height > 0 { grid[0].len() } else { 0 };

    // Create a mutable output grid to store counts
    let mut output: Vec<Vec<u8>> = grid.iter().map(|row| row.to_vec()).collect();

    for (y, row) in grid.iter().enumerate() {
        for (x, &cellbyte) in row.iter().enumerate() {
            // if a flower ('*'), increments all adjacent cells
            match cellbyte {
                b'*' => {
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dy == 0 && dx == 0 {
                                continue;
                            }
                            let ny = y as isize + dy;
                            let nx = x as isize + dx;
                            if ny >= 0 && ny < height as isize && nx >= 0 && nx < width as isize {
                                let ny = ny as usize;
                                let nx = nx as usize;
                                // increment cell at (ny, nx)
                                if output[ny][nx] != b'*' {
                                    if output[ny][nx] == b' ' {
                                        output[ny][nx] = b'1';
                                    } else {
                                        output[ny][nx] += 1;
                                    }
                                }
                            }
                        }
                    }
                }
                _ => { /* empty cell, do nothing */ }
            }
        }
    }
    output
        .iter()
        .map(|row| String::from_utf8(row.to_vec()).unwrap())
        .collect()
}
