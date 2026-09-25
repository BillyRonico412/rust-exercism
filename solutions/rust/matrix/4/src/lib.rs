pub struct Matrix {
    input: String,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        Matrix {
            input: input.to_string(),
        }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        self.input
            .lines()
            .map(|row| row.split_whitespace().map(|n| n.parse().unwrap()).collect())
            .nth(row_no - 1)
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        self.input
            .lines()
            .map(|row| {
                row.split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .nth(col_no - 1)
            })
            .collect()
    }
}
