use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct WeightedGraph<T, W>
where
    T: Eq + Hash,
{
    adjacency_list: HashMap<T, HashMap<T, W>>,
}

impl<T, W> WeightedGraph<T, W>
where
    T: Eq + Hash,
{
    pub fn new() -> Self {
        WeightedGraph {
            adjacency_list: HashMap::new(),
        }
    }

    // Add a directed edge from `from` to `to` with a weight
    pub fn add_edge(&mut self, from: T, to: T, weight: W) {
        self.adjacency_list
            .entry(from)
            .or_insert_with(HashMap::new)
            .insert(to, weight);
    }

    // Check if there's an edge from `from` to `to`
    pub fn has_edge(&self, from: &T, to: &T) -> bool {
        self.adjacency_list
            .get(from)
            .map_or(false, |neighbors| neighbors.contains_key(to))
    }

    // Get the weight of the edge from `from` to `to`
    pub fn get_weight(&self, from: &T, to: &T) -> Option<&W> {
        self.adjacency_list
            .get(from)
            .and_then(|neighbors| neighbors.get(to))
    }

    // Get all neighbors of a given node along with their weights
    pub fn neighbors(&self, node: &T) -> Option<&HashMap<T, W>> {
        self.adjacency_list.get(node)
    }
}
