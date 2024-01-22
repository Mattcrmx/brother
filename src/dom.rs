use std::collections::HashMap;
use std::fmt::{format, Display};

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
            namespace_uri: namespace_uri,
            prefix: prefix,
            local_name: local_name,
            name: name,
            value: value,
            owner_element: owner_element,
            specified: specified,
        }),
    }
}

pub fn document_tree(root_node: Node) -> Document {
    Document { root: root_node }
}

pub trait PrettyPrint {
    fn pretty_print(&self) -> String;
}

pub fn pretty_print_tree(root: Node) {
    fn dfs(root: &Node, root_string: &mut String) {
        if root.children.len() == 0 {
            print!("{}\n", root_string);
        } else {
            root_string.push_str(&format!("node: {:?}", root.node_type));

            for node in root.children.iter() {
                dfs(node, root_string);
            }
        }
    }

    let mut result_string = String::from("");
    dfs(&root, &mut result_string);
}
