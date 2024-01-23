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

    pub fn consume(&mut self) -> char {
        
    }
}
