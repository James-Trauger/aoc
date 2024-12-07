
enum Avl<T: Ord> {
    Leaf(T),
    Node(T, i32, Box<Avl<T>>, Box<Avl<T>>),
    Empty
}
use crate::avl::Avl::*;
impl<T: Ord> Avl<T> {

    fn node_height(&self) -> i32 {
        match self {
            Leaf(_) => 0,
            Node(_, height, _, _) => *height,
            Empty => -1
        }
    }

    fn balance_factor(&self) -> i32 {
        match self {
            Node(v, h, left, right) => {
                right.node_height() - left.node_height()
            },
            _ => 0
        }
    }

    fn change_height(&mut self, delta: i32) -> {
        match self {
            Node(val, h, left, right) => *h += delta,
            _ => unreachable!()
        }
    }

    fn search(&self, key: &T) -> bool {
        match self {
            Leaf(val) => val == key,
            Node(val, _, left, right) => {
                if key < val {
                    left.search(key)
                } else if key > val { 
                    right.search(key)
                } else {
                    true
                }
            },
            Empty => false
        }
    }

    fn insert(self, target: T) -> Box<Avl<T>> {
        match self {
            // normal tree insertion
            Leaf(val) => {
                if target <= val {
                    Box::new(Node(val, 1, Box::new(Leaf(target)), Box::new(Empty)))
                } else {
                    Box::new(Node(val, 1, Box::new(Empty), Box::new(Leaf(target))))
                }
            },
            Node(val, h, left, right) => {
                let (left, right) = if target <= val {
                    (left.insert(target), right)
                } else {
                    (left, right.insert(target))
                };
                // TODO rotate after insertion
                let balance = right.node_height() - left.node_height();
                // left subtree is not balanced
                if balance < -1 {
                    // left-left heavy
                    if left.balance_factor() < 0 {
                        // right rotation
                        Box::new(Node(val, h, left, right).right_rotate())
                    } else {
                        // left-right rotation
                        Node(val, h+1, left, right).left_right()
                    }
                } else {
                    // right-right heavy
                    if right.balance_factor() > 0 {
                        // left rotation
                        Box::new(Node(val, h, left, right).left_rotate())
                    } else {
                        // right-left rotation
                        Node(val, h+1, left, right).right_left()
                    }
                }
            },
            Empty => Box::new(self)
        }
    }
    // performs a single rotation to the left on the subtree rooted at self
    fn left_rotate(self) -> Avl<T> {
        match self {
            Empty => Empty,
            Leaf(v) => Leaf(v),
            Node(val, h, left, right) => {
                    match *right {
                        Empty => Empty,
                        Leaf(right_val) => Node(right_val, h, 
                            Box::new(Node(val, h-1, left, Box::new(Empty))), Box::new(Empty)),
                        Node(right_val, righth, right_left, right_right) => {
                            Node(right_val, h, Box::new(Node(val, h-1, left, right_left)), right_right)
                        }
                    }
                }
            }
        }
    // performs a single rotation to the right on the subtree rooted at self
    fn right_rotate(self) -> Avl<T> {
        match self {
            Empty => Empty,
            Leaf(v) => Leaf(v),
            Node(val, h, left, right) => {
                    match *left {
                        Empty => Empty,
                        Leaf(left_val) => Node(left_val, h, Box::new(Empty),
                            Box::new(Node(val, h-1, Box::new(Empty), right))),
                        Node(left_val, lefth, left_left, left_right) => {
                            Node(left_val, lefth+1, left_left, Box::new(Node(val, h-1, left_right, right)))
                        }
                    }
                }
            }
        }
   
    // left right rotation when balance factor 
    // left rotation on self's left subtree then a rotation on the subtree
    // rooted at self
    fn left_right(self) -> Box<Avl<T>> {
        match self {
            Node(val, h, left, right) => {
                Box::new(Node(val, h, left, Box::new(right.left_rotate())).right_rotate())
            },
            _ => Box::new(self)
        }
    }

    fn right_left(self) -> Box<Avl<T>> {
        match self {
            Node(val, h, left, right) => {
                Box::new(Node(val, h, Box::new(left.right_rotate()), right).left_rotate())
            },
            _ => Box::new(self)
        }
    }
    }

fn height<T: Ord>(node: &Option<Box<Avl<T>>>) -> i32 {
    match node {
        None => 0,
        Some(node) => node.node_height()
    }
}