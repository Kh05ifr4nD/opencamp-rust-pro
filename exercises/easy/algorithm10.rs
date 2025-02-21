/*
  graph
  This problem requires you to implement a basic graph
*/

use std::collections::{HashMap, HashSet};
use std::fmt;
#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "accessing a node that is not in the graph")
  }
}

pub trait Graph {
  fn new() -> Self;
  fn tbl_adj(&self) -> &HashMap<String, Vec<(String, i32)>>;
  fn tbl_adj_mut(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
  fn add_edge(&mut self, edge: (&str, &str, i32));
  fn add_node(&mut self, node: &str) -> bool;
  fn contain(&self, node: &str) -> bool;
  fn set_node(&self) -> HashSet<&String>;
  fn ls_edge(&self) -> Vec<(&String, &String, i32)>;
}

pub struct UndrctedGraph {
  tbl_adj: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for UndrctedGraph {
  fn new() -> UndrctedGraph {
    UndrctedGraph { tbl_adj: HashMap::new() }
  }

  fn tbl_adj(&self) -> &HashMap<String, Vec<(String, i32)>> {
    &self.tbl_adj
  }

  fn tbl_adj_mut(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
    &mut self.tbl_adj
  }

  fn add_edge(&mut self, edge: (&str, &str, i32)) {
    let (from, to, weight) = edge;
    let from_ent = self.tbl_adj_mut().entry(from.to_string()).or_default();
    from_ent.push((to.to_string(), weight));

    let to_entry = self.tbl_adj_mut().entry(to.to_string()).or_default();
    to_entry.push((from.to_string(), weight));
  }

  fn add_node(&mut self, node: &str) -> bool {
    let str_node = node.to_string();
    if self.contain(node) {
      return false;
    }
    self.tbl_adj_mut().insert(str_node, Vec::new());
    true
  }

  fn contain(&self, node: &str) -> bool {
    self.tbl_adj().get(node).is_some()
  }

  fn set_node(&self) -> HashSet<&String> {
    self.tbl_adj().keys().collect()
  }

  fn ls_edge(&self) -> Vec<(&String, &String, i32)> {
    let mut ls_edge = Vec::new();
    for (node_from, ls_nbr) in self.tbl_adj() {
      for (node_to, wt) in ls_nbr {
        ls_edge.push((node_from, node_to, *wt));
      }
    }
    ls_edge
  }
}

#[cfg(test)]
mod test_undirected_graph {
  use super::Graph;
  use super::UndrctedGraph;
  #[test]
  fn test_add_edge() {
    let mut graph = UndrctedGraph::new();
    graph.add_edge(("a", "b", 5));
    graph.add_edge(("b", "c", 10));
    graph.add_edge(("c", "a", 7));
    let expected_edges = [
      (&String::from("a"), &String::from("b"), 5),
      (&String::from("b"), &String::from("a"), 5),
      (&String::from("c"), &String::from("a"), 7),
      (&String::from("a"), &String::from("c"), 7),
      (&String::from("b"), &String::from("c"), 10),
      (&String::from("c"), &String::from("b"), 10),
    ];
    for edge in &expected_edges {
      assert!(graph.ls_edge().contains(edge));
    }
  }
}
