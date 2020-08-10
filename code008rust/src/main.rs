#[derive(Debug)]
struct Node<'a> {
    value: u16,
    left: Option<&'a Node<'a>>,
    right: Option<&'a Node<'a>>,
}

impl<'a> Node<'a> {
    fn new(v: u16) -> Self {
        Node {
            value: v,
            left: None,
            right: None,
        }
    }

    fn isunival(&self) -> bool {
        match &self.left {
            None => return true,
            Some(nodeleft) => {
                if nodeleft.value != self.value || !self.isunival() {
                    return false;
                }
            }
        }

        match &self.right {
            None => return true,
            Some(noderight) => {
                if noderight.value != self.value || !self.isunival() {
                    return false;
                }
            }
        }

        true
    }

    fn set_left(&mut self, node: &'a Node<'a>) {
        self.left = Some(node);
    }

    fn set_right(&mut self, node: &'a Node<'a>) {
        self.right = Some(node);
    }

    fn count_unival(&self) -> u16 {
        let mut count: u16 = 1;
        let mut state: bool = true;

        match &self.left {
            None => state = false,
            Some(nodeleft) => {
                if nodeleft.value == self.value {
                    count = count + 1;
                } else {
                    state = false;
                }

                count += nodeleft.count_unival();
            }
        }

        match &self.right {
            None => state = false,
            Some(noderight) => {
                if noderight.value == self.value {
                    count = count + 1;
                } else {
                    state = false;
                }

                count += noderight.count_unival();
            }
        }

        if state {
            count = count + 1;
        }

        return count;
    }
}

fn main() {
    println!("Starting");

    /**
     *              1
     *           /       \
     *         2           3
     *        /  \       /   \
     *       2    2     3     3
     *     /   \       / \   / \
     *    5     5     4   4  3  3
     */
    let mut root = Node::new(1);
    let mut n1 = Node::new(2);
    let mut n2 = Node::new(3);
    let mut n3 = Node::new(2);
    let n4 = Node::new(2);
    let n5 = Node::new(5);
    let n6 = Node::new(5);
    let mut n7 = Node::new(3);
    let mut n8 = Node::new(3);
    let n9 = Node::new(4);
    let n10 = Node::new(4);
    let n11 = Node::new(3);
    let n12 = Node::new(3);

    root.set_left(&mut n1);
    root.set_right(&mut n2);
    n1.set_left(&mut n3);
    n1.set_right(&mut n4);
    n3.set_left(&n5);
    n3.set_right(&n6);
    n2.set_left(&n7);
    n2.set_right(&n8);
    n7.set_left(&n9);
    n7.set_right(&n10);
    n8.set_left(&n11);
    n8.set_right(&n12);

    println!("Node: {:}", root.count_unival());
}
