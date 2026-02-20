use std::collections::BTreeMap;

pub struct School {
    grades: BTreeMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            grades: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let student_string = String::from(student);
        if self.grades.values().flatten().any(|s| s == student) {
            return;
        }
        self.grades
            .entry(grade)
            .and_modify(|students| {
                students.push(String::from(student));
                students.sort();
            })
            .or_insert(vec![student_string]);
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades.keys().copied().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.grades.get(&grade).cloned().unwrap_or_default()
    }
}
