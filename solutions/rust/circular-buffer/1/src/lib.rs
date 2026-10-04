use std::iter::repeat_with;

pub struct CircularBuffer<T> {
    buffer: Vec<Option<T>>,
    current_index: usize,
    oldest_index: usize,
    capacity: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: repeat_with(|| None).take(capacity).collect(),
            current_index: 0,
            oldest_index: 0,
            capacity,
        }
    }

    fn get_next_index(&self, index: usize) -> usize {
        (index + 1).rem_euclid(self.capacity)
    }

    fn increment_current_index(&mut self) {
        self.current_index = self.get_next_index(self.current_index)
    }

    fn increment_oldest_index(&mut self) {
        self.oldest_index = self.get_next_index(self.oldest_index)
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        let slot = &mut self.buffer[self.current_index];
        match slot {
            None => {
                *slot = Some(element);
                self.increment_current_index();
                Ok(())
            }
            Some(_) => Err(Error::FullBuffer),
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        let slot = self.buffer[self.oldest_index].take();
        match slot {
            None => Err(Error::EmptyBuffer),
            Some(slot) => {
                self.increment_oldest_index();
                Ok(slot)
            }
        }
    }

    pub fn clear(&mut self) {
        self.buffer = repeat_with(|| None).take(self.capacity).collect();
        self.current_index = 0;
        self.oldest_index = 0;
    }

    pub fn overwrite(&mut self, element: T) {
        let slot = &mut self.buffer[self.current_index];
        match slot {
            None => {
                *slot = Some(element);
                self.increment_current_index();
            }
            Some(slot) => {
                *slot = element;
                self.increment_current_index();
                self.increment_oldest_index();
            }
        }
    }
}
