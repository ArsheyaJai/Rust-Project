// this program loads student data from a CSV file into a graph structure, categorizes students by performance, and computes statistical and graph-based metrics (like mean attribute values and average path distances) to analyze differences between high and low performers
// aimed to construct a graph and perform BFS

// import necessary modules and functions
// import the student module for defining the student struct
mod student;
// import graph utility functions for graph-related operations
mod graph_utils;
// import CSV utilities for reading student data
mod csv_utils;

// function for graph computations
use graph_utils::{compute_avg_distance_sample, calculate_mean};
// function to load students from a CSV file
use csv_utils::load_students_from_csv;
// graph structures from the petgraph library
use petgraph::graph::{Graph, NodeIndex};
// import for handling errors
use std::error::Error;
use crate::student::Student;

// main function
fn main() -> Result<(), Box<dyn Error>> {
    // load student data from a CSV file
    // function returns a tuple (students, headers)
    let (students, _headers) = load_students_from_csv("Cleaned_for_graph_Students_Grading_Dataset.csv")?;

    // create an undirected graph to represent the student relationships
    let mut graph = Graph::new_undirected();
    // vectors to hold student nodes and catagorize them into high or low performers
    let mut student_nodes = Vec::new();
    let mut high_performers = Vec::new();
    let mut low_performers = Vec::new();

    // iterate through each student, adding them as nodes to the graph and categorize based on their score
    for student in &students {
        // add the student as a node
        let student_node = graph.add_node(student.clone());
        // store the node for later use
        student_nodes.push(student_node);

        // categorize the student based on their total score
        if student.total_score >= 60.0 {
            // high performer if score is greater than or equal to 60
            high_performers.push(student_node);
        } else {
            // low performer otherwise
            low_performers.push(student_node);
        }
    }

    // add edges between adjacent student nodes, forming a linear structure in the graph
    for i in 0..student_nodes.len() - 1 {
        graph.add_edge(student_nodes[i], student_nodes[i + 1], ());
    }

    // print graph construction details
    println!("Graph construction complete.");
    println!("Number of student nodes: {}", student_nodes.len());
    println!("Number of edges: {}", student_nodes.len() - 1);
    println!("High performers count: {}", high_performers.len());
    println!("Low performers count: {}", low_performers.len());

    // compute and print average distances for high and low performers
    // number of smaples to take for distance calculations
    let sample_size = 100;
    let avg_high_sample = compute_avg_distance_sample(&graph, &high_performers, sample_size);
    let avg_low_sample = compute_avg_distance_sample(&graph, &low_performers, sample_size);

    println!("Sampled average distance (high performers): {:.2}", avg_high_sample);
    println!("Sampled average distance (low performers): {:.2}", avg_low_sample);

    // calculate and print the mean values for various student features
    let high_attendance_mean = calculate_mean(&high_performers, &graph, |s| s.attendance);
    let low_attendance_mean = calculate_mean(&low_performers, &graph, |s| s.attendance);

    let high_parent_education_mean = calculate_mean(&high_performers, &graph, |s| s.parent_education);
    let low_parent_education_mean = calculate_mean(&low_performers, &graph, |s| s.parent_education);

    let high_family_income_mean = calculate_mean(&high_performers, &graph, |s| s.family_income);
    let low_family_income_mean = calculate_mean(&low_performers, &graph, |s| s.family_income);

    let high_stress_level_mean = calculate_mean(&high_performers, &graph, |s| s.stress_level);
    let low_stress_level_mean = calculate_mean(&low_performers, &graph, |s| s.stress_level);

    // output the calculated mean values for each feature
    println!("High Performers - Attendance Mean: {:.2}", high_attendance_mean);
    println!("Low Performers - Attendance Mean: {:.2}", low_attendance_mean);

    println!("High Performers - Parent Education: Mean {:.2}", high_parent_education_mean);
    println!("Low Performers - Parent Education: Mean {:.2}", low_parent_education_mean);

    println!("High Performers - Family Income: Mean {:.2}", high_family_income_mean);
    println!("Low Performers - Family Income: Mean {:.2}", low_family_income_mean);

    println!("High Performers - Stress Level: Mean {:.2}", high_stress_level_mean);
    println!("Low Performers - Stress Level: Mean {:.2}", low_stress_level_mean);

    Ok(())
}

// unit tests for the functions used in the program
#[cfg(test)]
mod tests {
    // import everything from the main module to use in tests
    use super::*;

    // test the calculate_mean function
    #[test]
    fn test_calculate_mean() {
        // create some sample students with different attributes
        let student1 = Student::new("1".to_string(), 70.0, 80.0, 2.0, 1.0, 4.0);
        let student2 = Student::new("2".to_string(), 85.0, 85.0, 3.0, 2.0, 3.0);
        let student3 = Student::new("3".to_string(), 90.0, 90.0, 2.5, 2.5, 2.5);

        // create a graph and add the students as nodes
        let mut graph = Graph::new_undirected();
        let s1 = graph.add_node(student1);
        let s2 = graph.add_node(student2);
        let s3 = graph.add_node(student3);

        let nodes = vec![s1, s2, s3];

        // calculate the mean attendance of the students in the nodes
        let mean_attendance = calculate_mean(&nodes, &graph, |s| s.attendance);
        // assert that the calculated mean is as expected
        assert_eq!(mean_attendance, 85.0);
    }

    // test the compute_avg_distance_sample function
    #[test]
    fn test_compute_avg_distance_sample() {
        // create some sample students
        let student1 = Student::new("1".to_string(), 70.0, 80.0, 2.0, 1.0, 4.0);
        let student2 = Student::new("2".to_string(), 85.0, 85.0, 3.0, 2.0, 3.0);
        let student3 = Student::new("3".to_string(), 90.0, 90.0, 2.5, 2.5, 2.5);

        // create a graph and add the students as nodes
        let mut graph = Graph::new_undirected();
        let s1 = graph.add_node(student1);
        let s2 = graph.add_node(student2);
        let s3 = graph.add_node(student3);

        // connect the nodes with edges
        graph.add_edge(s1, s2, ());
        graph.add_edge(s2, s3, ());

        // create a vector of nodes to calculate the average distance sample
        let nodes = vec![s1, s2, s3];
        // set the sample size for distance calculation
        let sample_size = 2;

        // compute the average distance for the given nodes and sample size
        let avg_distance = compute_avg_distance_sample(&graph, &nodes, sample_size);
        // assert that the average distance is greater than 0 (reasonable check)
        assert!(avg_distance > 0.0);
    }
}
