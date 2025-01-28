# DS210 Final Project: Analyzing Weightlifting Patterns Using Betweenness Centrality

This project uses **graph theory** and **betweenness centrality** to analyze patterns in weightlifting routines. By constructing a directed, weighted graph from the dataset, the project identifies exercises that play a critical role in improving performance.

---

##  About the Project

The goal of this project is to analyze the **weightlifting dataset** and identify influential exercises based on their relationships with others. Using Dijkstra's algorithm and a betweenness centrality measure, the project evaluates the importance of different exercises and their connections to improved performance.

---

##  Project Structure

### Key Components
- **Dataset**: Contains weightlifting data with exercise types, weights lifted, sets/reps, and dates.
- **Algorithms**: Implements Dijkstra's algorithm for shortest paths and a betweenness centrality algorithm to measure the influence of exercises.
- **Findings**: Discusses unexpected results, including the Hammer High Row exercise having the highest centrality score.

---

##  Dataset Overview

### **Source**
- Dataset URL: [Weightlifting Dataset](https://www.kaggle.com/datasets/joep89/weightlifting)
- Description:
  - Tracks a single individual's weightlifting routine.
  - Includes exercise types, weights, sets/reps, and performance metrics.

### **Motivation**
As a gym enthusiast, the goal is to explore training strategies and uncover patterns that can enhance performance.

---

##  Analysis and Methodology

### **Algorithms**
1. **Dijkstra's Algorithm**:
   - Calculates the shortest path between nodes in the graph.
   - Complexity: \(O((E+V)\log V)\), where \(E\) is the number of edges and \(V\) is the number of vertices.

2. **Betweenness Centrality**:
   - Measures the influence of nodes (exercises) in connecting other nodes.
   - Complexity: \(O(V(E+V)\log V)\).

### **Steps**
1. Read the dataset (`CSV` file).
2. Construct a directed weighted graph using **PetGraph**.
3. Apply Dijkstra's algorithm to compute shortest paths.
4. Compute betweenness centrality for each exercise.
5. Identify exercises with the highest centrality scores.

---

##  Key Findings

### **Interesting Insights**
- **Hammer High Row** has the highest centrality score (213.62), indicating its potential influence on overall performance.
- Traditional exercises like **Squat** and **Deadlift** also have high centrality scores (105.79 and 113.79, respectively).
- Unexpectedly, Hammer High Row surpassed Deadlift and Squat, implying it may play a unique role in improving performance.

### **Challenges**
- Some graph nodes with redundant names or weights caused `None` or `0` values in the shortest path outputs.
- Certain high-centrality exercises aligned with intuition, but not all matched expectations.

---



 **Outputs**
   - Centrality scores for each exercise.
   - Graph visualizations representing the dataset.

---

##  Conclusion

While the results provided new insights, some outcomes like the dominance of Hammer High Row were unexpected. Exploring subgraphs and additional metrics highlighted the importance of traditional exercises like Squat and Deadlift.

---

##  Resources and References

- **Dataset**: [Kaggle Weightlifting Dataset](https://www.kaggle.com/datasets/joep89/weightlifting)
- **Documentation**:
  - [PetGraph](https://docs.rs/petgraph/latest/petgraph/)
  - [RustWorkX](https://docs.rs/rustworkx-core/latest/rustworkx_core/)
- **Algorithms**:
  - [Betweenness Centrality](https://www.nebula-graph.io/posts/how-to-compute-betweenness-centrality-aganist-nebula-graph)
  - [Dijkstra's Algorithm](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm)


