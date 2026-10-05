#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let row_count = input.lines().count();
    if input.lines().count() % 4 != 0 {
        return Err(Error::InvalidRowCount(row_count));
    }
    let column_count = input.lines().find_map(|line| {
        let column_count = line.len();
        (column_count % 3 != 0).then_some(column_count)
    });
    if let Some(columns_not_multiple_of_three) = column_count {
        return Err(Error::InvalidColumnCount(columns_not_multiple_of_three));
    }
    let column_count = input.lines().next().map(|s| s.len()).unwrap_or_default();
    let digits = digits();
    let mut result = String::new();
    for row_cell in 0..row_count / 4 {
        if row_cell != 0 {
            result.push(',');
        }
        for column_cell in 0..column_count / 3 {
            let cell = input
                .lines()
                .skip(row_cell * 4)
                .take(4)
                .map(|line| {
                    line.chars()
                        .skip(column_cell * 3)
                        .take(3)
                        .collect::<String>()
                })
                .collect::<Vec<_>>()
                .join("\n");
            let digit = digits
                .iter()
                .enumerate()
                .find_map(|(i, d)| {
                    (*d == cell).then(|| char::from_digit(i as u32, 10).unwrap_or('?'))
                })
                .unwrap_or('?');
            result.push(digit);
        }
    }
    Ok(result)
}

fn digits() -> [String; 10] {
    let zero = format!("{}\n{}\n{}\n{}", " _ ", "| |", "|_|", "   ");
    let one = format!("{}\n{}\n{}\n{}", "   ", "  |", "  |", "   ");
    let two = format!("{}\n{}\n{}\n{}", " _ ", " _|", "|_ ", "   ");
    let three = format!("{}\n{}\n{}\n{}", " _ ", " _|", " _|", "   ");
    let four = format!("{}\n{}\n{}\n{}", "   ", "|_|", "  |", "   ");
    let five = format!("{}\n{}\n{}\n{}", " _ ", "|_ ", " _|", "   ");
    let six = format!("{}\n{}\n{}\n{}", " _ ", "|_ ", "|_|", "   ");
    let seven = format!("{}\n{}\n{}\n{}", " _ ", "  |", "  |", "   ");
    let eight = format!("{}\n{}\n{}\n{}", " _ ", "|_|", "|_|", "   ");
    let nine = format!("{}\n{}\n{}\n{}", " _ ", "|_|", " _|", "   ");
    [zero, one, two, three, four, five, six, seven, eight, nine]
}
