use std::iter::repeat_with;

pub struct CircularBuffer<T> {
    buffer: Vec<Option<T>>,
    head: usize,
    tail: usize,
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
            head: 0,
            tail: 0,
        }
    }

    fn get_next_index(&self, index: usize) -> usize {
        (index + 1).rem_euclid(self.buffer.len())
    }

    fn next_head(&mut self) {
        self.head = self.get_next_index(self.head)
    }

    fn next_tail(&mut self) {
        self.tail = self.get_next_index(self.tail)
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        let slot = &mut self.buffer[self.head];
        match slot {
            None => {
                *slot = Some(element);
                self.next_head();
                Ok(())
            }
            Some(_) => Err(Error::FullBuffer),
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        let slot = self.buffer[self.tail].take().ok_or(Error::EmptyBuffer)?;
        self.next_tail();
        Ok(slot)
    }

    pub fn clear(&mut self) {
        self.buffer.fill_with(|| None);
        self.head = 0;
        self.tail = 0;
    }

    pub fn overwrite(&mut self, element: T) {
        let slot = &mut self.buffer[self.head];
        match slot {
            None => {
                *slot = Some(element);
                self.next_head();
            }
            Some(slot) => {
                *slot = element;
                self.next_head();
                self.next_tail();
            }
        }
    }
}
