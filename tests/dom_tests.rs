use brother::dom::{pretty_print_tree, Node};

#[test]
fn test_pretty_print() {
    let mut root: Node = Node::text(String::from("toto"));
    let mut c1: Node = Node::text(String::from("titi"));
    let mut c2: Node = Node::text(String::from("tata"));
    let mut c3: Node = Node::text(String::from("tutu"));
    let c4: Node = Node::text(String::from("tete"));
    let p1: Node = Node::comment(String::from("zaza"));

    c3.add_child(c4);
    c2.add_child(c3);

    c1.add_child(p1);

    root.add_child(c1);
    root.add_child(c2);
    pretty_print_tree(&root);
}
