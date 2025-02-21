/*
  dfs
  This problem requires you to implement a basic DFS traversal
*/

use std::collections::HashSet;

struct Graph {
  ls_adj: Vec<Vec<usize>>,
}

impl Graph {
  fn new(n: usize) -> Self {
    Graph { ls_adj: vec![vec![]; n] }
  }

  fn add_edge(&mut self, src: usize, dest: usize) {
    self.ls_adj[src].push(dest);
    self.ls_adj[dest].push(src);
  }

  fn dfs_util(&self, v: usize, visited: &mut HashSet<usize>, visit_ord: &mut Vec<usize>) {
    if !visited.insert(v) {
      return;
    }
    visit_ord.push(v);

    for &nbr in &self.ls_adj[v] {
      self.dfs_util(nbr, visited, visit_ord);
    }
  }

  fn dfs(&self, ent: usize) -> Vec<usize> {
    let mut visited = HashSet::new();
    let mut visit_order = Vec::new();
    self.dfs_util(ent, &mut visited, &mut visit_order);
    visit_order
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_dfs_simple() {
    let mut graph = Graph::new(3);
    graph.add_edge(0, 1);
    graph.add_edge(1, 2);

    let visit_order = graph.dfs(0);
    assert_eq!(visit_order, vec![0, 1, 2]);
  }

  #[test]
  fn test_dfs_with_cycle() {
    let mut graph = Graph::new(4);
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 2);
    graph.add_edge(2, 3);
    graph.add_edge(3, 3);

    let visit_order = graph.dfs(0);
    assert_eq!(visit_order, vec![0, 1, 2, 3]);
  }

  #[test]
  fn test_dfs_disconnected_graph() {
    let mut graph = Graph::new(5);
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(3, 4);

    let visit_order = graph.dfs(0);
    assert_eq!(visit_order, vec![0, 1, 2]);
    let visit_order_disconnected = graph.dfs(3);
    assert_eq!(visit_order_disconnected, vec![3, 4]);
  }
}
