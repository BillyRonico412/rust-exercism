pub enum SimpleLinkedList<T> {
    Cons {
        data: T,
        next: Box<SimpleLinkedList<T>>,
    },
    Nil,
}

impl<T: Copy> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self::Nil
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Nil => true,
            _ => false,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Nil => 0,
            Self::Cons { data: _, next } => 1 + next.len(),
        }
    }

    pub fn push(&mut self, element: T) {
        match self {
            Self::Nil => {
                *self = Self::Cons {
                    data: element,
                    next: Box::new(Self::Nil),
                };
            }
            Self::Cons { data: _, next } => {
                next.push(element);
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self {
            Self::Nil => None,
            Self::Cons { data, next } => match &**next {
                Self::Nil => {
                    let data_cloned = data.clone();
                    *self = Self::Nil;
                    return Some(data_cloned);
                }
                _ => next.pop(),
            },
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match self {
            Self::Nil => None,
            Self::Cons { data, next } => next.peek().or(Some(data)),
        }
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        match self.pop() {
            None => Self::Nil,
            Some(element) => Self::Cons {
                data: element,
                next: Box::new(self.rev()),
            },
        }
    }
}

impl<T: Copy> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for i in iter {
            list.push(i);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.
//
// Please note that the "front" of the linked list should correspond to the "back"
// of the vector as far as the tests are concerned.

impl<T: Copy> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(list: SimpleLinkedList<T>) -> Vec<T> {
        let mut list_vec: Vec<_> = Vec::with_capacity(list.len());
        let mut current_list = &list;
        while let SimpleLinkedList::Cons { data, next } = current_list {
            list_vec.push(*data);
            current_list = next;
        }
        list_vec
    }
}
