
pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self{ head: None }
    }
}