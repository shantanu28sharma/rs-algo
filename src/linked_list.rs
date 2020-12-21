use std::cell::RefCell;
use std::rc::Rc;

type NodeOption = Option<Rc<RefCell<Node>>>;

pub struct Node {
    pub val: i32,
    pub next: NodeOption
}

impl Node {
    fn new(val: i32)->Self {
        Node {
            val,
            next : None,
        }
    }
} 

pub struct ListIterator {
    current: NodeOption
}

impl ListIterator {
    fn new(start_at: NodeOption) -> Self {
        ListIterator{
            current: start_at
        }
    }
}

impl Iterator for ListIterator {
    type Item = Rc<RefCell<Node>>;
    fn next(&mut self) -> NodeOption {
        let mut result : NodeOption;
        match self.current.take() {
            Some(node) => {
                result = Some(Rc::clone(&node));
                match &node.borrow().next{
                    Some(next) => {
                        self.current = Some(Rc::clone(next));                    }
                    _ => {
                        self.current = None;
                    }
                }
            }
            _ => {
                result = None;
            }
        }
        return result
    }
}

pub struct LinkedList{
    pub count: u32,
    head: NodeOption,
    tail: NodeOption
}

impl LinkedList{
     fn new(node: Node) -> Self {
        let node_ref = Rc::new(RefCell::new(node));
        LinkedList{
            count: 1,
            head: Some(Rc::clone(&node_ref)),
            tail: Some(Rc::clone(&node_ref))
        }
     }

     fn get_head(&self)-> NodeOption{
        match self.head.as_ref() {
            Some(head) => {
                Some(Rc::clone(head))
            }
            _=> None
        }
     }

     fn get_tail(&self)-> NodeOption{
        match self.tail.as_ref() {
            Some(tail) => {
                Some(Rc::clone(tail))
            }
            _ => None
        }
     }

     fn add_tail(&mut self, node: Node){
        let node_ref = Rc::new(RefCell::new(node));
        match self.head.take(){
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(Rc::clone(&node_ref));
                self.count += 1;
                self.tail = Some(Rc::clone(&node_ref));
            }
            _ => {}
        }
     }

     fn add_head(&mut self, node: Node){
        let node_ref = Rc::new(RefCell::new(node));
        match self.head.take(){
            Some(old_head) => {
                node_ref.borrow_mut().next = Some(Rc::clone(&old_head));
                self.count+=1;
                self.head = Some(Rc::clone(&node_ref));
            }
            _ => {}
        }
     }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn new_node() {
        let node = Node::new(5);
        let mut ll = LinkedList::new(node);
        let nnode = Node::new(10);
        ll.add_head(nnode);
        let tnode = Node::new(15);
        ll.add_tail(tnode);
        assert_eq!(ll.count, 3);
    }

    #[test]
    fn iterate() {
        let node = Node::new(5);
        let mut ll = LinkedList::new(node);
        let nnode = Node::new(10);
        ll.add_head(nnode);
        let tnode = Node::new(15);
        ll.add_tail(tnode);
        let mut iter = ListIterator::new(ll.get_head());
        let f = iter.next();
        // assert_eq!(Rc::clone(&f.unwrap()).into_inner().val, 15);
    }
}