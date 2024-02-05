use std::collections::HashMap;
use crate::dom::{self, element_node, Node};

pub struct HTMLParser {
    position: usize,
    input: String,
}

impl HTMLParser {
    fn eol(&self) -> bool {
        self.position >= self.input.len()
    }

    fn state(&self) -> &str {
        &self.input[self.position..]
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.position..].starts_with(s)
    }

    fn ends_with(&self, s: &str) -> bool {
        self.input[self.position..].ends_with(s)
    }

    fn get_current_char(&self) -> char {
        self.input.chars().nth(self.position).unwrap()
    }

    fn get_next_char(&self) -> Option<char> {
        // return None if we reach the end of the word
        // doesn't consume the char
        if !self.eol() {
            self.input.chars().nth(self.position + 1)
        } else {
            None
        }
    }

    fn peek(&self, offset: usize) -> Option<char> {
        let new_position: usize = self.position + offset;
        if new_position < self.input[self.position..].len() {
            self.input.chars().nth(new_position)
        } else {
            None
        }
    }

    fn consume_char(&mut self) -> char {
        if self.eol() {
            panic!("Trying to consume character when end of input is reached");
        }

        let mut char_iter = self.input[self.position..].chars();
        self.position += 1;

        char_iter.next().unwrap()
    }

    fn consume_sequence(&mut self, s: String) -> String {
        assert!(self.input[self.position..s.len()] == s);
        s
    }

    fn consume_chars_while<F>(&mut self, predicate: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::from("");

        // loop through the input
        while !self.eol() && predicate(self.get_current_char()) {
            result.push(self.consume_char());
        }
        return result;
    }

    fn remove_whitespaces(&mut self) {
        self.consume_chars_while(|c| c.is_whitespace());
    }

    fn parse_text_data(&mut self) -> String {
        self.consume_chars_while(|c| c != '<')
    }

    pub fn parse_tag_name(&mut self) -> String {
        self.consume_chars_while(|c| c.is_alphanumeric())
    }

    pub fn parse_text_node(&mut self) -> Node {
        dom::text_node(self.parse_text_data())
    }

    pub fn parse_comment_node(&mut self) -> Node {
        dom::comment_node(self.parse_text_data())
    }

    pub fn parse_processing_instruction_node(&mut self) -> Node {
        dom::processing_instruction_node(self.parse_text_data())
    }

    pub fn parse_nodes(&mut self) -> Vec<Box<Node>> {
        let mut nodes = Vec::new();
        loop {
            self.remove_whitespaces();
            if self.eol() || self.starts_with("</") {
                break;
            }
            // for the moment, a node is either an element or a text node
            let node = match self.get_current_char() {
                '<' => self.parse_element_node(),
                _ => self.parse_text_node(),
            };
            nodes.push(Box::new(node));
        }
        nodes
    }

    pub fn parse_element_node(&mut self) -> Node {
        // parse tag name
        let (tag_name, attrs) = self.consume_element_tag();
        let children = self.parse_nodes();

        // Check for tag closing
        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        assert!(self.parse_tag_name() == tag_name);
        assert!(self.consume_char() == '>');

        element_node(tag_name, attrs, children)
    }

    fn parse_element_attributes(&mut self) -> HashMap<String, String> {
        let mut attrs: HashMap<String, String> = HashMap::new();
        let cur_char = self.get_current_char();
        match cur_char {
            '>' => attrs,
            _ => {
                self.consume_char(); // consume the first whitespace
                let cur = self.consume_chars_while(|c| c != '>');
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
        assert!(self.consume_char() == '<');
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_element_attributes();
        assert!(self.consume_char() == '>');
        (tag_name, attrs)
    }

    pub fn parse_document(&mut self) -> Node {
        // parse a document and return the root node
        // parse the root html node
        let (document_tag, document_attributes) = self.consume_element_tag();
        assert!(document_tag == "html");

        let all_nodes = self.parse_nodes();
        element_node(document_tag, document_attributes, all_nodes)
    }
}

pub fn parser(input: String) -> HTMLParser {
    HTMLParser { position: 0, input }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_next_char() {
        let test_parser = parser(String::from("toto"));
        let c = test_parser.get_next_char().unwrap();
        assert!(c == 'o');
        let c2 = test_parser.get_next_char().unwrap();
        assert!(c2 == 'o');
    }

    #[test]
    fn test_no_next_char() {
        let mut test_parser = parser(String::from("to"));
        let c = test_parser.get_next_char();
        assert!(c == Some('o'));
        test_parser.position += 1;
        let c2 = test_parser.get_next_char();
        assert!(c2 == None);
    }

    #[test]
    fn test_peek() {
        let test_parser = parser(String::from("toto"));
        assert!(test_parser.peek(2) == Some('t'));
        assert!(test_parser.peek(25) == None);
    }

    #[test]
    fn test_whitespace_removal() {
        let mut test_parser = parser(String::from("    toto"));
        test_parser.remove_whitespaces();
        assert!(test_parser.input[test_parser.position..] == String::from("toto"));
    }
}
