mod centrality;
mod read_and_construct;
use centrality::btw_centrality;
use centrality::dijkstra_shr_path;
use petgraph::prelude::*;
use petgraph::visit::IntoNodeIdentifiers;



fn main() {
 
    let graph = read_and_construct::Exercise::construct_graph();
    let name = graph.node_identifiers().map(|node| graph.node_weight(node).unwrap().to_string()).collect::<Vec<_>>();
    let count=graph.node_count();
    let mut init_centrality: Vec<(usize, f64)> = Vec::with_capacity(count);
    for v_index in 0..graph.node_count() {
        let centrality = btw_centrality(&graph, NodeIndex::new(v_index))[v_index];
        init_centrality.push((v_index, centrality));
    }
        
    init_centrality.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    for (index, centerality_score) in init_centrality {
        println!("Node {}: Centrality {}", name[index], centerality_score);
    }

    // to see graph and shortest path respectively
    // println!("{:?}",graph);
    // println!("{:?}",dijkstra_shr_path(&graph, NodeIndex::new(0)))
    }


#[cfg(test)]

mod tests {

    use petgraph::{visit::IntoNodeIdentifiers, graph};

    use super::*;

    #[test]
    fn test_construct_graph() {
    let graph = read_and_construct::Exercise::construct_graph();
    let name = graph.node_identifiers().map(|node| graph.node_weight(node).unwrap().to_string()).collect::<Vec<_>>();
    let count=graph.node_count();
    let mut init_centrality: Vec<(usize, f64)> = Vec::with_capacity(count);
    for v_index in 0..graph.node_count() {
        let centrality = btw_centrality(&graph, NodeIndex::new(v_index))[v_index];
        init_centrality.push((v_index, centrality));
    }
        
    init_centrality.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Verify that the graph has the expected number of nodes and edges

        assert_eq!(graph.node_count(), 81);
        assert_eq!(graph.edge_count(), 9302);

    }
    #[test]
    fn test_centrality() {
        // Construct a test graph.
        let mut graph:Graph<String,i32>=Graph::new();
        let a = graph.add_node("1".to_string());
        let b = graph.add_node("2".to_string());
        let c = graph.add_node("3".to_string());
        let d = graph.add_node("4".to_string());
        let e = graph.add_node("5".to_string());
        graph.extend_with_edges(&[
            (a, b), (b, c), (c, d), (d, e), (a, c), (a, d), (b, d), (c, e)]);
        // Verify that the expected centrality and actual centrality are equal to each other
        let expected = vec![2.5, 2.0, 2.0, 1.0, 0.0];
        for (i, node) in graph.node_indices().enumerate() {
            let actual = btw_centrality(&graph, node);
            assert_eq!(actual[i], expected[i]);
        
}
}

}