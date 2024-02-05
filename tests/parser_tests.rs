use brother::parser::parser;

#[test]
fn test_tag_name_parsing() {
    let test_string = "html";
    let mut test_parser = parser(test_string.to_string());
    let first_tag = test_parser.parse_tag_name();
    assert!(first_tag == "html");
}

#[test]
fn test_element_node_parsing() {
    let test_string = "<html>
    <body>
        <h1>Title</h1>
        <div id='main' class='test'>
            <p>Hello <em>world</em>!</p>
        </div>
    </body>
</html>";
    let mut test_parser = parser(test_string.to_string());

    test_parser.parse_element_node();
}

#[test]
fn test_element_node_simple() {
    let test_string = "<div>Toto</div>";
    let mut test_parser = parser(test_string.to_string());
    test_parser.parse_element_node();
}
