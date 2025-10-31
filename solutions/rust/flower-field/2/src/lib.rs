pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() || garden[0].is_empty() {
        return garden.iter().map(|&s| s.to_string()).collect();
    }

    let annotator = GardenAnnotator::new(garden);
    annotator.annotate_all()
}

struct GardenAnnotator<'a> {
    garden: &'a [&'a str],
    max_row: usize,
    max_col: usize,
}

impl<'a> GardenAnnotator<'a> {
    fn new(garden: &'a [&'a str]) -> Self {
        Self {
            garden,
            max_row: garden.len().saturating_sub(1),
            max_col: garden[0].len().saturating_sub(1),
        }
    }

    fn annotate_all(&self) -> Vec<String> {
        (0..self.garden.len())
            .map(|row| self.annotate_row(row))
            .collect()
    }

    fn annotate_row(&self, row: usize) -> String {
        (0..self.garden[0].len())
            .map(|col| self.annotate_cell(row, col))
            .collect()
    }

    fn annotate_cell(&self, row: usize, col: usize) -> char {
        if self.is_flower(row, col) {
            return '*';
        }

        match self.count_adjacent_flowers(row, col) {
            0 => ' ',
            n => (b'0' + n as u8) as char,
        }
    }

    #[inline]
    fn is_flower(&self, row: usize, col: usize) -> bool {
        self.garden[row].as_bytes()[col] == b'*'
    }

    fn count_adjacent_flowers(&self, row: usize, col: usize) -> usize {
        let row_range = self.get_range(row, self.max_row);
        let col_range = self.get_range(col, self.max_col);

        row_range
            .flat_map(|r| col_range.clone().map(move |c| (r, c)))
            .filter(|&(r, c)| !(r == row && c == col) && self.is_flower(r, c))
            .count()
    }

    #[inline]
    fn get_range(&self, center: usize, max: usize) -> std::ops::RangeInclusive<usize> {
        center.saturating_sub(1)..=(center + 1).min(max)
    }
}
