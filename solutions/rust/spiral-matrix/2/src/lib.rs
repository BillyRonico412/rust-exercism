use std::vec;

enum Direction {
    Right,
    Bottom,
    Left,
    Top,
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut v = vec![vec![0; size as usize]; size as usize];

    if size == 0 {
        return v;
    }

    let mut direction = Direction::Right;
    let mut x = 0usize;
    let mut y = 0usize;
    let mut nb = 1;
    let size_usize = size as usize;

    loop {
        v[y][x] = nb;

        if nb == size.pow(2) {
            break v;
        }

        match direction {
            Direction::Right => {
                if x == size_usize - 1 || v[y][x + 1] != 0 {
                    direction = Direction::Bottom;
                } else {
                    x += 1;
                    nb += 1;
                }
            }

            Direction::Bottom => {
                if y == size_usize - 1 || v[y + 1][x] != 0 {
                    direction = Direction::Left;
                } else {
                    y += 1;
                    nb += 1;
                }
            }

            Direction::Left => {
                if x == 0 || v[y][x - 1] != 0 {
                    direction = Direction::Top;
                } else {
                    x -= 1;
                    nb += 1;
                }
            }

            Direction::Top => {
                if y == 0 || v[y - 1][x] != 0 {
                    direction = Direction::Right;
                } else {
                    y -= 1;
                    nb += 1;
                }
            }
        }
    }
}
