/*
	dfs
	This problem requires you to implement a basic DFS traversal
*/

use std::collections::HashSet;
use std::collections::VecDeque;
struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src); 
    }
    // 这里AI使用了HashSet,访问逻辑存在区别
    // 对于v节点的数据进行访问,并且若邻居未访问,则递归访问
    fn dfs_util(&self, v: usize, visited: &mut HashSet<usize>, visit_order: &mut Vec<usize>) {
        //TODO
        // AI给的结果:
        // 标记当前节点为已访问
        visited.insert(v);
        // 将当前节点添加到访问顺序中
        visit_order.push(v);
        // 遍历当前节点的所有邻居
        for &neighbor in &self.adj[v] {
            // 如果邻居尚未访问，则递归访问
            if !visited.contains(&neighbor) {
                self.dfs_util(neighbor, visited, visit_order);
            }
        }
        // 这里我没用hashset,然后竟然也过了,奇怪
        // let mut vbool = vec![false;self.adj.len()];
        // let mut queue = VecDeque::new();
        // queue.push_back(v);
        // vbool[v] = true;
        // while let Some(current) = queue.pop_front(){
        //     visit_order.push(current);
        //     for neighbor in &self.adj[current] {
        //         if !vbool[*neighbor] {
        //             queue.push_back(*neighbor);
        //             vbool[*neighbor] = true;
        //         }
        //     }
        // }
    }

    // Perform a depth-first search on the graph, return the order of visited nodes
    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut visit_order = Vec::new(); 
        self.dfs_util(start, &mut visited, &mut visit_order);
        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_simple() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_dfs_with_cycle() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 3); 

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_dfs_disconnected_graph() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(3, 4); 

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]); 
        let visit_order_disconnected = graph.dfs(3);
        assert_eq!(visit_order_disconnected, vec![3, 4]); 
    }
}

