use crate::dom::{self, element_node, Document, Node};
use crate::parser::TextParser;
use std::collections::HashMap;

pub struct HTMLParser {
    text_parser: TextParser,
}

impl HTMLParser {
    pub fn new(input: String) -> HTMLParser {
        let text_parser = TextParser::new(input);
        HTMLParser { text_parser }
    }

    fn parse_tag_name(&mut self) -> String {
        self.text_parser
            .consume_chars_while(|c| c.is_alphanumeric())
    }

    fn parse_text_node(&mut self) -> Node {
        dom::text_node(self.text_parser.parse_text_data())
    }

    fn parse_nodes(&mut self) -> Vec<Box<Node>> {
        let mut nodes = Vec::new();
        loop {
            self.text_parser.remove_whitespaces();
            if self.text_parser.eol() || self.text_parser.starts_with("</") {
                break;
            }
            // for the moment, a node is either an element or a text node
            let node = match self.text_parser.get_current_char() {
                '<' => self.parse_element_node(),
                _ => self.parse_text_node(),
            };
            nodes.push(Box::new(node));
        }
        nodes
    }

    fn parse_element_node(&mut self) -> Node {
        // parse tag name
        let (tag_name, attrs) = self.consume_element_tag();
        let children = self.parse_nodes();

        // Check for tag closing
        assert!(self.text_parser.consume_char() == '<');
        assert!(self.text_parser.consume_char() == '/');
        assert!(self.parse_tag_name() == tag_name);
        assert!(self.text_parser.consume_char() == '>');

        element_node(tag_name, attrs, children)
    }

    fn parse_element_attributes(&mut self) -> HashMap<String, String> {
        let mut attrs: HashMap<String, String> = HashMap::new();
        let cur_char = self.text_parser.get_current_char();
        match cur_char {
            '>' => attrs,
            _ => {
                self.text_parser.consume_char(); // consume the first whitespace
                let cur = self.text_parser.consume_chars_while(|c| c != '>');
                let all_pairs = cur.split(" ");
                for pair in all_pairs {
                    let (attr, val) = pair.split_once("=").unwrap();
                    attrs.insert(attr.to_string(), val.to_string());
                }
                attrs
            }
        }
    }

    fn consume_element_tag(&mut self) -> (String, HashMap<String, String>) {
        assert!(self.text_parser.consume_char() == '<');
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_element_attributes();
        assert!(self.text_parser.consume_char() == '>');
        (tag_name, attrs)
    }

    pub fn parse_document(&mut self) -> Document {
        // parse a document and return the root node
        // parse the root html node
        let (document_tag, document_attributes) = self.consume_element_tag();
        assert!(document_tag == "html");

        let all_nodes = self.parse_nodes();
        let root = element_node(document_tag, document_attributes, all_nodes);
        Document::new(root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_name_parsing() {
        let test_string = "html";
        let mut test_parser = HTMLParser::new(test_string.to_string());
        let first_tag = test_parser.parse_tag_name();
        assert!(first_tag == "html");
    }

    #[test]
    fn test_element_node_simple() {
        let test_string = "<div>Toto</div>";
        let mut test_parser = HTMLParser::new(test_string.to_string());
        test_parser.parse_element_node();
    }
}
