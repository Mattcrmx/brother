use brother::html::HTMLParser;

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
    let mut test_parser = HTMLParser::new(test_string.to_string());
    let document = test_parser.parse_document();
    document.display();
}
