fn main() {
    println!("Hello, world!");
}

struct Node {
    pub val: i32,
    pub prev: Option<Box<Node>>,
    pub next: Option<Box<Node>>,
    pub child: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Node {
        Node {
            val,
            prev: None,
            next: None,
            child: None,
        }
    }
}

fn flatten_multi_level_doubly_linked_list(head: Option<Box<Node>>) -> Option<Box<Node>> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        // TODO
    }
}
