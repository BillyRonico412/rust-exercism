pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let student_index = student.as_bytes().first();
    let student_index = match student_index {
        None => return Vec::new(),
        Some(student_index) => student_index - b'A',
    };
    let student_index = (student_index * 2) as usize;
    diagram
        .split('\n')
        .flat_map(|row| {
            let flowers = row.get(student_index..=(student_index + 1)).unwrap();
            flowers
                .chars()
                .map(|f| match f {
                    'G' => "grass",
                    'C' => "clover",
                    'R' => "radishes",
                    'V' => "violets",
                    _ => panic!(),
                })
                .collect::<Vec<&'static str>>()
        })
        .collect()
}
