use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Graph {
    adjacency_list: HashMap<i32, HashSet<i32>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    // Add a directed edge from `from` to `to`
    pub fn add_edge(&mut self, from: i32, to: i32) {
        self.adjacency_list
            .entry(from)
            .or_insert_with(HashSet::new)
            .insert(to);
    }

    // Check if there's an edge from `from` to `to`
    pub fn has_edge(&self, from: i32, to: i32) -> bool {
        if let Some(neighbors) = self.neighbors(from) {
            neighbors.contains(&to)
        } else {
            false
        }
    }

    // Get all neighbors of a given node
    pub fn neighbors(&self, node: i32) -> Option<&HashSet<i32>> {
        self.adjacency_list.get(&node)
    }
}
