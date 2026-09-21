/// Yields each item of a and then each item of b
pub fn append<I, J>(mut a: I, mut b: J) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    std::iter::from_fn(move || a.next().or_else(|| b.next()))
}

/// Combines all items in all nested iterators inside into one flattened iterator
pub fn concat<I>(mut nested_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
    I: Iterator,
    I::Item: Iterator,
{
    let mut current_iter_opt = nested_iter.next();
    std::iter::from_fn(move || {
        loop {
            let current_iter = current_iter_opt.as_mut()?;
            match current_iter.next() {
                None => current_iter_opt = nested_iter.next(),
                Some(item) => break Some(item),
            }
        }
    })
}

/// Returns an iterator of all items in iter for which `predicate(item)` is true
pub fn filter<I, F>(mut iter: I, predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    std::iter::from_fn(move || {
        loop {
            let item = iter.next()?;
            if predicate(&item) {
                break Some(item);
            }
        }
    })
}

pub fn length<I: Iterator>(mut iter: I) -> usize {
    let mut size = 0;
    loop {
        let Some(_) = iter.next() else {
            break size;
        };
        size += 1;
    }
}

/// Returns an iterator of the results of applying `function(item)` on all iter items
pub fn map<I, F, U>(mut iter: I, function: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    std::iter::from_fn(move || {
        loop {
            break iter.next().map(&function);
        }
    })
}

pub fn foldl<I, F, U>(mut iter: I, initial: U, function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    let mut acc = initial;
    loop {
        let Some(item) = iter.next() else {
            break acc;
        };
        acc = function(acc, item);
    }
}

pub fn foldr<I, F, U>(mut iter: I, initial: U, function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    let mut acc = initial;
    loop {
        let Some(item) = iter.next_back() else {
            break acc;
        };
        acc = function(acc, item);
    }
}

/// Returns an iterator with all the original items, but in reverse order
pub fn reverse<I: DoubleEndedIterator>(mut iter: I) -> impl Iterator<Item = I::Item> {
    std::iter::from_fn(move || iter.next_back())
}
