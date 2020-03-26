use std::ptr::null_mut;

fn main() {
    let mut mylist = RList::new();

    for i in 0..10 {
        mylist.add(10 * i);
    }

    println!("length {:?}", mylist.len());
    for i in 0..mylist.len() {
        println!("{:?}", mylist.get(i));
    }

    mylist.clear();
    println!("length {:?}", mylist.len());
    println!("{:?}", mylist);

    println!("restore values");
    for i in 0..10 {
        mylist.add(10 * i);
    }

    while mylist.len() != 0 {
        println!("pop_front {:?}", mylist.pop_front());
    }
}

type Link<T> = Option<Box<ListElement<T>>>;

#[derive(Debug)]
pub struct RList<T> where T: std::fmt::Debug {
    length: usize,
    first: Link<T>,
    last: *mut ListElement<T>,
}

impl<T> RList<T> where T: std::fmt::Debug {
    pub fn new() -> Self {
        Self {
            length: 0,
            first: None,
            last: null_mut(),
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn clear(&mut self) {
        self.length = 0;
        self.first = None;
        self.last = null_mut();
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.length -= 1;
        self.first.take().map(|n| {
            self.first = n.next;
            if self.first.is_none() {
                self.last = null_mut();
            }
            n.item
        })
    }

    pub fn add(&mut self, item: T) {
        let mut new_item = Box::new(ListElement {
            item,
            next: None,
        });

        let raw_last: *mut _ = &mut *new_item;
        if self.last.is_null() {
            self.first = Some(new_item);
        } else {
            unsafe {
                (*self.last).next = Some(new_item);
            }
        }

        self.last = raw_last;
        self.length += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if self.length > 1 {
            if index == 0 {
                return Some(&self.first.as_ref().unwrap().item);
            } else if index + 1 == self.length {
                unsafe {
                    return Some(&self.last.as_ref().unwrap().item);
                }
            } else {
                let mut count = 0;
                let mut current = self.first.as_ref().unwrap();
                while index != count {
                    current = current.next.as_ref().unwrap();
                    count += 1;
                }
                return Some(&current.item);
            }
        }

        None
    }
}

#[derive(Debug)]
struct ListElement<T> where T: std::fmt::Debug {
    item: T,
    next: Link<T>,
}
