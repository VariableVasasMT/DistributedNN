use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use rand::Rng;

use crate::threshold_node::ThresholdGatingNode;
use crate::memory::ClusterMemory;

// Import the console_log macro
use crate::console_log;

/// Device-level cluster managing multiple threshold-gating nodes
/// Implements local coordination, specialization, and memory management
#[wasm_bindgen]
#[derive(Clone)]
pub struct DeviceCluster {
    cluster_id: String,
    nodes: HashMap<String, ThresholdGatingNode>,
    cluster_memory: ClusterMemory,
    topology: NetworkTopology,
    current_time: f64,
    
    // Error propagation and coordination
    global_error: f64,
    local_modulatory_signal: f64,
    
    // Dynamic topology parameters
    node_split_threshold: f64,
    edge_duplication_threshold: f64,
    pruning_threshold: f64,
    
    // Specialization tracking
    specialization_scores: HashMap<String, f64>,
    node_usage_stats: HashMap<String, u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkTopology {
    pub connections: HashMap<String, Vec<String>>, // node_id -> connected_node_ids
    pub edge_weights: HashMap<(String, String), f64>,
    pub edge_usage: HashMap<(String, String), u32>,
}

impl NetworkTopology {
    pub fn new() -> Self {
        NetworkTopology {
            connections: HashMap::new(),
            edge_weights: HashMap::new(),
            edge_usage: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node_id: String) {
        self.connections.insert(node_id, Vec::new());
    }

    pub fn connect_nodes(&mut self, from: String, to: String, weight: f64) {
        self.connections.entry(from.clone()).or_default().push(to.clone());
        self.edge_weights.insert((from, to), weight);
    }

    pub fn record_edge_usage(&mut self, from: &str, to: &str) {
        let key = (from.to_string(), to.to_string());
        *self.edge_usage.entry(key).or_insert(0) += 1;
    }

    pub fn get_connections(&self, node_id: &str) -> Vec<String> {
        self.connections.get(node_id).cloned().unwrap_or_default()
    }
}

#[wasm_bindgen]
impl DeviceCluster {
    #[wasm_bindgen(constructor)]
    pub fn new(cluster_id: String, num_initial_nodes: usize) -> DeviceCluster {
        let mut cluster = DeviceCluster {
            cluster_id: cluster_id.clone(),
            nodes: HashMap::new(),
            cluster_memory: ClusterMemory::new(cluster_id.clone()),
            topology: NetworkTopology::new(),
            current_time: 0.0,
            global_error: 0.0,
            local_modulatory_signal: 0.0,
            node_split_threshold: 10.0,
            edge_duplication_threshold: 5.0,
            pruning_threshold: 0.1,
            specialization_scores: HashMap::new(),
            node_usage_stats: HashMap::new(),
        };

        // Create initial nodes with random topology
        cluster.initialize_nodes(num_initial_nodes);
        cluster
    }

    fn initialize_nodes(&mut self, num_nodes: usize) {
        let mut rng = rand::thread_rng();
        
        for i in 0..num_nodes {
            let node_id = format!("{}_node_{}", self.cluster_id, i);
            let node = ThresholdGatingNode::new(node_id.clone(), 4); // 4 input connections
            
            self.nodes.insert(node_id.clone(), node);
            self.topology.add_node(node_id.clone());
            self.cluster_memory.add_node_memory(node_id.clone(), 50);
            
            // Create random connections to other nodes
            if i > 0 {
                let num_connections = rng.gen_range(1..=std::cmp::min(3, i));
                for _ in 0..num_connections {
                    let target_idx = rng.gen_range(0..i);
                    let target_id = format!("{}_node_{}", self.cluster_id, target_idx);
                    let weight = rng.gen_range(0.1..1.0);
                    self.topology.connect_nodes(node_id.clone(), target_id, weight);
                }
            }
        }
    }

    #[wasm_bindgen]
    pub fn process_input(&mut self, input_data: &[f64]) -> Vec<f64> {
        self.current_time += 1.0; // Simplified time increment
        let mut outputs = Vec::new();
        let mut node_activations: HashMap<String, f64> = HashMap::new();

        // First pass: collect all node outputs
        for (node_id, node) in &mut self.nodes {
            let output = node.process_input(input_data, self.current_time, 1.0);
            node_activations.insert(node_id.clone(), output);
            
            // Update usage statistics
            if output > 0.0 {
                *self.node_usage_stats.entry(node_id.clone()).or_insert(0) += 1;
            }
            
            // Update memory
            self.cluster_memory.update_node_memory(
                node_id,
                output,
                self.global_error,
                node.eligibility_trace(),
                node.threshold()
            );
        }

        // Second pass: propagate activations through topology
        let mut processed_outputs: HashMap<String, f64> = HashMap::new();
        
        for (node_id, activation) in &node_activations {
            let connections = self.topology.get_connections(node_id);
            let mut total_input = *activation;
            
            // Sum inputs from connected nodes
            for connected_id in &connections {
                if let Some(connected_activation) = node_activations.get(connected_id) {
                    let edge_key = (node_id.clone(), connected_id.clone());
                    let weight = self.topology.edge_weights.get(&edge_key).unwrap_or(&1.0);
                    total_input += connected_activation * weight;
                    
                    // Record edge usage
                    self.topology.record_edge_usage(node_id, connected_id);
                }
            }
            
            processed_outputs.insert(node_id.clone(), total_input);
            outputs.push(total_input);
        }

        // Update specialization scores
        self.update_specialization_scores(&processed_outputs);
        
        // Check for topology adaptations
        self.adapt_topology();
        
        // Generate memory capsule if needed
        if let Some(capsule) = self.cluster_memory.create_memory_capsule(self.current_time) {
            // In a real implementation, this would be uploaded to distributed storage
            console_log!("Generated memory capsule: {}", capsule.capsule_id);
        }

        outputs
    }

    fn update_specialization_scores(&mut self, outputs: &HashMap<String, f64>) {
        for (node_id, output) in outputs {
            let current_score = self.specialization_scores.get(node_id).unwrap_or(&0.0);
            
            // Simple specialization metric based on activation consistency
            let new_score = if *output > 0.5 {
                current_score + 0.1
            } else {
                (current_score - 0.05).max(0.0)
            };
            
            self.specialization_scores.insert(node_id.clone(), new_score);
        }
    }

    fn adapt_topology(&mut self) {
        // Node splitting: duplicate highly used nodes
        let nodes_to_split: Vec<String> = self.node_usage_stats
            .iter()
            .filter(|(_, &usage)| usage as f64 > self.node_split_threshold)
            .map(|(id, _)| id.clone())
            .collect();

        for node_id in nodes_to_split {
            self.split_node(&node_id);
        }

        // Edge duplication: strengthen high-usage connections
        let edges_to_duplicate: Vec<(String, String)> = self.topology.edge_usage
            .iter()
            .filter(|(_, &usage)| usage as f64 > self.edge_duplication_threshold)
            .map(|(edge, _)| edge.clone())
            .collect();

        for (from, to) in edges_to_duplicate {
            self.duplicate_edge(&from, &to);
        }

        // Pruning: remove weak connections
        self.prune_weak_connections();
    }

    fn split_node(&mut self, node_id: &str) -> Option<String> {
        if let Some(original_node) = self.nodes.get(node_id).cloned() {
            let new_node_id = format!("{}_split_{}", node_id, self.current_time as u64);
            
            // Create new node as a copy with slight variations
            let mut new_node = original_node.clone();
            
            // Add some noise to make the split nodes different
            let mut rng = rand::thread_rng();
            for _ in 0..5 {
                let noise_input = vec![rng.gen_range(-0.1..0.1); 4];
                new_node.process_input(&noise_input, self.current_time, 0.1);
            }
            
            self.nodes.insert(new_node_id.clone(), new_node);
            self.topology.add_node(new_node_id.clone());
            self.cluster_memory.add_node_memory(new_node_id.clone(), 50);
            
            // Copy some connections from original node
            let original_connections = self.topology.get_connections(node_id);
            for connected_id in original_connections.iter().take(2) {
                let weight = rng.gen_range(0.1..1.0);
                self.topology.connect_nodes(new_node_id.clone(), connected_id.clone(), weight);
            }
            
            // Reset usage stats for original node
            self.node_usage_stats.insert(node_id.to_string(), 0);
            
            console_log!("Split node {} into {}", node_id, new_node_id);
            Some(new_node_id)
        } else {
            None
        }
    }

    fn duplicate_edge(&mut self, from: &str, to: &str) {
        let edge_key = (from.to_string(), to.to_string());
        if let Some(&current_weight) = self.topology.edge_weights.get(&edge_key) {
            // Increase edge weight to simulate duplication
            let new_weight = (current_weight * 1.2).min(2.0);
            self.topology.edge_weights.insert(edge_key.clone(), new_weight);
            
            // Reset usage counter
            self.topology.edge_usage.insert(edge_key, 0);
        }
    }

    fn prune_weak_connections(&mut self) {
        let edges_to_remove: Vec<(String, String)> = self.topology.edge_weights
            .iter()
            .filter(|(_, &weight)| weight < self.pruning_threshold)
            .map(|(edge, _)| edge.clone())
            .collect();

        for (from, to) in edges_to_remove {
            self.topology.edge_weights.remove(&(from.clone(), to.clone()));
            self.topology.edge_usage.remove(&(from.clone(), to.clone()));
            
            // Remove from connections list
            if let Some(connections) = self.topology.connections.get_mut(&from) {
                connections.retain(|id| id != &to);
            }
        }
    }

    #[wasm_bindgen]
    pub fn update_error_signal(&mut self, error: f64) {
        self.global_error = error;
        
        // Propagate error to all nodes (forward-only)
        for node in self.nodes.values_mut() {
            node.update_error(error);
        }
        
        // Update local modulatory signal based on global error
        self.local_modulatory_signal = 0.9 * self.local_modulatory_signal + 0.1 * error.abs();
    }

    #[wasm_bindgen]
    pub fn step(&mut self, delta_time: f64) {
        self.current_time += delta_time;
        
        // Decay modulatory signal
        self.local_modulatory_signal *= 0.99;
        
        // Periodic maintenance
        if (self.current_time % 100.0) < delta_time {
            self.maintenance_cycle();
        }
    }

    fn maintenance_cycle(&mut self) {
        // Reset usage statistics periodically
        for usage in self.node_usage_stats.values_mut() {
            *usage = (*usage as f64 * 0.8) as u32;
        }
        
        for usage in self.topology.edge_usage.values_mut() {
            *usage = (*usage as f64 * 0.8) as u32;
        }
        
        // Update adaptation thresholds based on cluster performance
        let avg_specialization: f64 = self.specialization_scores.values().sum::<f64>() 
            / self.specialization_scores.len() as f64;
        
        if avg_specialization > 0.8 {
            self.node_split_threshold *= 0.95; // Make splitting easier
        } else if avg_specialization < 0.3 {
            self.node_split_threshold *= 1.05; // Make splitting harder
        }
    }

    #[wasm_bindgen]
    pub fn get_state(&self) -> JsValue {
        let state = ClusterState {
            cluster_id: self.cluster_id.clone(),
            num_nodes: self.nodes.len(),
            num_connections: self.topology.edge_weights.len(),
            current_time: self.current_time,
            global_error: self.global_error,
            avg_specialization: self.specialization_scores.values().sum::<f64>() 
                / self.specialization_scores.len().max(1) as f64,
            total_activations: self.node_usage_stats.values().sum::<u32>(),
        };
        serde_wasm_bindgen::to_value(&state).unwrap_or(JsValue::NULL)
    }

    #[wasm_bindgen]
    pub fn get_node_count(&self) -> usize {
        self.nodes.len()
    }

    #[wasm_bindgen]
    pub fn get_connection_count(&self) -> usize {
        self.topology.edge_weights.len()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClusterState {
    pub cluster_id: String,
    pub num_nodes: usize,
    pub num_connections: usize,
    pub current_time: f64,
    pub global_error: f64,
    pub avg_specialization: f64,
    pub total_activations: u32,
} 