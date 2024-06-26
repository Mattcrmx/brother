pub mod assign;
pub mod css;
pub mod dom;
pub mod html;
pub mod parser;

use css::CSSParser;
use html::HTMLParser;

fn main() {
    // html
    let test_html = "<html>
    <body>
        <h1>Title</h1>
        <div id='main' class='test'>
            <p>Hello <em>world</em>!</p>
        </div>
    </body>
</html>";
    let mut html_parser = HTMLParser::new(test_html.to_string());
    let document = html_parser.parse_document();
    document.display();

    // css
    let test_stylesheet = "
    h1, h2, h3 { margin: auto; color: #cc0000; }
    div.note { margin-bottom: 20px; padding: 10px; }
    #answer { display: none; }
    ";
    let mut css_parser = CSSParser::new(test_stylesheet.to_string());
    let _stylesheet = css_parser.parse_stylesheet();
}
