use core::fmt;
use std::borrow::BorrowMut;

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

    fn change_height(&mut self, delta: i32)  {
        match self {
            Node(val, h, left, right) => *h += delta,
            _ => () // do nothing
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

    fn balance(self) -> Box<Avl<T>> {
        match self {
            Node(val, h, left, right) => {
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
                } else if balance > 1 {
                    // right-right heavy
                    if right.balance_factor() > 0 {
                        // left rotation
                        Box::new(Node(val, h, left, right).left_rotate())
                    } else {
                        // right-left rotation
                        Node(val, h+1, left, right).right_left()
                    }
                } else {
                    Box::new(Node(val, h, left, right))
                }
            },
            _ => Box::new(self)
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
            Node(val, _, left, right) => {
                let (left, right) = if target <= val {
                    (left.insert(target), right)
                } else {
                    (left, right.insert(target))
                };
                // update heights
                let new_height = left.node_height().max(right.node_height()) + 1;
                // balance after insertion
                Node(val, new_height, left, right).balance()
            },
            Empty => Box::new(Leaf(target))
        }
    }

    fn delete(self, target: &T) -> Box<Avl<T>> {
        match self {
            Empty => Box::new(Empty),
            Leaf(val) => {
                if val == *target {
                    // remove the leaf
                    Box::new(Empty)
                } else {
                    Box::new(Leaf(val))
                }
            },
            Node(val, _, mut left, right) => {
                let x = if *target < val {
                    // check the left subtree
                    let left = left.delete(target);
                    // update height
                    let new_height = left.node_height().max(right.node_height()) + 1;
                    Box::new(Node(val, new_height, left, right))
                } else if *target > val {
                    // check the right subtree
                    let right = right.delete(target);
                    // update height
                    let new_height = left.node_height().max(right.node_height()) + 1;
                    Box::new(Node(val, new_height, left, right))
                } else {
                    // remove the node, replace with the left node
                    // replace the current node with the largest value in the left subtree
                    let replacement = left.largest();
                    match replacement {
                        // replace current node with the right instead
                        None => right,
                        Some(large) => {
                            // delete the replacement from the left subtree
                            //let new_left = left.delete(large);
                            Box::new(Empty)
                        }
                    }
                };
                Box::new(Empty)
            }
        }
    }

    // finds the largest value
    fn largest(&mut self) -> Option<&T> {
        match self {
            Empty => None,
            Node(_, _, _, right) => {
                match *right {
                    Leaf(val) => {
                        //*right = Box::new(Empty);
                        None
                    },
                    _ => right.largest()
                }
            },
            Leaf(large) => Some(large)
        }
    }


   // performs a single rotation to the left on the subtree rooted at self
    fn left_rotate(self) -> Avl<T> {
        match self {
            Empty => Empty,
            Leaf(v) => Leaf(v),
            Node(val, _, mut left, right) => {
                    match *right {
                        Empty => Empty,
                        Leaf(right_val) => Node(right_val, left.node_height()+2, 
                            Box::new(Node(val, left.node_height()+1, left, Box::new(Empty))), Box::new(Empty)),
                        Node(right_val, righth, right_left, mut right_right) => {
                             // right_right subtree is pulled up by one level
                             right_right.change_height(1);
                             // left subtree is pulled down by one level
                             left.change_height(-1);
                            let left_height = left.node_height().max(right_left.node_height()) + 1;
                            let new_height = righth.max(left_height);
                            Node(right_val, new_height, Box::new(Node(val, left_height, left, right_left)), right_right)
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
            Node(val, _, left, mut right) => {
                    match *left {
                        Empty => Empty,
                        Leaf(left_val) => Node(left_val, right.node_height()+2, Box::new(Empty),
                            Box::new(Node(val, right.node_height()+1, Box::new(Empty), right))),
                        Node(left_val, lefth, mut left_left, left_right) => {
                            // left_left subtree is pulled up by one level
                            left_left.change_height(1);
                            // right subtree is pulled down by one level
                            right.change_height(-1);
                            let right_height = left_right.node_height().max(right.node_height()) + 1;
                            let new_height = lefth.max(right_height) + 1;
                            Node(left_val, new_height, left_left, Box::new(Node(val, right_height, left_right, right)))
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



