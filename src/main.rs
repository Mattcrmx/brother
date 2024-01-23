pub mod dom;
use dom::{comment_node, pretty_print_tree, Node};

fn main() {
    let mut root: Node = comment_node(String::from("toto"));
    let c1: Node = comment_node(String::from("titi"));
    let mut c2: Node = comment_node(String::from("tata"));
    let mut c3: Node = comment_node(String::from("tutu"));
    let c4: Node = comment_node(String::from("tete"));

    c3.add_child(c4);
    c2.add_child(c3);

    root.add_child(c2);
    root.add_child(c1);

    pretty_print_tree(root);
}
