use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
pub struct TextData {
    data: String,
}

#[derive(Debug)]
pub struct ElementData {
    tag_name: String,
    attributes: HashMap<String, String>,
}

#[derive(Debug)]
pub struct DocumentTypeData {
    name: String,
    public_id: String,
    system_id: String,
}

#[derive(Debug)]
pub struct ProcessingInstructionData {
    target: String,
}

#[derive(Debug)]
pub struct AttrData {
    namespace_uri: String,
    prefix: String,
    local_name: String,
    name: String,
    value: String,
    owner_element: ElementData,
    specified: bool,
}

#[derive(Debug)]
pub enum NodeType {
    Text(TextData),
    Element(ElementData),
    Comment(TextData),
    Attr(AttrData),
    ProcessingInstruction(ProcessingInstructionData),
    DocumentType(DocumentTypeData),
}

#[derive(Debug)]
pub struct Node {
    children: Vec<Box<Node>>,
    node_type: NodeType,
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
        node_type: NodeType::Text(TextData { data: data }),
    }
}

pub fn element_node(tag: String, attributes: HashMap<String, String>) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Element(ElementData {
            tag_name: tag,
            attributes: attributes,
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
    dbg!(depth);
    let base_line: &str = base.lines().nth(depth).unwrap();
    let space = " ".repeat(base_line.len() / 2);
    
    format!("\n{}|__ ", space)
}


pub fn pretty_print_tree(root: Node) {
    fn dfs(root: &Node, root_string: &mut String, visited: &mut HashSet<String>, depth: &mut usize) {
        let node_repr: String = format!("node: {:?} ", root.node_type);

        // check if the node was already visited
        if !visited.contains(&node_repr) {

            // increase depth and push current node representation
            root_string.push_str(&node_repr);

            if ! (root.children.len() == 0) {

                // add carret for prettier print
                if root_string.len() > 0 {
                    let carret = get_indentation_carret(&root_string, *depth);
                    root_string.push_str(&carret);
                }
                
                visited.insert(node_repr);

                for node in root.children.iter() {
                    dfs(node, root_string, visited, depth);
                }
            }
        }
    }

    let mut result_string = String::from("");
    let mut visited: HashSet<String> = HashSet::new();
    let mut depth: usize = 0;
    dfs(&root, &mut result_string, &mut visited, &mut depth);
    print!("{}\n", result_string);
}
