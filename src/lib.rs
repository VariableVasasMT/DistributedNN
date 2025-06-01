use wasm_bindgen::prelude::*;
use std::collections::HashMap;

mod threshold_node;
mod memory;
mod cluster;
mod utils;

pub use threshold_node::*;
pub use memory::*;
pub use cluster::*;
pub use utils::*;

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
    console_log!("Distributed Neural Network WASM module initialized");
}

// Main API for JavaScript interaction
#[wasm_bindgen]
pub struct DistributedNeuralNetwork {
    clusters: HashMap<String, DeviceCluster>,
    global_memory: GlobalMemory,
    device_id: String,
}

#[wasm_bindgen]
impl DistributedNeuralNetwork {
    #[wasm_bindgen(constructor)]
    pub fn new(device_id: String) -> DistributedNeuralNetwork {
        console_log!("Creating new distributed neural network for device: {}", device_id);
        
        DistributedNeuralNetwork {
            clusters: HashMap::new(),
            global_memory: GlobalMemory::new(),
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
            cluster.process_input(input_data)
        } else {
            console_log!("Cluster {} not found", cluster_id);
            vec![]
        }
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
    }
} 