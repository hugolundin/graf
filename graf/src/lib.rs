
#![allow(dead_code)]
use std::fs;

pub type NodeIndex = usize;
pub type EdgeIndex = usize;

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>
}

#[derive(Debug)]
pub struct Node {
    label: String,
    first_outgoing_edge: Option<NodeIndex>
}

#[derive(Debug)]
pub struct Edge {
    weight: i32,
    destination: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new()
        }
    }

    pub fn add_node(&mut self, label: String) -> NodeIndex {
        let index = self.nodes.len();

        self.nodes.push(Node {
            label: label,
            first_outgoing_edge: None
        });

        index
    }

    pub fn add_edge(
        &mut self,
        source: NodeIndex,
        destination: NodeIndex,
        weight: i32
    ) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];

        self.edges.push(Edge {
            weight: weight,
            destination: destination,
            next_outgoing_edge: node_data.first_outgoing_edge
        });

        node_data.first_outgoing_edge = Some(edge_index);
    }

    pub fn export(&self, filename: &str) {
        let mut result = "graph g {\n".to_string();

        for node in &self.nodes {
            result += &Graph::draw_node(node);

            if let Some(index) = node.first_outgoing_edge {
                result += &Graph::extract_edges(
                    &self,
                    node,
                    &self.edges[index]
                );
            }
        }

        result += "}";
        fs::write(filename, result).expect("Unable to write file");
    }

    fn extract_edges(&self, node: &Node, edge: &Edge) -> String {
        let result = Graph::draw_edge(node, &self.nodes[edge.destination]);

        if let Some(index) = edge.next_outgoing_edge {
            return result + &Graph::extract_edges(&self, node, &self.edges[index]);
        }

        result
    }

    fn draw_node(node: &Node) -> String {
        format!("    {}\n", node.label)
    }

    fn draw_edge(source: &Node, destination: &Node) -> String {
        format!("    {} -- {}\n", source.label, destination.label)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
