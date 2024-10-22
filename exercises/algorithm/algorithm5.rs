/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/

use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }
    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        let mut visit_order = Vec::new();
        let mut queue = VecDeque::new();
        let mut visited = vec![false; self.adj.len()];
        // Start with the initial node
        queue.push_back(start);
        visited[start] = true;
        // 从start开始寻找
        // 这个while let.....
        // 画了一个图，容易理解多了
        while let Some(current) = queue.pop_front() {
            visit_order.push(current);
            for neighbor in &self.adj[current] {
                if !visited[*neighbor] {
                    queue.push_back(*neighbor);
                    visited[*neighbor] = true;
                }
            }
        }
        visit_order
    }
    // Perform a breadth-first search on the graph, return the order of visited nodes
    // fn bfs_with_return(&self, start: usize) -> Vec<usize> {
    //     let mut visit_order = vec![];
    //     visit_order.push(start);
    //     // 遍历每一个节点,看他们可以到达哪里
    //     for i in 0..start{
    //             if self.adj[i][0] == start{
    //                 visit_order.push(i);
    //             }
    //     }
    //     visit_order
    // }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        
        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}

