use std::ops::Rem;

pub struct Matcher<T>
where
    T: ToString + Clone + Rem<Output = T> + From<u8> + PartialEq,
{
    func: fn(T) -> bool,
    subs: String,
}

impl<T> Matcher<T>
where
    T: ToString + Clone + Rem<Output = T> + From<u8> + PartialEq,
{
    pub fn new<S>(func: fn(T) -> bool, subs: S) -> Matcher<T>
    where
        S: ToString,
    {
        Self {
            func,
            subs: subs.to_string(),
        }
    }
}

pub struct Fizzy<T>(Vec<Matcher<T>>)
where
    T: ToString + Clone + Rem<Output = T> + From<u8> + PartialEq;

impl<T> Fizzy<T>
where
    T: ToString + Clone + Rem<Output = T> + From<u8> + PartialEq,
{
    pub fn new() -> Self {
        Fizzy(vec![])
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.0.push(matcher);
        Self(self.0)
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = T>,
    {
        iter.map(move |it| {
            let subs = self
                .0
                .iter()
                .filter_map(|matcher| (matcher.func)(it.clone()).then_some(matcher.subs.clone()))
                .collect::<String>();
            if subs.len() == 0 {
                it.to_string()
            } else {
                subs
            }
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: ToString + Clone + Rem<Output = T> + From<u8> + PartialEq,
{
    Fizzy(vec![
        Matcher::new(|n: T| n % T::from(3u8) == T::from(0u8), "fizz"),
        Matcher::new(|n: T| n % T::from(5u8) == T::from(0u8), "buzz"),
    ])
}
