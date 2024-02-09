pub struct TextParser {
    position: usize,
    input: String,
}

impl TextParser {
    pub fn new(input: String) -> TextParser {
        TextParser { position: 0, input }
    }

    pub fn eol(&self) -> bool {
        self.position >= self.input.len()
    }

    pub fn _state(&self) -> &str {
        &self.input[self.position..]
    }

    pub fn starts_with(&self, s: &str) -> bool {
        self.input[self.position..].starts_with(s)
    }

    fn _ends_with(&self, s: &str) -> bool {
        self.input[self.position..].ends_with(s)
    }

    pub fn split_on(&self, separator: char) -> std::str::Split<'_, char> {
        self.input.split(separator)
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

        while !self.eol() && predicate(self.get_current_char()) {
            result.push(self.consume_char());
        }
        result
    }

    pub fn consume_sequence<F, G>(&mut self, predicate: F, discard: G, drop_last: bool) -> String
    where
        F: Fn(char) -> bool,
        G: Fn(char) -> bool,
    {
        let mut result = String::from("");

        while !self.eol() && predicate(self.get_current_char()) {
            let cur = self.consume_char();
            if !discard(cur) {
                result.push(cur);
            }
        }

        if drop_last {
            self.consume_char();
        }

        result
    }

    pub fn consume_pattern(&mut self, pat: String) -> String {
        // consume pattern and advances the position pointer
        let target_position = self.position + pat.len();
        assert!(self.input[self.position..target_position] == pat);

        while self.position < target_position {
            self.consume_char();
        }
        pat
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
        assert!(test_parser.input[test_parser.position..] == *"toto");
    }

    #[test]
    fn test_consume_char() {
        let mut test_parser = TextParser::new(String::from("toto"));
        assert!(test_parser.consume_char() == 't');
    }

    #[test]
    #[should_panic(expected = "Trying to consume character when end of input is reached")]
    fn test_panic_eol() {
        let mut test_parser = TextParser::new(String::from(""));
        test_parser.consume_char();
    }

    #[test]
    fn test_basic_operations() {
        let test_parser = TextParser::new(String::from("toto"));
        assert!(test_parser.get_current_char() == 't');
        assert!(test_parser.starts_with("tot"));
    }

    #[test]
    fn test_consum_while() {
        let mut test_parser = TextParser::new(String::from("toto: tata;"));
        assert!(test_parser.consume_chars_while(|c| c != ':') == "toto");
        test_parser.consume_char();
        assert!(test_parser.consume_chars_while(|c| c != ';') == " tata");
    }

    #[test]
    fn test_consume_with_mods() {
        let mut test_parser = TextParser::new(String::from("toto: tata;"));
        assert!(test_parser.consume_sequence(|c| c != ':', |c| c == 'o', true) == "tt");
    }
}
