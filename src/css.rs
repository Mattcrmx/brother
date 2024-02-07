use std::{u8, f32};
use crate::parser::TextParser;
struct Stylesheet {
    rules: Vec<Rule>,
}

struct Rule {
    selectors: Vec<Selector>,
    declarations: Vec<Declaration>,
}

enum Selector {
    Simple(SimpleSelector),
}

struct SimpleSelector {
    tag_name: Option<String>,
    id: Option<String>,
    class: Vec<String>,
}

struct Declaration {
    name: String,
    value: Value,
}

enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
}

enum Unit {
    Px,
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

struct CSSParser {
    text_parser: TextParser
}


impl Color {
    fn from_hex_code(code: String) -> Color {
        // transform code string to color
        let r = u8::from_str_radix(&code[1..3], 16).unwrap();
        let g = u8::from_str_radix(&code[3..5], 16).unwrap();
        let b = u8::from_str_radix(&code[5..7], 16).unwrap();

        Color { r, g, b }
    }
}



impl CSSParser {

    pub fn new(input: String) -> CSSParser {
        let text_parser = TextParser::new(input);
        CSSParser {text_parser}
    }

    fn parse_declaration(&mut self) -> Declaration {
        // parse declaration, char by char

        let name = self.text_parser.consume_chars_while(|c| c != ':');
        self.text_parser.remove_whitespaces();
        let value = self.text_parser.consume_chars_while(|c| c != ';' || c != '}');
        let first_char = value.chars().next().unwrap();

        // test first character to see which type of value we'll return
        if first_char == '#' {
            Declaration {name, value: Value::ColorValue(Color::from_hex_code(value))}
        } else if first_char.is_digit(10) {
            let mut qty = String::from("");
            // let mut split_idx = 0;

            for c in value.chars() {
                if c.is_digit(10) {
                    qty.push(c);
                    // split_idx += 1;
                } else {
                    break;
                }
            }
            
            // TODO: support multiple units
            // let unit = &value[split_idx..];

            Declaration {name, value: Value::Length(qty.parse::<f32>().unwrap(), Unit::Px)}
        } else {
            Declaration{name, value: Value::Keyword(value)}
        }

    }

    // fn parse_rule(&mut self) -> Rule {
    //     assert!(self.text_parser.consume_char() == '{');
    //     self.text_parser.remove_whitespaces();

    //     let declarations = Vec::new();

    //     while self.text_parser.get_current_char() != '}' {
    //         declarations.push(self.parse_declaration());
    //     }

    //     Rule { selectors: (), declarations: declarations }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_declaration_parsing() {

        let test_input = "{ margin: auto; }";
        let mut css_parser = CSSParser::new(test_input.to_string());
        let test_declaration = css_parser.parse_declaration();
        let validation_declaration = Declaration{name: "margin".to_string(), value: Value::Keyword("auto".to_string())};
        assert!(test_declaration.name == validation_declaration.name);
    }

}