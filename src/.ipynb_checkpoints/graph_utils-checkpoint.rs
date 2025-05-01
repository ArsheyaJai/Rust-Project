// this module provides utility functions to analyze a graph of Student nodes, including computing the average shortest-path distance between sampled nodes and calculating the mean of a selected student attribute within a group.

// import necessary types for graph manipulation and other utilities
// import the Graph and NodeIndex types from petgraph for graph manipulation
use petgraph::{graph::{Graph, NodeIndex}};
// import HashMap for storing visited nodes and VecDeque for implementing BFS
use std::collections::HashMap;
// VecDeque for queue data structure, useful in the BFS algorithm
use std::collections::VecDeque;
// import the Student struct from a local module, likely to represent student-related data
use crate::student::Student;

// function to compute the average distance between a random sample of nodes in the graph
pub fn compute_avg_distance_sample(
    // the graph to perform the distance computation on
    graph: &Graph<Student, (), petgraph::Undirected>,
    // the group of nodes (students) to sample from
    group: &Vec<NodeIndex>,
    // the number of nodes to randomly sample
    sample_size: usize,
) -> f64 {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    let mut rng = thread_rng();
    // randomly select a sample of nodes from the group
    let sample: Vec<_> = group
        .choose_multiple(&mut rng, sample_size)
        .cloned()
        .collect();

    // accumulated total distance between sampled nodes
    let mut total_distance = 0;
    // counter for valid distances
    let mut count = 0;

    // perform BFS from each source node in the sample
    for &source in &sample {
        // track visited nodes and their distances
        let mut visited = HashMap::new();
        // queue for BFS
        let mut queue = VecDeque::new();

        // start with the source node
        visited.insert(source, 0);
        queue.push_back(source);

        // perform BFS to compute distances
        while let Some(current) = queue.pop_front() {
            let distance = visited[&current];
            for neighbor in graph.neighbors(current) {
                if !visited.contains_key(&neighbor) {
                    visited.insert(neighbor, distance + 1);
                    queue.push_back(neighbor);
                }
            }
        }

        // for each target node in the sample, compute the distance to the source
        for &target in &sample {
            if source != target {
                if let Some(&d) = visited.get(&target) {
                    // add distance to total
                    total_distance += d;
                    // increment valid distance count
                    count += 1;
                }
            }
        }
    }

    // return the average distance, or 0 if no distances were computed 
    if count > 0 {
        total_distance as f64 / count as f64
    } else {
        0.0
    }
}

// calculate the mean of a particular attribute for the students in the provided group
pub fn calculate_mean<F>(
    // list of nodes to calculate mean for
    nodes: &Vec<NodeIndex>,
    // the graph of students
    graph: &Graph<Student, (), petgraph::Undirected>,
    // function to select the attribute from a student
    selector: F,
) -> f64
where
    // the selector function type
    F: Fn(&Student) -> f64,
{
    // sum up the selected attribute for all students in the group
    let total: f64 = nodes.iter()
        // apply selector to each student
        .map(|&node| selector(&graph[node]))
        .sum();

    // return the mean by dividing the total by the number of nodes
    total / nodes.len() as f64
}
