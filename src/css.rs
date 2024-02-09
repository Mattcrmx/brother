use crate::parser::TextParser;
use std::{f32, u8};

#[derive(Debug, Clone)]
struct Stylesheet {
    rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
struct Rule {
    selectors: Vec<SimpleSelector>,
    declarations: Vec<Declaration>,
}

#[derive(Debug, Clone)]
struct SimpleSelector {
    tag_name: Option<String>,
    id: Option<String>,
    class: Vec<String>,
}

#[derive(Debug, Clone)]
struct Declaration {
    name: String,
    value: Value,
}

#[derive(Debug, Clone)]
enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
}

#[derive(Debug, Clone)]
enum Unit {
    Px,
}

#[derive(Debug, Clone)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

pub struct CSSParser {
    text_parser: TextParser,
}

impl Color {
    fn from_hex_code(code: String) -> Color {
        // transform code string to color
        let convert_to_u8 = |s| u8::from_str_radix(s, 16).unwrap();

        let r = convert_to_u8(&code[1..3]);
        let g = convert_to_u8(&code[3..5]);
        let b = convert_to_u8(&code[5..7]);

        Color { r, g, b }
    }
}

impl Declaration {
    fn new(name: String, value: String) -> Declaration {
        let first_char = value.chars().next().unwrap();

        // test first character to see which type of value we'll return
        if first_char == '#' {
            Declaration {
                name,
                value: Value::ColorValue(Color::from_hex_code(value)),
            }
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

            Declaration {
                name,
                value: Value::Length(qty.parse::<f32>().unwrap(), Unit::Px),
            }
        } else {
            Declaration {
                name,
                value: Value::Keyword(value),
            }
        }
    }
}

impl CSSParser {
    pub fn new(input: String) -> CSSParser {
        let text_parser = TextParser::new(input);
        CSSParser { text_parser }
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        // assume that the input has been sanitized before
        let mut declarations: Vec<Declaration> = Vec::new();

        while !self.text_parser.eol() {
            match self.text_parser.get_current_char() {
                '}' => break,
                ' ' => {
                    self.text_parser.consume_char();
                }
                _ => {
                    let name = self
                        .text_parser
                        .consume_sequence(|c| c != ':', |c| c == ' ', true);

                    let value = self
                        .text_parser
                        .consume_sequence(|c| c != ';', |c| c == ' ', true);

                    declarations.push(Declaration::new(name, value));
                }
            }
        }
        declarations
    }

    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new(),
        };

        while !self.text_parser.eol() {
            match self.text_parser.get_current_char() {
                '#' => {
                    // id
                    self.text_parser.consume_char();
                    selector.id = Some(self.text_parser.consume_sequence(
                        |c| (c != ',' && c != '.'),
                        |c| c == ' ',
                        false,
                    ));
                }
                '.' => {
                    // class
                    self.text_parser.consume_char();
                    selector.class.push(self.text_parser.consume_sequence(
                        |c| c != ',',
                        |c| c == ' ',
                        false,
                    ));
                }
                ' ' => {
                    // space separation
                    self.text_parser.consume_char();
                }
                '{' | ',' => {
                    // start the declaration
                    break;
                }
                _ => {
                    // otherwise tag name
                    selector.tag_name = Some(
                        self.text_parser
                            .consume_chars_while(|c| c.is_alphanumeric()),
                    );
                }
            }
        }

        selector
    }

    fn parse_selectors(&mut self) -> Vec<SimpleSelector> {
        let mut selectors: Vec<SimpleSelector> = Vec::new();

        while !self.text_parser.eol() {
            match self.text_parser.get_current_char() {
                ',' => {
                    self.text_parser.consume_char();
                    self.text_parser.remove_whitespaces();
                }
                '{' => break,
                _ => {
                    selectors.push(self.parse_simple_selector());
                }
            }
        }

        selectors
    }

    fn parse_rule(&mut self) -> Rule {
        let selectors: Vec<SimpleSelector> = self.parse_selectors();
        self.text_parser.remove_whitespaces();
        assert!(self.text_parser.consume_char() == '{');
        let declarations: Vec<Declaration> = self.parse_declarations();

        Rule {
            selectors,
            declarations,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_declaration_parsing() {
        let test_input = "margin: auto; titi: toto;";
        let mut css_parser = CSSParser::new(test_input.to_string());
        let test_declarations = css_parser.parse_declarations();
        let decl1 = Declaration {
            name: "margin".to_string(),
            value: Value::Keyword("auto".to_string()),
        };

        let decl2 = Declaration {
            name: "titi".to_string(),
            value: Value::Keyword("toto".to_string()),
        };
        assert!(test_declarations.get(0).unwrap().name == decl1.name);
        assert!(test_declarations.get(1).unwrap().name == decl2.name);
    }

    #[test]
    fn test_selector_parsing() {
        let test_input = "h1, p, div.toto, #param.sasa";
        let mut css_parser = CSSParser::new(test_input.to_string());
        let test_selectors = css_parser.parse_selectors();
        dbg!(test_selectors);
    }

    #[test]
    fn test_rule_parsing() {
        let test_input = "h1, h2, h3 { margin: auto; color: #cc0000; }";
        let mut css_parser = CSSParser::new(test_input.to_string());
        let rule = css_parser.parse_rule();
        dbg!(rule);
    }
}
