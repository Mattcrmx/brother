use brother::{dom::pretty_print_tree, parser::parser};

#[test]
fn test_document_parsing() {
    let test_string = "<html>
    <body>
        <h1>Title</h1>
        <div id='main' class='test'>
            <p>Hello <em>world</em>!</p>
        </div>
    </body>
</html>";
    let mut test_parser = parser(test_string.to_string());
    let root = test_parser.parse_document();
    pretty_print_tree(root)
}
