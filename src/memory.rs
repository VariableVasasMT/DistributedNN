use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, VecDeque};

/// Three-level memory hierarchy as described in the paper
/// Level 1: Node Memory - local to each threshold gating node
/// Level 2: Cluster Memory - shared within device clusters  
/// Level 3: Global Memory - distributed across the network

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeMemory {
    pub node_id: String,
    pub activations: VecDeque<f64>,
    pub errors: VecDeque<f64>,
    pub eligibility_history: VecDeque<f64>,
    pub threshold_history: VecDeque<f64>,
    pub timer_events: VecDeque<(f64, String)>, // (time, event_type)
    pub context_tags: Vec<String>,
    pub max_size: usize,
}

impl NodeMemory {
    pub fn new(node_id: String, max_size: usize) -> Self {
        NodeMemory {
            node_id,
            activations: VecDeque::with_capacity(max_size),
            errors: VecDeque::with_capacity(max_size),
            eligibility_history: VecDeque::with_capacity(max_size),
            threshold_history: VecDeque::with_capacity(max_size),
            timer_events: VecDeque::with_capacity(max_size),
            context_tags: Vec::new(),
            max_size,
        }
    }

    pub fn store_activation(&mut self, activation: f64, error: f64, eligibility: f64, threshold: f64) {
        self.activations.push_back(activation);
        self.errors.push_back(error);
        self.eligibility_history.push_back(eligibility);
        self.threshold_history.push_back(threshold);

        // Maintain size limits
        if self.activations.len() > self.max_size {
            self.activations.pop_front();
            self.errors.pop_front();
            self.eligibility_history.pop_front();
            self.threshold_history.pop_front();
        }
    }

    pub fn add_event(&mut self, time: f64, event_type: String) {
        self.timer_events.push_back((time, event_type));
        if self.timer_events.len() > self.max_size {
            self.timer_events.pop_front();
        }
    }

    pub fn add_context_tag(&mut self, tag: String) {
        if !self.context_tags.contains(&tag) {
            self.context_tags.push(tag);
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryCapsule {
    pub capsule_id: String,
    pub timestamp: f64,
    pub cluster_id: String,
    pub privacy_level: PrivacyLevel,
    pub context_vector: Vec<f64>,
    pub semantic_tags: Vec<String>,
    pub adaptation_summary: AdaptationSummary,
    pub compressed_data: Vec<u8>, // Encrypted and compressed node states
    pub novelty_score: f64,
    pub importance_score: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PrivacyLevel {
    Personal,    // Encrypted, private to device
    Behavioral,  // Shareable, behavioral patterns only
    Public,      // Open sharing allowed
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdaptationSummary {
    pub threshold_adaptations: u32,
    pub timer_adaptations: u32,
    pub weight_changes: f64,
    pub error_magnitude: f64,
    pub learning_rate_changes: f64,
    pub specialization_metrics: HashMap<String, f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClusterMemory {
    pub cluster_id: String,
    pub node_memories: HashMap<String, NodeMemory>,
    pub capsule_buffer: VecDeque<MemoryCapsule>,
    pub aggregated_stats: HashMap<String, f64>,
    pub semantic_index: HashMap<String, Vec<String>>, // tag -> capsule_ids
    pub consolidation_threshold: usize,
    pub last_consolidation: f64,
}

impl ClusterMemory {
    pub fn new(cluster_id: String) -> Self {
        ClusterMemory {
            cluster_id,
            node_memories: HashMap::new(),
            capsule_buffer: VecDeque::new(),
            aggregated_stats: HashMap::new(),
            semantic_index: HashMap::new(),
            consolidation_threshold: 10,
            last_consolidation: 0.0,
        }
    }

    pub fn add_node_memory(&mut self, node_id: String, memory_size: usize) {
        self.node_memories.insert(node_id.clone(), NodeMemory::new(node_id, memory_size));
    }

    pub fn update_node_memory(&mut self, node_id: &str, activation: f64, error: f64, eligibility: f64, threshold: f64) {
        if let Some(memory) = self.node_memories.get_mut(node_id) {
            memory.store_activation(activation, error, eligibility, threshold);
        }
    }

    pub fn create_memory_capsule(&mut self, current_time: f64) -> Option<MemoryCapsule> {
        if self.should_consolidate(current_time) {
            let capsule = self.consolidate_memories(current_time);
            self.last_consolidation = current_time;
            self.capsule_buffer.push_back(capsule.clone());
            
            // Maintain buffer size
            if self.capsule_buffer.len() > 100 {
                self.capsule_buffer.pop_front();
            }
            
            Some(capsule)
        } else {
            None
        }
    }

    fn should_consolidate(&self, current_time: f64) -> bool {
        // Consolidate based on time interval or buffer size
        (current_time - self.last_consolidation) > 60.0 || // Every minute
        self.node_memories.values().any(|mem| mem.activations.len() >= mem.max_size * 3/4)
    }

    fn consolidate_memories(&mut self, current_time: f64) -> MemoryCapsule {
        // Aggregate statistics from all nodes
        let mut adaptation_summary = AdaptationSummary {
            threshold_adaptations: 0,
            timer_adaptations: 0,
            weight_changes: 0.0,
            error_magnitude: 0.0,
            learning_rate_changes: 0.0,
            specialization_metrics: HashMap::new(),
        };

        let mut context_vector = vec![0.0; 16]; // Fixed-size context embedding
        let mut semantic_tags = Vec::new();

        // Process each node's memory
        for (_node_id, memory) in &self.node_memories {
            if !memory.activations.is_empty() {
                // Compute summary statistics
                let avg_activation: f64 = memory.activations.iter().sum::<f64>() / memory.activations.len() as f64;
                let avg_error: f64 = memory.errors.iter().sum::<f64>() / memory.errors.len() as f64;
                
                adaptation_summary.error_magnitude += avg_error.abs();
                
                // Add to context vector (simple encoding)
                context_vector[0] += avg_activation;
                context_vector[1] += avg_error;
                context_vector[2] += memory.eligibility_history.iter().sum::<f64>();
                
                // Collect semantic tags
                semantic_tags.extend(memory.context_tags.clone());
            }
        }

        // Normalize context vector
        let magnitude: f64 = context_vector.iter().map(|x| x * x).sum::<f64>().sqrt();
        if magnitude > 0.0 {
            for val in &mut context_vector {
                *val /= magnitude;
            }
        }

        // Remove duplicate tags
        semantic_tags.sort();
        semantic_tags.dedup();

        // Calculate novelty and importance scores
        let novelty_score = self.calculate_novelty(&context_vector);
        let importance_score = adaptation_summary.error_magnitude + (semantic_tags.len() as f64 * 0.1);

        // Determine privacy level based on semantic tags
        let privacy_level = if semantic_tags.iter().any(|tag| tag.contains("personal") || tag.contains("private")) {
            PrivacyLevel::Personal
        } else if semantic_tags.iter().any(|tag| tag.contains("behavior") || tag.contains("pattern")) {
            PrivacyLevel::Behavioral
        } else {
            PrivacyLevel::Public
        };

        // Create compressed data (simplified - in real implementation would use proper compression/encryption)
        let compressed_data = serde_json::to_vec(&self.node_memories).unwrap_or_default();

        MemoryCapsule {
            capsule_id: format!("{}_{}", self.cluster_id, current_time as u64),
            timestamp: current_time,
            cluster_id: self.cluster_id.clone(),
            privacy_level,
            context_vector,
            semantic_tags,
            adaptation_summary,
            compressed_data,
            novelty_score,
            importance_score,
        }
    }

    fn calculate_novelty(&self, context_vector: &[f64]) -> f64 {
        // Simple novelty calculation based on distance from previous capsules
        if self.capsule_buffer.is_empty() {
            return 1.0; // First capsule is novel
        }

        let mut min_distance = f64::MAX;
        for capsule in &self.capsule_buffer {
            let distance = euclidean_distance(context_vector, &capsule.context_vector);
            min_distance = min_distance.min(distance);
        }

        // Normalize novelty score (higher = more novel)
        (min_distance / 2.0).min(1.0)
    }

    pub fn query_similar_capsules(&self, query_vector: &[f64], num_results: usize) -> Vec<String> {
        let mut scored_capsules: Vec<(f64, String)> = self.capsule_buffer
            .iter()
            .map(|capsule| {
                let similarity = cosine_similarity(query_vector, &capsule.context_vector);
                (similarity, capsule.capsule_id.clone())
            })
            .collect();

        scored_capsules.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        scored_capsules.into_iter()
            .take(num_results)
            .map(|(_, id)| id)
            .collect()
    }

    pub fn get_latest_capsule(&self) -> Option<MemoryCapsule> {
        self.capsule_buffer.back().cloned()
    }
}

#[wasm_bindgen]
pub struct GlobalMemory {
    capsules: HashMap<String, MemoryCapsule>,
    semantic_index: HashMap<String, Vec<String>>,
    device_contributions: HashMap<String, u32>,
    incentive_scores: HashMap<String, f64>,
}

#[wasm_bindgen]
impl GlobalMemory {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GlobalMemory {
        GlobalMemory {
            capsules: HashMap::new(),
            semantic_index: HashMap::new(),
            device_contributions: HashMap::new(),
            incentive_scores: HashMap::new(),
        }
    }

    #[wasm_bindgen]
    pub fn store_capsule(&mut self, capsule_json: &str) -> bool {
        if let Ok(capsule) = serde_json::from_str::<MemoryCapsule>(capsule_json) {
            // Update semantic index
            for tag in &capsule.semantic_tags {
                self.semantic_index
                    .entry(tag.clone())
                    .or_insert_with(Vec::new)
                    .push(capsule.capsule_id.clone());
            }

            // Update device contributions
            *self.device_contributions.entry(capsule.cluster_id.clone()).or_insert(0) += 1;
            
            // Update incentive scores based on novelty and importance
            let score = capsule.novelty_score * capsule.importance_score;
            *self.incentive_scores.entry(capsule.cluster_id.clone()).or_insert(0.0) += score;

            self.capsules.insert(capsule.capsule_id.clone(), capsule);
            true
        } else {
            false
        }
    }

    #[wasm_bindgen]
    pub fn query_capsules_by_tags(&self, tags: &str) -> String {
        let tag_list: Vec<String> = tags.split(',').map(|s| s.trim().to_string()).collect();
        let mut matching_capsules = Vec::new();

        for tag in tag_list {
            if let Some(capsule_ids) = self.semantic_index.get(&tag) {
                for id in capsule_ids {
                    if let Some(capsule) = self.capsules.get(id) {
                        matching_capsules.push(capsule);
                    }
                }
            }
        }

        serde_json::to_string(&matching_capsules).unwrap_or_default()
    }

    #[wasm_bindgen]
    pub fn get_device_incentive_score(&self, device_id: &str) -> f64 {
        self.incentive_scores.get(device_id).copied().unwrap_or(0.0)
    }

    #[wasm_bindgen]
    pub fn get_total_capsules(&self) -> usize {
        self.capsules.len()
    }
}

// Utility functions for vector operations
fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    let dot_product: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let magnitude_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let magnitude_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();
    
    if magnitude_a == 0.0 || magnitude_b == 0.0 {
        0.0
    } else {
        dot_product / (magnitude_a * magnitude_b)
    }
} 