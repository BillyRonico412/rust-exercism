const COORDS: &[(i8, i8); 8] = &[
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
    let mut res = Vec::new();
    for (main_x, main_row) in garden.iter().enumerate() {
        let mut row_res = String::from("");
        for (main_y, item) in main_row.as_bytes().iter().enumerate() {
            if *item == b'*' {
                row_res.push('*');
                continue;
            }
            let mut nb_flower = 0;
            for (dx, dy) in COORDS.iter() {
                let check_x = (main_x as i32 + *dx as i32) as usize;
                match garden.get(check_x) {
                    None => continue,
                    Some(row) => {
                        let check_y = (main_y as i32 + *dy as i32) as usize;
                        match row.as_bytes().get(check_y) {
                            Some(it) if *it == b'*' => {
                                nb_flower += 1;
                            }
                            _ => continue,
                        }
                    }
                }
            }
            if nb_flower == 0 {
                row_res.push(' ');
                continue;
            }
            row_res.push(char::from_digit(nb_flower, 10).unwrap());
        }
        res.push(row_res);
    }
    res
}
