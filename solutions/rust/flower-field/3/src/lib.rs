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
                .map(|(main_y, item)| {
                    if *item == b'*' {
                        return '*';
                    }
                    let nb_flower = COORDS
                        .iter()
                        .filter(|(dx, dy)| {
                            let check_x = (main_x as i32 + *dx) as usize;
                            let check_y = (main_y as i32 + *dy) as usize;
                            garden
                                .get(check_x)
                                .and_then(|row| row.as_bytes().get(check_y))
                                == Some(&b'*')
                        })
                        .count();
                    if nb_flower == 0 {
                        return ' ';
                    }
                    char::from_digit(nb_flower as u32, 10).unwrap()
                })
                .collect()
        })
        .collect()
}
