type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    data: T,
    next: Link<T>,
}

pub struct LinkedList<T> {
    head: Link<T>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self {
            head: Default::default(),
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push_front(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn find(&self, target_value: &T) -> bool
    where
        T: Eq,
    {
        let mut current = self.head.as_ref(); // Get an immutable reference to the head

        while let Some(node) = current {
            if node.data == *target_value {
                return true; // Found the value
            }
            current = node.next.as_ref(); // Move to the next node
        }

        false // Value not found after traversing the entire list
    }

    pub fn delete_first(&mut self) {
        self.head = self.head.as_mut().and_then(|node| node.next.take());
    }

    fn delete(&mut self, value: T)
    where
        T: Eq,
    {
        // Special case: delete from head
        while let Some(ref node) = self.head {
            if node.data == value {
                self.head = self.head.take().and_then(|node| node.next);
                return;
            } else {
                break;
            }
        }

        // General case: traverse and delete
        let mut current = self.head.as_mut();
        while let Some(node) = current {
            if let Some(ref mut next_node) = node.next {
                if next_node.data == value {
                    node.next = next_node.next.take(); // skip over matched node
                    return;
                }
            }
            current = node.next.as_mut();
        }
    }

    fn traverse(&self)
    where
        T: std::fmt::Debug,
    {
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            println!("{:?}", node.data);
            current = node.next.as_ref();
        }
    }
}
