extern crate graf;
use graf::{Graph};

fn main() {
    let mut g = Graph::new();

    let a = g.add_node(String::from("A"));
    let b = g.add_node(String::from("B"));
    let c = g.add_node(String::from("C"));

    g.add_edge(a, b, 0);
    g.add_edge(a, c, 0);
    g.add_edge(b, c, 0);

    println!("{:#?}", g);
    g.export("test.dot");
}