use arrayvec::ArrayVec;

const SIZE: i32 = 8;

#[derive(Clone, Copy, Debug)]
pub struct Node {
    pub loc: i32,
    pub value: i32,
}

impl Node {
    pub fn new(loc: i32, value: i32) -> Node {
        Node {
            loc: loc,
            value: value,
        }
    }

    pub fn to_string(&self) -> String {
        let x = (self.loc % SIZE) as u8;
        let y = (self.loc / SIZE) as u8;

        format!("{}{}", x, y)
    }
}

#[derive(Debug)]
pub struct Nodes {
    pub nodes: ArrayVec<Node, 32>,
}

impl Nodes {
    pub fn new() -> Nodes {
        Nodes {
            nodes: ArrayVec::<Node, 32>::new(),
        }
    }

    fn less(&self, i: usize, j: usize) -> bool {
        self.nodes[i].value < self.nodes[j].value
    }

    fn large(&self, i: usize, j: usize) -> bool {
        self.nodes[i].value > self.nodes[j].value
    }

    fn swap(&mut self, i: usize, j: usize) {
        let tmp = self.nodes[i];
        self.nodes[i] = self.nodes[j];
        self.nodes[j] = tmp;
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    pub fn push(&mut self, n: Node) {
        self.nodes.push(n);
    }

    pub fn sort_desc(&mut self) {
        let len = self.nodes.len();
        if len > 1 {
            for i in 1..len {
                let mut j = i;
                while j > 0 && self.large(j, j - 1) {
                    self.swap(j, j - 1);
                    j -= 1;
                }
            }
        }
    }

    pub fn sort_asc(&mut self) {
        let len = self.nodes.len();
        if len > 1 {
            for i in 1..len {
                let mut j = i;
                while j > 0 && self.less(j, j - 1) {
                    self.swap(j, j - 1);
                    j -= 1;
                }
            }
        }
    }
}
