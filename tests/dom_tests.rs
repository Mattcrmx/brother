use brother::dom::{comment_node, pretty_print_tree, text_node, Node};

#[test]
fn test_pretty_print() {
    let mut root: Node = text_node(String::from("toto"));
    let mut c1: Node = text_node(String::from("titi"));
    let mut c2: Node = text_node(String::from("tata"));
    let mut c3: Node = text_node(String::from("tutu"));
    let c4: Node = text_node(String::from("tete"));
    let p1: Node = comment_node(String::from("zaza"));

    c3.add_child(c4);
    c2.add_child(c3);

    c1.add_child(p1);

    root.add_child(c1);
    root.add_child(c2);
    pretty_print_tree(root);
}
