use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: HashMap<String, String>,
}

pub trait Representation {
    fn repr(&self) -> String;
}

impl Representation for ElementData {
    fn repr(&self) -> String {
        let mut repr = String::from("Element ");
        repr.push_str(&self.tag_name);
        let attrs = self
            .attributes
            .iter()
            .map(|(k, v)| format!("{}:{}", k, v))
            .collect::<Vec<String>>()
            .join(", ");
        repr.push_str(&attrs);
        repr
    }
}

impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(cls) => cls.split(' ').collect(),
            None => HashSet::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub children: Vec<Box<Node>>,
    pub node_type: NodeType,
}

impl Representation for Node {
    fn repr(&self) -> String {
        let mut representation = String::from("Node ");
        let node_type_repr = match &self.node_type {
            NodeType::Element(elem) => elem.repr(),
            NodeType::Text(txt) => format!("Text {}", txt),
            NodeType::Comment(cmt) => format!("Comment {}", cmt),
        };
        representation.push_str(&node_type_repr);
        representation
    }
}

impl Node {
    pub fn add_child(&mut self, child: Node) {
        self.children.push(Box::new(child));
    }

    pub fn element(
        tag_name: String,
        attributes: HashMap<String, String>,
        children: Vec<Box<Node>>,
    ) -> Node {
        Node {
            children,
            node_type: NodeType::Element(ElementData {
                tag_name,
                attributes,
            }),
        }
    }

    pub fn text(data: String) -> Node {
        Node {
            children: Vec::new(),
            node_type: NodeType::Text(data),
        }
    }

    pub fn comment(data: String) -> Node {
        Node {
            children: Vec::new(),
            node_type: NodeType::Comment(data),
        }
    }
}
pub struct Document {
    root: Node,
}

impl Document {
    pub fn new(root: Node) -> Document {
        Document { root }
    }

    pub fn display(&self) {
        pretty_print_tree(&self.root);
    }
}

fn get_indentation_carret(base: &str, depth: usize) -> String {
    let base_line: &str = base.lines().nth(depth).unwrap();
    let space = " ".repeat(base_line.len() / 2);
    format!("\n{}|__ ", space)
}

pub fn pretty_print_tree(root: &Node) {
    fn dfs(root: &Node, root_string: &mut String, visited: &mut HashSet<String>, depth: usize) {
        let node_repr: String = format!("{:?} ", root.node_type);

        // check if the node was already visited
        if !visited.contains(&node_repr) {
            root_string.push_str(&node_repr);

            // add carret for prettier print

            if !root.children.is_empty() {
                let new_depth: usize = depth + 1;

                visited.insert(node_repr);

                for node in root.children.iter() {
                    let carret = get_indentation_carret(root_string, depth);
                    root_string.push_str(&carret);

                    dfs(node, root_string, visited, new_depth);
                }
            } else {
                root_string.push('\n');
            }
        }
    }

    let mut result_string = String::from("");
    let mut visited: HashSet<String> = HashSet::new();
    let depth: usize = 0;
    dfs(root, &mut result_string, &mut visited, depth);
    println!("{}", result_string);
}
