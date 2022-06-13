struct Solution {}

struct Node {
    name: String,
    children: Vec<Box<Node>>,
}

impl Node {
    fn dfs(&self, array: &mut Vec<String>) {
        array.push(self.name.clone());

        for child in self.children.iter() {
            child.dfs(array);
        }
    }
}

impl Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dfs() {
        let d_node = Node {
            name: String::from("D"),
            children: vec![],
        };

        let b_node = Node {
            name: String::from("B"),
            children: vec![Box::new(d_node)],
        };

        let c_node = Node {
            name: String::from("C"),
            children: vec![],
        };

        let node = Node {
            name: String::from("A"),
            children: vec![Box::new(b_node), Box::new(c_node)],
        };

        let mut result: Vec<String> = vec![];
        node.dfs(&mut result);

        println!("RESULT IS {:?}", result);
    }
}
