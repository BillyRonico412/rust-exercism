enum Direction {
    Top,
    Bottom,
}

struct RailIterator {
    rails: usize,
    row: usize,
    col: usize,
    direction: Direction,
}

impl RailIterator {
    fn new(rails: usize) -> Self {
        Self {
            rails,
            row: 0,
            col: 0,
            direction: Direction::Bottom,
        }
    }
}

impl Iterator for RailIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let result = (self.row, self.col);

        if self.row == 0 {
            self.direction = Direction::Bottom;
        } else if self.row == self.rails - 1 {
            self.direction = Direction::Top;
        }

        match self.direction {
            Direction::Bottom => self.row += 1,
            Direction::Top => self.row -= 1,
        };

        self.col += 1;

        Some(result)
    }
}

pub struct RailFence {
    rails: usize,
}

impl RailFence {
    pub fn new(rails: u32) -> Self {
        Self {
            rails: rails as usize,
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut rail_vec = vec![vec!['.'; text.len()]; self.rails];
        let mut rail_iter = RailIterator::new(self.rails);

        text.chars().for_each(|c| {
            let (row, col) = rail_iter.next().unwrap();
            rail_vec[row][col] = c;
        });

        rail_vec.iter().flatten().filter(|&&c| c != '.').collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut rail_vec = vec![vec!['.'; cipher.len()]; self.rails];
        let rail_iter = RailIterator::new(self.rails);

        rail_iter.take(cipher.len()).for_each(|(row, col)| {
            rail_vec[row][col] = '?';
        });

        let mut chars = cipher.chars();

        rail_vec.iter_mut().for_each(|rail| {
            rail.iter_mut().for_each(|c| {
                if *c == '?' {
                    *c = chars.next().unwrap();
                }
            });
        });

        let rail_iter = RailIterator::new(self.rails);

        rail_iter
            .take(cipher.len())
            .map(|(row, col)| rail_vec[row][col])
            .collect()
    }
}
