use petgraph::graph::{Graph, NodeIndex};
use std::cmp::Reverse;
use std::collections::{BinaryHeap};


type Distance = i64;




// This function computes betweenness centrality using helper function of dijkstra_algorithm to find the shortest path and count the number 
// of the shortes path. 
pub fn btw_centrality(graph: &Graph<String, i32>, start: NodeIndex) -> Vec<f64> {
    //calculate the shortest path
    let dist = dijkstra_shr_path(graph, start);

    
    let mut btw_centrality: Vec<f64> = vec![0.0; graph.node_count()];

    // Iterate all over the nodes so that it can count the number of shortest path to compute
    // centrality scores later using previously calculated the shortest path.
    for vertice in graph.node_indices() {
        let mut num_shortest = 0.0;

//checks if there is a shortest path between the starting node and the current node.
        if let Some(vertice_dist) = dist[vertice.index()] {
            for nbh in graph.neighbors_directed(vertice, petgraph::Direction::Incoming) {
                let neigh_dist = dist[nbh.index()];
                if let Some(neighbor_dist) = neigh_dist {
                    let weight = graph.edge_weight(graph.find_edge(nbh, vertice).unwrap()).unwrap();
                    // if distance is equalt to the shortest path, then there is a shortest path betweenn those nodes, 
                    // so we add 1 to num_shortest and also update the centrality score.
                    if vertice_dist == neighbor_dist + *weight as i64 {
                        num_shortest += 1.0;
                        btw_centrality[nbh.index()] += (1.0 + btw_centrality[vertice.index()]) / num_shortest;
                    }
                }
            }
        }
        // divide it by 2, b/c each shortest path counted twice
        if vertice != start {
            btw_centrality[vertice.index()] *= 0.5;
        }
    }

   return btw_centrality
}


// This function finds the shortest path between nodes from the start node given.
pub fn dijkstra_shr_path(graph: &Graph<String, i32>, start: NodeIndex) -> Vec<Option<Distance>> {
    
    let mut shr_dist: Vec<Option<Distance>> = vec![None; graph.node_count()];
    
    shr_dist[start.index()] = Some(0);

    //intialize priority queue to track nodes to visit and the starting node has the highest priority
    let mut pq = BinaryHeap::<Reverse<(Distance, NodeIndex)>>::new();
    pq.push(Reverse((0, start)));

    // by looping, make sure we visit all the nodes and check if we have already found a shorter path to the current node
    while let Some(Reverse((dist, v))) = pq.pop() {

        if let Some(current_dist) = shr_dist[v.index()] {
            if dist > current_dist {
                continue;
            }
        }

        // Loop over all neighbors of the current node to calculate the distance to the neigbor.
        for nbh in graph.neighbors_directed(v, petgraph::Direction::Outgoing) {
           
            let nbh_dist = graph.edge_weight(graph.find_edge(v, nbh).unwrap()).unwrap();

           
            let new_dist = dist + (*nbh_dist as i64);

            // update the distance if it is shorter than the previous one.
            let new_dis = match shr_dist[nbh.index()] {
                None => true,
                Some(d) => new_dist < d,
            };
            if new_dis {
                shr_dist[nbh.index()] = Some(new_dist);
                
                pq.push(Reverse((new_dist, nbh)));
            }
        }
    }

    
    return shr_dist
}