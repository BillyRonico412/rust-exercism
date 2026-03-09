use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T>
where
    T: Eq + Hash + Clone,
{
    set: HashSet<T>,
}

impl<T: Eq + Hash + Clone> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut data = HashSet::with_capacity(input.len());
        for x in input {
            data.insert(x.clone());
        }
        Self { set: data }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.set.contains(element)
    }

    pub fn add(&mut self, element: T) {
        self.set.insert(element);
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.set.is_subset(&other.set)
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.set.is_disjoint(&other.set)
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        Self {
            set: self.set.intersection(&other.set).cloned().collect(),
        }
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        Self {
            set: self.set.difference(&other.set).cloned().collect(),
        }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        Self {
            set: self.set.union(&other.set).cloned().collect(),
        }
    }
}
