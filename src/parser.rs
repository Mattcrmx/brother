use std::collections::HashMap;

use crate::dom::{self, Node};

pub struct HTMLParser {
    position: usize,
    input: String,
}

impl HTMLParser {
    fn eol(&self) -> bool {
        self.position >= self.input.len()
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

    fn consume_chars_while<F>(&mut self, predicate: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::from("");
        let mut cons: bool = true;

        // loop through the input
        while !self.eol() && cons {
            let next_char = self.get_next_char();
            cons = match next_char {
                Some(c) => predicate(c),
                None => false,
            };
            result.push(self.consume_char());
        }
        return result;
    }

    fn remove_whitespaces(&mut self) {
        self.consume_chars_while(|c| c.is_whitespace());
    }

    pub fn parse_tag_name(&mut self) -> String {
        assert!(self.get_next_char().unwrap() == '<');
        self.consume_chars_while(|c| c.is_alphanumeric() && c != '>')
    }

    fn _parse_text_data(&mut self) -> String {
        self.consume_chars_while(|c| c.is_alphanumeric() && c != '<')
    }

    pub fn parse_text_node(&mut self) -> Node {
        dom::text_node(self._parse_text_data())
    }

    pub fn parse_comment_node(&mut self) -> Node {
        dom::comment_node(self._parse_text_data())
    }

    pub fn parse_processing_instruction_node(&mut self) -> Node {
        dom::processing_instruction_node(self._parse_text_data())
    }

    pub fn parse_element_node(&mut self) -> Node {
        // parse tag name
        // start by the '<' character
        let full_tag = self.consume_chars_while(|c| c != '>');
        let mut tag_iter = full_tag.split(" ");
        let tag_name = tag_iter.next().unwrap();
        let mut attrs: HashMap<String, String> = HashMap::new(); // we know the tag will be the first element

        for pair in tag_iter {
            let (attr, val) = pair.split_once("=").unwrap();
            attrs.insert(attr.to_string(), val.to_string());
        }

        let mut elem = dom::element_node(tag_name.to_string(), attrs.clone());

        // add children recursively
        dbg!(attrs);
        elem
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
