extern crate graf;
use graf::{Graph};

fn main() {
    let mut g = Graph::new();

    let a = g.add_node(String::from("A"));
    let b = g.add_node(String::from("B"));
    let c = g.add_node(String::from("C"));
    let d = g.add_node(String::from("D"));

    g.add_edge(a, b, 1);
    g.add_edge(a, c, 1);
    g.add_edge(a, d, 1);

    // println!("{:#?}", g);
    g.export("test.dot");
}