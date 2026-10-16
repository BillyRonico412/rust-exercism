use std::marker::PhantomData;
use std::ptr::NonNull;

mod pre_implemented;

type Link<T> = Option<NonNull<Node<T>>>;

pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

struct Node<T> {
    value: T,
    prev: Link<T>,
    next: Link<T>,
}

pub struct Cursor<'a, T> {
    list: &'a mut LinkedList<T>,
    cur: Link<T>,
}

pub struct Iter<'a, T> {
    next: Link<T>,
    _marker: PhantomData<&'a T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        Cursor {
            cur: self.head,
            list: self,
        }
    }

    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor {
            cur: self.tail,
            list: self,
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head,
            _marker: PhantomData,
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.cursor_front().take().is_some() {}
    }
}

impl<T> Cursor<'_, T> {
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.cur.map(|n| unsafe { &mut (*n.as_ptr()).value })
    }

    pub fn next(&mut self) -> Option<&mut T> {
        let next = unsafe { (*self.cur?.as_ptr()).next }?;
        self.cur = Some(next);
        self.peek_mut()
    }

    pub fn prev(&mut self) -> Option<&mut T> {
        let prev = unsafe { (*self.cur?.as_ptr()).prev }?;
        self.cur = Some(prev);
        self.peek_mut()
    }

    pub fn take(&mut self) -> Option<T> {
        let node = unsafe { Box::from_raw(self.cur?.as_ptr()) };
        match node.prev {
            Some(p) => unsafe { (*p.as_ptr()).next = node.next },
            None => self.list.head = node.next,
        }
        match node.next {
            Some(n) => unsafe { (*n.as_ptr()).prev = node.prev },
            None => self.list.tail = node.prev,
        }
        self.cur = node.next.or(node.prev);
        self.list.len -= 1;
        Some(node.value)
    }

    pub fn insert_after(&mut self, element: T) {
        let new = NonNull::from(Box::leak(Box::new(Node {
            value: element,
            prev: self.cur,
            next: None,
        })));
        match self.cur {
            None => {
                self.list.head = Some(new);
                self.list.tail = Some(new);
                self.cur = Some(new);
            }
            Some(cur) => unsafe {
                let next = (*cur.as_ptr()).next;
                (*new.as_ptr()).next = next;
                (*cur.as_ptr()).next = Some(new);
                match next {
                    Some(n) => (*n.as_ptr()).prev = Some(new),
                    None => self.list.tail = Some(new),
                }
            },
        }
        self.list.len += 1;
    }

    pub fn insert_before(&mut self, element: T) {
        let new = NonNull::from(Box::leak(Box::new(Node {
            value: element,
            prev: None,
            next: self.cur,
        })));
        match self.cur {
            None => {
                self.list.head = Some(new);
                self.list.tail = Some(new);
                self.cur = Some(new);
            }
            Some(cur) => unsafe {
                let prev = (*cur.as_ptr()).prev;
                (*new.as_ptr()).prev = prev;
                (*cur.as_ptr()).prev = Some(new);
                match prev {
                    Some(p) => (*p.as_ptr()).next = Some(new),
                    None => self.list.head = Some(new),
                }
            },
        }
        self.list.len += 1;
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let node = unsafe { &*self.next?.as_ptr() };
        self.next = node.next;
        Some(&node.value)
    }
}
