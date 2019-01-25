use std::cell::{RefCell};
use std::rc::Rc;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Node<T>
    where T: Clone + Debug{
    next : Option<Rc<RefCell<Node<T>>>>,
    prev : Option<Rc<RefCell<Node<T>>>>,
    value : T,
}

impl <T> Node<T>
    where T: Clone + Debug{
    pub fn new(value: T) -> Node<T> {
        Node {
            next:None,
            prev:None,
            value
        }
    }

    pub fn value(&self) -> T {
        self.value.clone()
    }

    pub fn prev(&mut self) -> Option<Rc<RefCell<Node<T>>>> {
        match &self.prev{
            Some(x) => Some(Rc::clone(&x)),
            None => None
        }
    }

    pub fn next(&mut self) -> Option<Rc<RefCell<Node<T>>>> {
        match &self.next{
            Some(x) => Some(Rc::clone(&x)),
            None => None
        }
    }

    // Set the next element, and return a mutable reference to self
    pub fn set_next(&mut self, next: Option<Rc<RefCell<Node<T>>>>) -> &mut Self {
        self.next = next;
        self
    }

    // Set the previous element, and return a mutable reference to self
    pub fn set_prev(&mut self, prev: Option<Rc<RefCell<Node<T>>>>) -> &mut Self {
        self.prev = prev;
        self
    }

    // Replace the next item, and return a mutable reference to NEXT item
    pub fn replace_next_and_retrieve(&mut self,  next: Rc<RefCell<Node<T>>>) -> Option<Rc<RefCell<Node<T>>>> {
        let old_next = self.next.clone();
        /*let old_next = match &self.next {
            Some(N) => Some(Rc::clone(&N)),
            None => None
        };*/
        self.next = Some(next);
        old_next
    }

    // Replace the previous item, and return a mutable reference to PREVIOUS item
    pub fn replace_prev_and_retrieve(&mut self,  prev: Rc<RefCell<Node<T>>>) -> Option<Rc<RefCell<Node<T>>>> {
        let old_prev = self.prev.clone();
        self.prev = Some(prev);
        old_prev
    }

    // Swap the next of this node with the next of the argument node. Return mutable reference to self.
    pub fn swap_next<'a>(&'a mut self, other: &'a Rc<RefCell<Node<T>>>) -> &'a mut Self {
        let tmp_next = other.borrow().next.clone();
        other.borrow_mut().next = self.next.clone();
        self.next = tmp_next;
        self
    }

    // Swap the prev of this node with the prev of the argument node. Return mutable reference to self.
    pub fn swap_prev<'a>(&'a mut self, other: &'a Rc<RefCell<Node<T>>>) -> &'a mut Self {
        let tmp_prev = other.borrow().prev.clone();
        other.borrow_mut().prev = self.prev.clone();
        self.next = tmp_prev;
        self
    }
}

pub struct NodeIterator<T>
    where T: Clone + Debug{
    node : Option<Rc<RefCell<Node<T>>>>,
}

impl <T> Iterator for NodeIterator<T>
    where T: Clone + Debug{
    type Item = Rc<RefCell<Node<T>>>;

    fn next(&mut self) ->  Option<Rc<RefCell<Node<T>>>> {
        let node = self.node.clone();
        if let Some(x) = self.node.clone() {
            self.node = x.borrow_mut().next();
        } else {
            self.node = None;
        }
        node
    }
}

/* Assertions
 * 1) Both head and tail are either empty, or contain a node.
 * 2) The list is not cyclic.
 */
pub struct List<T>
    where T: Clone + Debug{
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl <T> List<T>
    where T: Clone + Debug{
    pub fn new() -> List<T> {
        List {
            head : None,
            tail : None,
        }
    }

    pub fn head(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.head.clone()
    }

    pub fn tail(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.tail.clone()
    }

    pub fn remove(&mut self, node: Option<Rc<RefCell<Node<T>>>>) -> Option<Rc<RefCell<Node<T>>>> {
        if let Some(node_to_remove) = node {
            for n in &*self {
                if Rc::ptr_eq(&n, &node_to_remove) {
                    let node_next = n.borrow_mut().next();
                    let node_prev = n.borrow_mut().prev();

                    if let Some(prev) = node_prev.clone() {
                        prev.borrow_mut().set_next(node_next.clone());
                    } else {
                        // we are in the head
                        self.head = node_next.clone();
                    }

                    if let Some(next) = node_next {
                        next.borrow_mut().set_prev(node_prev.clone());
                    } else {
                        // we are in the tail
                        self.tail = node_prev;
                    }

                    return Some(n);
                }
            }
            None
        } else {
            None
        }
    }

    pub fn head_value(&self) -> Option<T> {
        match &self.head {
            Some(node) => Some(node.borrow().value()),
            None => None
        }
    }

    pub fn push_back(&mut self, value : T) -> &mut List<T> {
        let node = Rc::new(RefCell::new(Node::new(value)));

        match &self.tail {
            Some(tail) => {
                tail.borrow_mut().set_next(Some(Rc::clone(&node)));
                node.borrow_mut().set_prev(self.tail.clone());
                self.tail = Some(node);
            },
            None => {
                self.head = Some(Rc::clone(&node));
                self.tail = Some(node);
            }
        };

        self
    }
}

impl<T> IntoIterator for List<T>
    where T: Clone + Debug {
    type Item = Rc<RefCell<Node<T>>>;
    type IntoIter = NodeIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        NodeIterator {
            node: self.head
        }
    }
}

impl <T> IntoIterator for &List<T>
    where T: Clone + Debug {
    type Item = Rc<RefCell<Node<T>>>;
    type IntoIter = NodeIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        NodeIterator {
            node: self.head.clone()
        }
    }
}

impl <T> IntoIterator for &mut List<T>
    where T: Clone + Debug {
    type Item = Rc<RefCell<Node<T>>>;
    type IntoIter = NodeIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        NodeIterator {
            node: self.head.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let mut list = List::new();
        list.push_back(7)
            .push_back(8)
            .push_back(9)
            .push_back(10);

        for value in &list {
            println!("{}", value.borrow().value());
        }

        let node = list.head().unwrap().borrow_mut().next();
        list.remove(node);

        for value in &list {
            println!("{}", value.borrow().value());
        }

        list.remove(list.tail());

        for value in list {
            println!("{}", value.borrow().value());
        }

        println!("Done");
    }
}
