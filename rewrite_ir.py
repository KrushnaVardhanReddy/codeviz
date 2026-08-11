import sys

def run():
    with open("codeviz-core/src/ir.rs", "r") as f:
        ir_content = f.read()

    new_methods = """
    /// Returns all paths (up to `max_paths`) from `start_node_id` to `target_node_id`.
    /// Paths follow the `Calls` edge kind.
    pub fn all_paths(&self, start_node_id: &str, target_node_id: &str, max_paths: usize) -> Vec<Vec<String>> {
        let mut paths = Vec::new();
        let mut current_path = vec![start_node_id.to_string()];
        let mut visited = std::collections::HashSet::new();
        visited.insert(start_node_id.to_string());
        
        self.dfs_all_paths(
            start_node_id,
            target_node_id,
            max_paths,
            &mut current_path,
            &mut visited,
            &mut paths,
        );
        
        paths
    }

    #[allow(clippy::collapsible_if)]
    fn dfs_all_paths(
        &self,
        current_node_id: &str,
        target_node_id: &str,
        max_paths: usize,
        current_path: &mut Vec<String>,
        visited: &mut std::collections::HashSet<String>,
        paths: &mut Vec<Vec<String>>,
    ) {
        if paths.len() >= max_paths {
            return;
        }

        if current_node_id == target_node_id {
            paths.push(current_path.clone());
            return;
        }

        for edge in &self.edges {
            if edge.from_id == current_node_id && edge.kind == EdgeKind::Calls {
                if !visited.contains(&edge.to_id) {
                    visited.insert(edge.to_id.clone());
                    current_path.push(edge.to_id.clone());
                    
                    self.dfs_all_paths(
                        &edge.to_id,
                        target_node_id,
                        max_paths,
                        current_path,
                        visited,
                        paths,
                    );
                    
                    current_path.pop();
                    visited.remove(&edge.to_id);
                }
            }
        }
    }

    /// Returns the recursive caller tree up to `max_depth`.
    pub fn callers_recursive(&self, target_node_id: &str, max_depth: usize) -> serde_json::Value {
        let mut visited_path = std::collections::HashSet::new();
        visited_path.insert(target_node_id.to_string());
        self.build_callers_tree(target_node_id, max_depth, 0, &mut visited_path)
    }

    #[allow(clippy::collapsible_if)]
    fn build_callers_tree(
        &self,
        current_node_id: &str,
        max_depth: usize,
        current_depth: usize,
        visited_path: &mut std::collections::HashSet<String>,
    ) -> serde_json::Value {
        if current_depth >= max_depth {
            return serde_json::json!({
                "node": current_node_id,
                "callers": []
            });
        }

        let mut callers_list = Vec::new();

        for edge in &self.edges {
            if edge.to_id == current_node_id && edge.kind == EdgeKind::Calls {
                if !visited_path.contains(&edge.from_id) {
                    visited_path.insert(edge.from_id.clone());
                    
                    let caller_tree = self.build_callers_tree(
                        &edge.from_id,
                        max_depth,
                        current_depth + 1,
                        visited_path,
                    );
                    callers_list.push(caller_tree);
                    
                    visited_path.remove(&edge.from_id);
                }
            }
        }

        serde_json::json!({
            "node": current_node_id,
            "callers": callers_list
        })
    }

    /// Returns all transitively reachable nodes from the given node.
    #[allow(clippy::collapsible_if)]
    pub fn blast_radius(&self, start_node_id: &str) -> Vec<String> {
        let mut reachable = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        
        queue.push_back(start_node_id.to_string());
        
        while let Some(current) = queue.pop_front() {
            for edge in &self.edges {
                if edge.from_id == current && edge.kind == EdgeKind::Calls {
                    if !reachable.contains(&edge.to_id) && edge.to_id != start_node_id {
                        reachable.insert(edge.to_id.clone());
                        queue.push_back(edge.to_id.clone());
                    }
                }
            }
        }
        
        reachable.into_iter().collect()
    }
"""

    if "pub fn all_paths" not in ir_content:
        ir_content = ir_content.replace(
            "impl CodeGraph {\n    /// Creates a new empty CodeGraph.",
            "impl CodeGraph {\n" + new_methods + "\n    /// Creates a new empty CodeGraph."
        )
        with open("codeviz-core/src/ir.rs", "w") as f:
            f.write(ir_content)
    
    with open("codeviz-core/src/graph.rs", "r") as f:
        graph_content = f.read()

    # The code was duplicated in graph.rs, meaning it also has an `impl CodeGraph` block we need to remove completely,
    # or just remove the methods we added if `impl CodeGraph` was originally there.
    # Looking at the earlier checkout, `graph.rs` might not have had `impl CodeGraph` originally. Let's do a hard reset of it
    
    pass

run()
