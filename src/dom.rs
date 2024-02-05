use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct TextData {
    data: String,
}

#[derive(Debug, Clone)]
pub struct ElementData {
    tag_name: String,
    attributes: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct DocumentTypeData {
    name: String,
    public_id: String,
    system_id: String,
}

#[derive(Debug, Clone)]
pub struct ProcessingInstructionData {
    target: String,
}

#[derive(Debug, Clone)]
pub struct AttrData {
    namespace_uri: String,
    prefix: String,
    local_name: String,
    name: String,
    value: String,
    owner_element: ElementData,
    specified: bool,
}

#[derive(Debug, Clone)]
pub enum NodeType {
    Text(TextData),
    Element(ElementData),
    Comment(TextData),
    Attr(AttrData),
    ProcessingInstruction(ProcessingInstructionData),
    DocumentType(DocumentTypeData),
}

#[derive(Debug, Clone)]
pub struct Node {
    pub children: Vec<Box<Node>>,
    pub node_type: NodeType,
}

impl Node {
    pub fn add_child(&mut self, child: Node) {
        self.children.push(Box::new(child));
    }
}

pub struct Document {
    root: Node,
}

// constructors

pub fn text_node(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(TextData { data }),
    }
}

pub fn element_node(
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

pub fn comment_node(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Comment(TextData { data: data }),
    }
}

pub fn processing_instruction_node(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::ProcessingInstruction(ProcessingInstructionData { target: data }),
    }
}

pub fn attr_node(
    namespace_uri: String,
    prefix: String,
    local_name: String,
    name: String,
    value: String,
    owner_element: ElementData,
    specified: bool,
) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Attr(AttrData {
            namespace_uri,
            prefix,
            local_name,
            name,
            value,
            owner_element,
            specified,
        }),
    }
}

pub fn document_tree(root_node: Node) -> Document {
    Document { root: root_node }
}

fn get_indentation_carret(base: &str, depth: usize) -> String {
    let base_line: &str = base.lines().nth(depth).unwrap();
    let space = " ".repeat(base_line.len() / 2);
    format!("\n{}|__ ", space)
}

pub fn pretty_print_tree(root: Node) {
    fn dfs(root: &Node, root_string: &mut String, visited: &mut HashSet<String>, depth: usize) {
        let node_repr: String = format!("{:?} ", root.node_type);

        // check if the node was already visited
        if !visited.contains(&node_repr) {
            root_string.push_str(&node_repr);

            // add carret for prettier print

            if !(root.children.len() == 0) {
                let new_depth: usize = depth + 1;

                visited.insert(node_repr);

                for node in root.children.iter() {
                    let carret = get_indentation_carret(&root_string, depth);
                    root_string.push_str(&carret);

                    dfs(node, root_string, visited, new_depth);
                }
            } else {
                root_string.push_str("\n");
            }
        }
    }

    let mut result_string = String::from("");
    let mut visited: HashSet<String> = HashSet::new();
    let depth: usize = 0;
    dfs(&root, &mut result_string, &mut visited, depth);
    print!("{}\n", result_string);
}
