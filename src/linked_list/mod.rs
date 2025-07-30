type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    data: T,
    next: Link<T>
}

pub struct LinkedList<T> {
    head: Link<T>
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self{ head: None }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self { head: Default::default() }
    }
}