#![allow(dead_code)]
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct Graph<T>
where
    T: Eq + Hash + Clone,
{
    adjacency_list: HashMap<T, HashSet<T>>,
}

impl<T> Graph<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    // Get all nodes in the graph
    pub fn nodes(&self) -> HashSet<T> {
        self.adjacency_list.keys().cloned().collect()
    }

    // Get all edges as (from, to) tuples
    pub fn edges(&self) -> Vec<(T, T)> {
        self.adjacency_list
            .iter()
            .flat_map(|(from, neighbors)| {
                let from = from.clone();
                neighbors.iter().map(move |to| (from.clone(), to.clone()))
            })
            .collect()
    }

    // Add a node without any edges
    pub fn add_node(&mut self, node: T) {
        self.adjacency_list.entry(node).or_insert_with(HashSet::new);
    }

    // Remove a node and all its edges
    pub fn remove_node(&mut self, node: &T) {
        self.adjacency_list.remove(node);
        self.adjacency_list.values_mut().for_each(|neighbors| {
            neighbors.remove(node);
        });
    }

    // Add a directed edge from `from` to `to`
    pub fn add_edge(&mut self, from: T, to: T) {
        self.adjacency_list
            .entry(from)
            .or_insert_with(HashSet::new)
            .insert(to);
    }

    // Remove the edge from `from` to `to`
    pub fn remove_edge(&mut self, from: &T, to: &T) {
        if let Some(neighbors) = self.adjacency_list.get_mut(from) {
            neighbors.remove(to);
            if neighbors.is_empty() {
                self.adjacency_list.remove(from);
            }
        }
    }

    // Check if there's an edge from `from` to `to`
    pub fn has_edge(&self, from: &T, to: &T) -> bool {
        self.adjacency_list
            .get(from)
            .map_or(false, |neighbors| neighbors.contains(to))
    }

    // Get all neighbors of a given node
    pub fn neighbors(&self, node: &T) -> Option<&HashSet<T>> {
        self.adjacency_list.get(node)
    }
}

#[derive(Debug, Clone)]
pub struct WeightedGraph<T, W>
where
    T: Eq + Hash + Clone,
{
    adjacency_list: HashMap<T, HashMap<T, W>>,
}

impl<T, W> WeightedGraph<T, W>
where
    T: Eq + Hash + Clone,
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

    // Remove a directed edge from `from` to `to`
    pub fn remove_edge(&mut self, from: &T, to: &T) {
        if let Some(neighbors) = self.adjacency_list.get_mut(from) {
            neighbors.remove(to);
            if neighbors.is_empty() {
                self.adjacency_list.remove(from);
            }
        }
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
