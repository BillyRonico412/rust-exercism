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
            .split('\n')
            .map(|row| row.split(' ').map(|n| n.parse::<u32>().unwrap()).collect())
            .nth(row_no - 1)
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        self.input
            .split('\n')
            .map(|row| {
                row.split(' ')
                    .map(|n| n.parse::<u32>().unwrap())
                    .nth(col_no - 1)
            })
            .collect()
    }
}
