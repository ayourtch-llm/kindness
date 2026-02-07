type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    val: T,
    next: Link<T>,
}

pub struct LinkedList<T> {
    head: Link<T>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None, len: 0 }
    }

    pub fn push_front(&mut self, val: T) {
        let new_node = Box::new(Node {
            val,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.len += 1;
    }

    pub fn push_back(&mut self, val: T) {
        let new_node = Box::new(Node { val, next: None });
        let node_ptr = &mut self.head;
        let mut current = node_ptr;
        while let Some(ref mut node) = current {
            current = &mut node.next;
        }
        *current = Some(new_node);
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.len -= 1;
            node.val
        })
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T: Clone> LinkedList<T> {
    pub fn to_vec(&self) -> Vec<T> {
        let mut result = Vec::new();
        let mut current = &self.head;
        while let Some(ref node) = current {
            result.push(node.val.clone());
            current = &node.next;
        }
        result
    }
}
