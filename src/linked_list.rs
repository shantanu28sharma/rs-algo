use std::cell::RefCell;
use std::rc::Rc;

type NodeOption = Option<Rc<RefCell<Node>>>;

struct Node {
    val: i32,
    next: NodeOption
}

impl Node {
    fn new(val: i32)->Self {
        Node {
            val,
            next : None,
        }
    }
} 

pub struct LinkedList{
    count: u32,
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
}