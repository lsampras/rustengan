use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct NeighborHood {
    topology: HashMap<String, Vec<String>>,
    node_count: usize,
    interval: usize,
    leaders: Vec<String>,
    nodes: Vec<String>,
}


// Currently this neighborhood uses a naive algorithm for choosing sqrt(n) regional leaders for max 2 hop propogation
// This assumes that each node is at a fixed distance from each other which isn't true for real world conditions
impl NeighborHood {
    pub fn create(topology: HashMap<String, Vec<String>>) -> Self {
        let interval = f64::sqrt(topology.len() as f64).floor() as usize;
        let mut nodes: Vec<String> = topology.keys().cloned().collect();
        nodes.sort();
        let leaders = nodes.iter().step_by(interval).cloned().collect();
        Self {
            topology,
            node_count: nodes.len(),
            interval,
            leaders,
            nodes 
        }
    }

    pub fn get_neighbours(&self, source: &String) -> Vec<String> {
        if let Ok(position) = self.nodes.binary_search(source) {

            if position%self.interval == 0 {
                let start = position;
                let end = std::cmp::min(start + self.interval, self.node_count);
                let mut child = self.nodes.get(start..end).expect(&format!("neighbours not found for {start} and {end}")).to_vec();
                child.append(&mut self.leaders.clone());
                child
    
            } else {
                self.leaders.clone()
            }
        } else {
            vec![]
        }
    }
}