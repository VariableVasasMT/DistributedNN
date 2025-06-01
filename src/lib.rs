use wasm_bindgen::prelude::*;
use std::collections::HashMap;

mod threshold_node;
mod memory;
mod cluster;
mod utils;
mod blockchain;
mod vector_db;

pub use threshold_node::*;
pub use memory::*;
pub use cluster::*;
pub use utils::*;
pub use blockchain::*;
pub use vector_db::*;

// Re-export key types for JavaScript
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Macro for easier console logging from Rust
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// Initialize the WASM module
#[wasm_bindgen(start)]
pub fn init() {
    utils::set_panic_hook();
    console_log!("Distributed Neural Network with Blockchain Vector Database WASM module initialized");
}

// Main API for JavaScript interaction
#[wasm_bindgen]
pub struct DistributedNeuralNetwork {
    clusters: HashMap<String, DeviceCluster>,
    global_memory: GlobalMemory,
    blockchain: BlockchainLedger,
    vector_database: VectorMemoryDatabase, // Long-term memory blockchain vector database
    device_id: String,
}

#[wasm_bindgen]
impl DistributedNeuralNetwork {
    #[wasm_bindgen(constructor)]
    pub fn new(device_id: String) -> DistributedNeuralNetwork {
        console_log!("Creating new distributed neural network for device: {}", device_id);
        
        let mut blockchain = BlockchainLedger::new();
        // Register this device with initial credits
        blockchain.register_device(device_id.clone(), 10.0);
        
        DistributedNeuralNetwork {
            clusters: HashMap::new(),
            global_memory: GlobalMemory::new(),
            blockchain,
            vector_database: VectorMemoryDatabase::new(),
            device_id,
        }
    }

    #[wasm_bindgen]
    pub fn create_cluster(&mut self, cluster_id: String, num_nodes: usize) -> bool {
        console_log!("Creating cluster {} with {} nodes", cluster_id, num_nodes);
        
        let cluster = DeviceCluster::new(cluster_id.clone(), num_nodes);
        self.clusters.insert(cluster_id, cluster);
        true
    }

    #[wasm_bindgen]
    pub fn process_input(&mut self, cluster_id: String, input_data: &[f64]) -> Vec<f64> {
        if let Some(cluster) = self.clusters.get_mut(&cluster_id) {
            let outputs = cluster.process_input(input_data);
            
            // Check if a memory capsule was created and register it in blockchain + vector database
            if let Some(capsule) = cluster.get_latest_memory_capsule() {
                let capsule_json = serde_json::to_string(&capsule).unwrap_or_default();
                if !capsule_json.is_empty() {
                    // Register on blockchain for auditability and incentives
                    let capsule_id = self.blockchain.register_memory_capsule(&capsule_json, self.device_id.clone());
                    
                    if !capsule_id.is_empty() {
                        // Store in vector database for long-term semantic search
                        let blockchain_hash = format!("blockchain_hash_{}", capsule_id);
                        self.vector_database.store_memory_capsule(&capsule_json, blockchain_hash);
                        
                        // Also store in global memory for immediate access
                        self.global_memory.store_capsule(&capsule_json);
                        
                        console_log!("Memory capsule {} registered in blockchain vector database", capsule_id);
                    }
                }
            }
            
            outputs
        } else {
            console_log!("Cluster {} not found", cluster_id);
            vec![]
        }
    }

    #[wasm_bindgen]
    pub fn semantic_memory_search(&mut self, query_vector: &[f64], context_tags: &str, max_results: usize) -> String {
        let query = crate::vector_db::VectorSearchQuery {
            query_vector: query_vector.to_vec(),
            context_filter: context_tags.split(',').map(|s| s.trim().to_string()).collect(),
            time_range: None,
            quality_threshold: 0.3,
            max_results,
            search_algorithm: crate::vector_db::SearchAlgorithm::Hybrid,
        };
        
        let query_json = serde_json::to_string(&query).unwrap_or_default();
        self.vector_database.semantic_search(&query_json)
    }

    #[wasm_bindgen]
    pub fn get_memory_trends(&self) -> String {
        self.vector_database.get_memory_trends()
    }

    #[wasm_bindgen]
    pub fn consolidate_long_term_memory(&mut self) -> bool {
        console_log!("Consolidating long-term memory database");
        self.vector_database.consolidate_memory()
    }

    #[wasm_bindgen]
    pub fn get_cluster_state(&self, cluster_id: String) -> JsValue {
        if let Some(cluster) = self.clusters.get(&cluster_id) {
            cluster.get_state()
        } else {
            JsValue::NULL
        }
    }

    #[wasm_bindgen]
    pub fn update_error_signal(&mut self, cluster_id: String, error: f64) {
        if let Some(cluster) = self.clusters.get_mut(&cluster_id) {
            cluster.update_error_signal(error);
        }
    }

    #[wasm_bindgen]
    pub fn step_simulation(&mut self, delta_time: f64) {
        for cluster in self.clusters.values_mut() {
            cluster.step(delta_time);
        }
        
        // Periodically mine blocks to commit transactions
        static mut LAST_MINING_TIME: f64 = 0.0;
        static mut LAST_CONSOLIDATION_TIME: f64 = 0.0;
        let current_time = js_sys::Date::now();
        
        unsafe {
            // Mine blocks every 10 seconds
            if current_time - LAST_MINING_TIME > 10000.0 {
                let block_hash = self.blockchain.mine_block();
                if !block_hash.is_empty() {
                    console_log!("Mined block: {}", block_hash);
                }
                LAST_MINING_TIME = current_time;
            }
            
            // Consolidate long-term memory every 5 minutes
            if current_time - LAST_CONSOLIDATION_TIME > 300000.0 {
                self.consolidate_long_term_memory();
                LAST_CONSOLIDATION_TIME = current_time;
            }
        }
    }

    #[wasm_bindgen]
    pub fn request_node_borrowing(&mut self, node_owner: String, node_id: String, duration: f64) -> String {
        self.blockchain.request_node_borrowing(
            self.device_id.clone(),
            node_owner,
            node_id,
            duration
        )
    }

    #[wasm_bindgen]
    pub fn complete_node_borrowing(&mut self, borrowing_id: String, performance_data: &str) -> bool {
        self.blockchain.complete_node_borrowing(borrowing_id, performance_data)
    }

    #[wasm_bindgen]
    pub fn get_account_balance(&self) -> f64 {
        self.blockchain.get_account_balance(&self.device_id)
    }

    #[wasm_bindgen]
    pub fn get_blockchain_stats(&self) -> JsValue {
        self.blockchain.get_blockchain_stats()
    }

    #[wasm_bindgen]
    pub fn mine_block(&mut self) -> String {
        self.blockchain.mine_block()
    }

    #[wasm_bindgen]
    pub fn validate_blockchain(&self) -> bool {
        self.blockchain.validate_chain()
    }

    #[wasm_bindgen]
    pub fn get_memory_record(&self, capsule_id: &str) -> String {
        self.blockchain.get_memory_record(capsule_id)
    }

    #[wasm_bindgen]
    pub fn get_vector_database_stats(&self) -> JsValue {
        let stats = VectorDatabaseStats {
            total_vectors: self.vector_database.get_vector_count(),
            total_memory_size: self.vector_database.get_total_memory_size(),
            average_vector_dimension: self.vector_database.get_average_vector_dimension(),
            semantic_clusters: self.vector_database.get_semantic_cluster_count(),
            temporal_entries: self.vector_database.get_temporal_entry_count(),
            blockchain_verified_rate: self.vector_database.calculate_blockchain_verification_rate(),
        };
        
        serde_wasm_bindgen::to_value(&stats).unwrap_or(JsValue::NULL)
    }
}

#[derive(serde::Serialize)]
struct VectorDatabaseStats {
    total_vectors: usize,
    total_memory_size: usize,
    average_vector_dimension: usize,
    semantic_clusters: usize,
    temporal_entries: usize,
    blockchain_verified_rate: f64,
} 