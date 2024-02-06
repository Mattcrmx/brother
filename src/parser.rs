pub struct TextParser {
    position: usize,
    input: String,
}

impl TextParser {

    pub fn new(input: String) -> TextParser {
        TextParser {position: 0, input}
    }
    
    pub fn eol(&self) -> bool {
        self.position >= self.input.len()
    }

    fn _state(&self) -> &str {
        &self.input[self.position..]
    }

    pub fn starts_with(&self, s: &str) -> bool {
        self.input[self.position..].starts_with(s)
    }

    fn _ends_with(&self, s: &str) -> bool {
        self.input[self.position..].ends_with(s)
    }

    pub fn get_current_char(&self) -> char {
        self.input.chars().nth(self.position).unwrap()
    }

    pub fn consume_char(&mut self) -> char {
        if self.eol() {
            panic!("Trying to consume character when end of input is reached");
        }

        let mut char_iter = self.input[self.position..].chars();
        self.position += 1;

        char_iter.next().unwrap()
    }

    pub fn consume_chars_while<F>(&mut self, predicate: F) -> String
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

    pub fn remove_whitespaces(&mut self) {
        self.consume_chars_while(|c| c.is_whitespace());
    }

    pub fn parse_text_data(&mut self) -> String {
        self.consume_chars_while(|c| c != '<')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitespace_removal() {
        let mut test_parser = TextParser::new(String::from("    toto"));
        test_parser.remove_whitespaces();
        assert!(test_parser.input[test_parser.position..] == String::from("toto"));
    }
}
