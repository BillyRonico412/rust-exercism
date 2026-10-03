use std::cmp::min;

const BOOK_PRICE: u32 = 8;

struct BookState([u32; 5]);

impl BookState {
    fn new(books: &[u32]) -> Self {
        let mut book_state = Self([0; 5]);
        books.iter().for_each(|&b| {
            book_state.increment(b);
        });
        book_state
    }

    fn is_empty(&self) -> bool {
        self.0.iter().sum::<u32>() == 0
    }

    fn increment(&mut self, b: u32) {
        self.0[(b - 1) as usize] += 1;
    }

    fn remove_book(&mut self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }

        let mut nb_book = 0;
        self.0.iter_mut().for_each(|b| {
            if *b == 0 {
                return;
            }
            *b -= 1;
            nb_book += 1;
        });
        Some(nb_book)
    }
}

fn compute(nb_book: usize) -> u32 {
    let discount = match nb_book {
        2 => 5,
        3 => 10,
        4 => 20,
        5 => 25,
        _ => 0,
    };
    BOOK_PRICE * (100 - discount) * nb_book as u32
}

#[derive(Default)]
struct Combinations([u32; 5]);

impl Combinations {
    fn add_combination(&mut self, nb_book: usize) {
        self.0[nb_book - 1] += 1;
    }

    fn compute(&self) -> u32 {
        self.0
            .iter()
            .enumerate()
            .map(|(i, &nb_combination)| nb_combination * compute(i + 1))
            .sum()
    }

    fn optimize(&mut self) {
        let m = min(self.0[2], self.0[4]);
        self.0[2] -= m;
        self.0[4] -= m;
        self.0[3] += m + m;
    }
}

pub fn lowest_price(books: &[u32]) -> u32 {
    let mut book_state = BookState::new(books);
    let mut combinations = Combinations::default();
    while let Some(nb_book) = book_state.remove_book() {
        combinations.add_combination(nb_book);
    }
    combinations.optimize();
    combinations.compute()
}
