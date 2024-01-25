use std::result;

struct HTMLParser {
    position: usize,
    input: String,
}

impl HTMLParser {
    pub fn eol(&self) -> bool {
        self.position >= self.input.len()
    }

    pub fn get_next_char(&self) -> Option<char> {
        // return None if we reach the end of the word
        // doesn't consume the char
        if !self.eol() {
            Some(self.input.chars().nth(self.position + 1).unwrap())
        } else {
            None
        }
    }

    pub fn peek(&self, offset: usize) -> Option<char> {
        let new_position: usize = self.position + offset;
        if new_position < self.input[self.position..].len() {
            Some(self.input.chars().nth(new_position).unwrap())
        } else {
            None
        }
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
        where F: Fn(char) -> bool {
            let mut result = String::from("");
            let mut cons: bool = true;

            // loop through the input
            while !self.eol()  && cons {
                let next_char = self.get_next_char();
                cons = match next_char {
                    Some(c) => predicate(c),
                    None => false
                };
                result.push(self.consume_char());
            }
            return result
        }

    pub fn remove_whitespaces(&mut self) {
        self.consume_chars_while(|c| c.is_whitespace());
    }
}
