const COORDS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
pub fn annotate(garden: &[&str]) -> Vec<String> {
    garden
        .iter()
        .enumerate()
        .map(|(main_x, main_row)| {
            main_row
                .as_bytes()
                .iter()
                .enumerate()
                .filter_map(|(main_y, item)| {
                    if *item == b'*' {
                        return Some('*');
                    }
                    let nb_flower = COORDS.iter().fold(0, |acc, (dx, dy)| {
                        let check_x = (main_x as i32 + *dx) as usize;
                        if let Some(row) = garden.get(check_x) {
                            let check_y = (main_y as i32 + *dy) as usize;
                            if row.as_bytes().get(check_y) == Some(&b'*') {
                                return acc + 1;
                            }
                        }
                        acc
                    });
                    if nb_flower == 0 {
                        return Some(' ');
                    }
                    Some(char::from_digit(nb_flower, 10).unwrap())
                })
                .collect()
        })
        .collect()
}
