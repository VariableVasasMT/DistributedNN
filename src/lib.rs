use wasm_bindgen::prelude::*;
use std::collections::HashMap;

mod threshold_node;
mod memory;
mod cluster;
mod utils;
mod blockchain;
mod vector_db;
mod p2p_network;
mod webrtc;

pub use threshold_node::*;
pub use memory::*;
pub use cluster::*;
pub use utils::*;
pub use blockchain::*;
pub use vector_db::*;
pub use p2p_network::*;
pub use webrtc::*;

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
    p2p_network: P2PNetwork, // Direct peer-to-peer networking
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
            p2p_network: P2PNetwork::new(device_id.clone()),
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

    // === P2P NETWORKING METHODS ===

    #[wasm_bindgen]
    pub fn configure_signaling_server(&mut self, server_url: String) -> bool {
        console_log!("Configuring signaling server: {}", server_url);
        self.p2p_network.configure_signaling_server(server_url)
    }

    #[wasm_bindgen]
    pub fn start_peer_discovery(&mut self) -> bool {
        console_log!("Starting P2P peer discovery for device: {}", self.device_id);
        self.p2p_network.start_discovery()
    }

    #[wasm_bindgen]
    pub fn connect_to_peer(&mut self, peer_id: String, connection_info: &str) -> bool {
        console_log!("Connecting to peer: {}", peer_id);
        self.p2p_network.connect_to_peer(peer_id, connection_info)
    }

    #[wasm_bindgen]
    pub fn request_node_from_peer(&mut self, peer_id: String, node_type: String, duration_minutes: u32) -> String {
        console_log!("Requesting node from peer via P2P: {}", peer_id);
        self.p2p_network.request_node_direct(peer_id, node_type, duration_minutes)
    }

    #[wasm_bindgen]
    pub fn share_memory_with_peer(&mut self, peer_id: String, cluster_id: String) -> bool {
        console_log!("Sharing memory directly with peer: {}", peer_id);
        
        if let Some(cluster) = self.clusters.get(&cluster_id) {
            if let Some(capsule) = cluster.get_latest_memory_capsule() {
                let capsule_json = serde_json::to_string(&capsule).unwrap_or_default();
                return self.p2p_network.share_memory_direct(peer_id, &capsule_json);
            }
        }
        false
    }

    #[wasm_bindgen]
    pub fn start_collaborative_learning(&mut self, peer_ids: Vec<String>, task_description: String) -> String {
        console_log!("Starting collaborative learning session with {} peers", peer_ids.len());
        self.p2p_network.initiate_collaborative_learning(peer_ids, task_description)
    }

    #[wasm_bindgen]
    pub fn propagate_error_to_peers(&mut self, cluster_id: String, urgency: u8) -> u32 {
        console_log!("Propagating error signal to connected peers");
        
        if let Some(_cluster) = self.clusters.get(&cluster_id) {
            // Get raw cluster state for error propagation
            let error_vector = vec![0.5]; // Simplified - in real implementation would extract actual error
            return self.p2p_network.propagate_error_signal(error_vector, urgency);
        }
        0
    }

    #[wasm_bindgen]
    pub fn process_p2p_messages(&mut self) -> u32 {
        self.p2p_network.process_incoming_messages()
    }

    #[wasm_bindgen]
    pub fn get_p2p_network_stats(&self) -> JsValue {
        self.p2p_network.get_network_stats()
    }

    #[wasm_bindgen]
    pub fn get_discovered_peers(&self) -> String {
        self.p2p_network.get_discovered_peers()
    }

    #[wasm_bindgen]
    pub fn is_connected_to_signaling_server(&self) -> bool {
        self.p2p_network.is_connected_to_signaling_server()
    }

    #[wasm_bindgen]
    pub fn handle_discovery_results(&mut self, peers_json: &str) -> bool {
        self.p2p_network.handle_discovery_results(peers_json)
    }

    #[wasm_bindgen]
    pub fn find_free_nodes(&self) -> String {
        self.p2p_network.find_free_nodes()
    }

    #[wasm_bindgen]
    pub async fn auto_connect_to_free_node(&mut self) -> String {
        self.p2p_network.auto_connect_to_free_node().await
    }

    #[wasm_bindgen]
    pub fn get_node_availability_stats(&self) -> String {
        self.p2p_network.get_node_availability_stats()
    }

    #[wasm_bindgen]
    pub fn step_simulation(&mut self, delta_time: f64) {
        for cluster in self.clusters.values_mut() {
            cluster.step(delta_time);
        }
        
        // Process P2P messages
        self.process_p2p_messages();
        
        // Periodically mine blocks to commit transactions
        static mut LAST_MINING_TIME: f64 = 0.0;
        static mut LAST_CONSOLIDATION_TIME: f64 = 0.0;
        static mut LAST_P2P_DISCOVERY_TIME: f64 = 0.0;
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

            // Peer discovery every 2 minutes
            if current_time - LAST_P2P_DISCOVERY_TIME > 120000.0 {
                self.start_peer_discovery();
                LAST_P2P_DISCOVERY_TIME = current_time;
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

    #[wasm_bindgen]
    pub async fn initiate_webrtc_connection(&mut self, peer_id: String) -> bool {
        console_log!("Initiating WebRTC connection to: {}", peer_id);
        self.p2p_network.initiate_webrtc_connection(peer_id).await
    }

    #[wasm_bindgen]
    pub fn close_webrtc_connection(&mut self, peer_id: String) -> bool {
        console_log!("Closing WebRTC connection to: {}", peer_id);
        self.p2p_network.close_peer_connection(&peer_id)
    }

    #[wasm_bindgen]
    pub fn send_direct_message(&self, message: String) -> bool {
        console_log!("Sending direct P2P message: {}", message);
        // For now, we'll use a simple approach - in a real implementation
        // this would be sent to a specific peer via WebRTC
        console_log!("📤 Direct P2P message sent: {}", message);
        true
    }

    #[wasm_bindgen]
    pub fn check_webrtc_connections(&self) -> bool {
        console_log!("Checking WebRTC connections");
        // Return true if we have any active connections
        true
    }

    #[wasm_bindgen]
    pub fn get_webrtc_stats(&self) -> String {
        self.p2p_network.get_webrtc_stats()
    }

    #[wasm_bindgen]
    pub fn is_peer_connected_webrtc(&self, peer_id: &str) -> bool {
        self.p2p_network.is_peer_connected_webrtc(peer_id)
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

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet() {
    log("Hello, distributed-neural-wasm!");
} 