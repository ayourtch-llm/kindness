#[derive(Clone, Copy, PartialEq)]
enum Color {
    Red,
    Black,
}

struct Node<T> {
    val: T,
    color: Color,
    left: Option<usize>,
    right: Option<usize>,
    parent: Option<usize>,
}

pub struct RBTree<T: Ord> {
    nodes: Vec<Node<T>>,
    root: Option<usize>,
}

impl<T: Ord> RBTree<T> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            root: None,
        }
    }

    pub fn insert(&mut self, val: T) {
        let new_idx = self.nodes.len();

        // BST insert
        if self.root.is_none() {
            self.nodes.push(Node {
                val,
                color: Color::Black,
                left: None,
                right: None,
                parent: None,
            });
            self.root = Some(0);
            return;
        }

        let mut current = self.root.unwrap();
        loop {
            use std::cmp::Ordering;
            match val.cmp(&self.nodes[current].val) {
                Ordering::Equal => return, // duplicate
                Ordering::Less => {
                    if let Some(left) = self.nodes[current].left {
                        current = left;
                    } else {
                        self.nodes.push(Node {
                            val,
                            color: Color::Red,
                            left: None,
                            right: None,
                            parent: Some(current),
                        });
                        self.nodes[current].left = Some(new_idx);
                        break;
                    }
                }
                Ordering::Greater => {
                    if let Some(right) = self.nodes[current].right {
                        current = right;
                    } else {
                        self.nodes.push(Node {
                            val,
                            color: Color::Red,
                            left: None,
                            right: None,
                            parent: Some(current),
                        });
                        self.nodes[current].right = Some(new_idx);
                        break;
                    }
                }
            }
        }

        self.fix_insert(new_idx);
    }

    fn fix_insert(&mut self, mut z: usize) {
        while let Some(p) = self.nodes[z].parent {
            if self.nodes[p].color == Color::Black {
                break;
            }
            let gp = match self.nodes[p].parent {
                Some(g) => g,
                None => break,
            };

            if Some(p) == self.nodes[gp].left {
                let uncle = self.nodes[gp].right;
                if uncle.map_or(false, |u| self.nodes[u].color == Color::Red) {
                    // Case 1: uncle is red
                    self.nodes[p].color = Color::Black;
                    self.nodes[uncle.unwrap()].color = Color::Black;
                    self.nodes[gp].color = Color::Red;
                    z = gp;
                } else {
                    if Some(z) == self.nodes[p].right {
                        // Case 2: z is right child
                        z = p;
                        self.rotate_left(z);
                    }
                    let p = self.nodes[z].parent.unwrap();
                    let gp = self.nodes[p].parent.unwrap();
                    // Case 3
                    self.nodes[p].color = Color::Black;
                    self.nodes[gp].color = Color::Red;
                    self.rotate_right(gp);
                }
            } else {
                let uncle = self.nodes[gp].left;
                if uncle.map_or(false, |u| self.nodes[u].color == Color::Red) {
                    self.nodes[p].color = Color::Black;
                    self.nodes[uncle.unwrap()].color = Color::Black;
                    self.nodes[gp].color = Color::Red;
                    z = gp;
                } else {
                    if Some(z) == self.nodes[p].left {
                        z = p;
                        self.rotate_right(z);
                    }
                    let p = self.nodes[z].parent.unwrap();
                    let gp = self.nodes[p].parent.unwrap();
                    self.nodes[p].color = Color::Black;
                    self.nodes[gp].color = Color::Red;
                    self.rotate_left(gp);
                }
            }
        }
        if let Some(root) = self.root {
            self.nodes[root].color = Color::Black;
        }
    }

    fn rotate_left(&mut self, x: usize) {
        let y = self.nodes[x].right.unwrap();
        self.nodes[x].right = self.nodes[y].left;
        if let Some(yl) = self.nodes[y].left {
            self.nodes[yl].parent = Some(x);
        }
        self.nodes[y].parent = self.nodes[x].parent;
        match self.nodes[x].parent {
            None => self.root = Some(y),
            Some(p) => {
                if self.nodes[p].left == Some(x) {
                    self.nodes[p].left = Some(y);
                } else {
                    self.nodes[p].right = Some(y);
                }
            }
        }
        self.nodes[y].left = Some(x);
        self.nodes[x].parent = Some(y);
    }

    fn rotate_right(&mut self, x: usize) {
        let y = self.nodes[x].left.unwrap();
        self.nodes[x].left = self.nodes[y].right;
        if let Some(yr) = self.nodes[y].right {
            self.nodes[yr].parent = Some(x);
        }
        self.nodes[y].parent = self.nodes[x].parent;
        match self.nodes[x].parent {
            None => self.root = Some(y),
            Some(p) => {
                if self.nodes[p].left == Some(x) {
                    self.nodes[p].left = Some(y);
                } else {
                    self.nodes[p].right = Some(y);
                }
            }
        }
        self.nodes[y].right = Some(x);
        self.nodes[x].parent = Some(y);
    }

    pub fn contains(&self, val: &T) -> bool {
        let mut current = self.root;
        while let Some(idx) = current {
            use std::cmp::Ordering;
            match val.cmp(&self.nodes[idx].val) {
                Ordering::Equal => return true,
                Ordering::Less => current = self.nodes[idx].left,
                Ordering::Greater => current = self.nodes[idx].right,
            }
        }
        false
    }

    pub fn to_sorted_vec(&self) -> Vec<&T> {
        let mut result = Vec::new();
        self.inorder(self.root, &mut result);
        result
    }

    fn inorder<'a>(&'a self, node: Option<usize>, result: &mut Vec<&'a T>) {
        if let Some(idx) = node {
            self.inorder(self.nodes[idx].left, result);
            result.push(&self.nodes[idx].val);
            self.inorder(self.nodes[idx].right, result);
        }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}
