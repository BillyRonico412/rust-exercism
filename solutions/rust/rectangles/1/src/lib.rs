#[derive(Clone, PartialEq, Eq)]
struct Coord {
    row: usize,
    col: usize,
}

enum Direction {
    Right,
    Bottom,
    Left,
    Top,
}

struct Grid(Vec<Vec<char>>);

impl Grid {
    fn new(lines: &[&str]) -> Self {
        Self(lines.iter().map(|line| line.chars().collect()).collect())
    }

    fn nb_rows(&self) -> usize {
        self.0.len()
    }

    fn nb_cols(&self) -> usize {
        self.0.get(0).map(|row| row.len()).unwrap_or_default()
    }

    fn get_by_coord(&self, coord: &Coord) -> char {
        self.0[coord.row][coord.col]
    }

    fn get_intersection_follow_direction(
        &self,
        start: &Coord,
        coord: &Coord,
        direction: &Direction,
    ) -> Vec<Coord> {
        let mut intersections = vec![];
        let mut current = coord.clone();
        loop {
            match direction {
                Direction::Right => {
                    if current.col == self.nb_cols() - 1 {
                        break;
                    }
                    current = Coord {
                        row: current.row,
                        col: current.col + 1,
                    };
                    let c = self.get_by_coord(&current);
                    if c != '+' && c != '-' {
                        break;
                    }
                }
                Direction::Left => {
                    if current.col == start.col {
                        break;
                    }
                    current = Coord {
                        row: current.row,
                        col: current.col - 1,
                    };
                    let c = self.get_by_coord(&current);
                    if c != '+' && c != '-' {
                        break;
                    }
                }
                Direction::Bottom => {
                    if current.row == self.nb_rows() - 1 {
                        break;
                    }
                    current = Coord {
                        row: current.row + 1,
                        col: current.col,
                    };
                    let c = self.get_by_coord(&current);
                    if c != '+' && c != '|' {
                        break;
                    }
                }
                Direction::Top => {
                    if current.row == start.row {
                        break;
                    }
                    current = Coord {
                        row: current.row - 1,
                        col: current.col,
                    };
                    let c = self.get_by_coord(&current);
                    if c != '+' && c != '|' {
                        break;
                    }
                }
            }
            let c = self.get_by_coord(&current);
            if c != '+' {
                continue;
            }
            intersections.push(current.clone());
        }
        intersections
    }
}

pub fn count(lines: &[&str]) -> u32 {
    let grid = Grid::new(lines);
    let mut rectangle_count = 0;
    for row in 0..grid.nb_rows() {
        for col in 0..grid.nb_cols() {
            let start = Coord { row, col };
            if grid.get_by_coord(&start) != '+' {
                continue;
            }
            for right in grid.get_intersection_follow_direction(&start, &start, &Direction::Right) {
                for bottom in
                    grid.get_intersection_follow_direction(&start, &right, &Direction::Bottom)
                {
                    for left in
                        grid.get_intersection_follow_direction(&start, &bottom, &Direction::Left)
                    {
                        for top in
                            grid.get_intersection_follow_direction(&start, &left, &Direction::Top)
                        {
                            if start == top {
                                rectangle_count += 1
                            }
                        }
                    }
                }
            }
        }
    }
    rectangle_count
}
