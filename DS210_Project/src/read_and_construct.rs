use csv::ReaderBuilder;
use std::{fs::File,io::{BufReader},};
use petgraph::{Graph};
use serde::Deserialize;


#[derive(Deserialize)]
pub struct Exercise {
    #[serde(rename = "Exercise Name")]
    workout: String,
    //set order is not used as weight on edge or nodes, it is just for preprosessing the data
    #[serde(rename = "Set Order")]
    order: i32,
    #[serde(rename = "Weight")]
    weight: f32,
}




impl Exercise {
    //using pet graph, put the read data into adjacency list and construct directed weighted graph.
    pub fn construct_graph() -> Graph<std::string::String, i32> {
        // Initialize a graph here
        let mut graph: Graph<String, i32> = Graph::new();
        let mut nodes: Vec<(String, petgraph::graph::NodeIndex)> = Vec::new();
        //read the data and preprocess it using two functions above their.
        let data_set = read_data();
        let validated_data_set = validate_data(data_set);


        //add nodes here

        for d in &validated_data_set {
            let exercise = &d.workout;
            let index = graph.add_node(exercise.to_string());
            nodes.push((exercise.to_string(), index));
        }

        //add edges here
        for v in 0..&validated_data_set.len() - 1 {
            let s = nodes.iter().find(|(name, _)| name == &validated_data_set[v].workout).unwrap().1;
            let t = nodes.iter().find(|(name, _)| name == &validated_data_set[v + 1].workout).unwrap().1;
            let w = validated_data_set[v].weight;
            graph.add_edge(s, t, w.round() as i32);
        }
        graph
    }
}

//using csv reader, read the data from the csv file.

pub fn read_data() -> Vec<Exercise> {
    let mut data_set = Vec::new();
    let file = File::open("weightlifting_721_workouts.csv").unwrap();
    let reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);

    for record in csv_reader.deserialize() {
        let record: Exercise = record.unwrap();
        if record.order > 0 && record.weight > 0.0 {
            data_set.push(record);
        }
    }
    data_set
}

pub fn validate_data(mut data_set: Vec<Exercise>) -> Vec<Exercise> {
    // Remove exercises with invalid data
    data_set.retain(|exercise| exercise.order > 0 && exercise.weight > 0.0);

    // Sort exercises by set order
    data_set.sort_by(|a, b| a.order.cmp(&b.order));

    // Normalize weight to a common scale because the data set has some typo in the weight field.
    let mut maximum = f32::NEG_INFINITY;
    for exercise in &data_set {
    if exercise.weight > maximum {
        maximum = exercise.weight;
    }
}

    for exercise in &mut data_set {
    exercise.weight = exercise.weight / maximum * 100.0;
}
    data_set
}