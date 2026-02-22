pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut res = Vec::with_capacity(self.row_count as usize);
        for row in 0..self.row_count {
            if row == 0 {
                res.push(vec![1]);
                continue;
            }
            let prev_vec = &res[(row - 1) as usize];
            let mut current_vec = Vec::with_capacity((row + 1) as usize);
            current_vec.push(1);
            for window in prev_vec.windows(2) {
                current_vec.push(window[0] + window[1]);
            }
            current_vec.push(1);
            res.push(current_vec);
        }
        res
    }
}
