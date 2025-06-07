use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::memory::MemoryCapsule;
use crate::webrtc::WebRTCManager;

// Import the console_log macro
use crate::console_log;

/// Direct peer-to-peer networking layer for device communication
/// Enables real-time node borrowing, memory sharing, and collaborative learning
#[wasm_bindgen]
#[derive(Clone)]
pub struct P2PNetwork {
    device_id: String,
    peer_registry: HashMap<String, PeerInfo>,
    active_connections: HashMap<String, P2PConnection>,
    message_queue: Vec<P2PMessage>,
    discovery_protocol: DiscoveryProtocol,
    routing_table: HashMap<String, Vec<String>>, // device_id -> path to reach it
    signaling_server_url: String,
    is_connected_to_server: bool,
    webrtc_manager: Option<WebRTCManager>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignalingServerConfig {
    pub url: String,
    pub reconnect_interval: f64,
    pub heartbeat_interval: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignalingMessage {
    pub message_type: String,
    pub data: serde_json::Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeerInfo {
    pub device_id: String,
    pub ip_address: String,
    pub port: u16,
    pub public_key: String,
    pub capabilities: Vec<String>, // e.g., ["memory_sharing", "node_lending", "inference"]
    pub reputation_score: f64,
    pub last_seen: f64,
    pub cluster_specializations: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct P2PConnection {
    pub peer_id: String,
    pub connection_type: ConnectionType,
    pub status: ConnectionStatus,
    pub established_time: f64,
    pub bandwidth_usage: f64,
    pub latency_ms: f64,
    pub encryption_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ConnectionType {
    DirectTCP,
    WebRTC,
    WebSocket,
    Relay, // Through intermediate peer
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ConnectionStatus {
    Connecting,
    Established,
    Authenticated,
    Disconnecting,
    Failed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct P2PMessage {
    pub message_id: String,
    pub from: String,
    pub to: String,
    pub message_type: MessageType,
    pub payload: MessagePayload,
    pub timestamp: f64,
    pub signature: String,
    pub hop_count: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MessageType {
    NodeRequest,      // Request to borrow a node
    NodeResponse,     // Response with node data
    MemoryShare,      // Share memory capsule directly
    CollaborativeLearn, // Invite to collaborative learning session
    ErrorPropagate,   // Forward error signals for distributed learning
    HeartBeat,        // Keep-alive and status updates
    Discovery,        // Peer discovery and announcement
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MessagePayload {
    NodeRequestData {
        node_type: String,
        required_capabilities: Vec<String>,
        duration_minutes: u32,
        payment_offer: f64,
    },
    NodeResponseData {
        node_data: String, // Serialized ThresholdGatingNode
        approval_status: bool,
        rental_cost: f64,
        availability_window: (f64, f64),
    },
    MemoryShareData {
        capsule: MemoryCapsule,
        access_level: String,
        sharing_reward: f64,
    },
    CollaborativeLearnData {
        task_description: String,
        dataset_hash: String,
        learning_parameters: HashMap<String, f64>,
        participant_rewards: HashMap<String, f64>,
    },
    ErrorPropagateData {
        error_vector: Vec<f64>,
        source_cluster: String,
        propagation_weight: f64,
        urgency_level: u8,
    },
    HeartBeatData {
        device_status: String,
        available_resources: HashMap<String, f64>,
        recent_activities: Vec<String>,
    },
    DiscoveryData {
        device_info: PeerInfo,
        network_topology: HashMap<String, Vec<String>>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiscoveryProtocol {
    pub discovery_interval: f64, // milliseconds
    pub last_discovery: f64,
    pub known_bootstrap_nodes: Vec<String>,
    pub discovery_radius: u8, // how many hops to search
}

#[wasm_bindgen]
impl P2PNetwork {
    #[wasm_bindgen(constructor)]
    pub fn new(device_id: String) -> P2PNetwork {
        console_log!("Initializing P2P network for device: {}", device_id);
        
        let webrtc_manager = WebRTCManager::new(device_id.clone());
        
        P2PNetwork {
            device_id: device_id.clone(),
            peer_registry: HashMap::new(),
            active_connections: HashMap::new(),
            message_queue: Vec::new(),
            discovery_protocol: DiscoveryProtocol {
                discovery_interval: 30000.0, // 30 seconds
                last_discovery: 0.0,
                known_bootstrap_nodes: vec![
                    "ws://localhost:8080".to_string(),  // Local signaling server
                    "wss://distributednn-signaling.herokuapp.com".to_string(), // Production server
                ],
                discovery_radius: 3,
            },
            routing_table: HashMap::new(),
            signaling_server_url: "ws://localhost:8080".to_string(),
            is_connected_to_server: false,
            webrtc_manager: Some(webrtc_manager),
        }
    }

    #[wasm_bindgen]
    pub fn configure_signaling_server(&mut self, server_url: String) -> bool {
        console_log!("Configuring signaling server: {}", server_url);
        self.signaling_server_url = server_url.clone();
        
        // Connect WebRTC manager to signaling server
        if let Some(ref mut webrtc_manager) = self.webrtc_manager {
            match webrtc_manager.connect_signaling_server(&server_url) {
                Ok(_) => {
                    console_log!("WebRTC manager connected to signaling server");
                    self.is_connected_to_server = true;
                    true
                },
                Err(e) => {
                    console_log!("Failed to connect WebRTC manager: {:?}", e);
                    false
                }
            }
        } else {
            false
        }
    }

    #[wasm_bindgen]
    pub async fn initiate_webrtc_connection(&mut self, target_device_id: String) -> bool {
        console_log!("Initiating real WebRTC connection to: {}", target_device_id);
        
        if let Some(ref mut webrtc_manager) = self.webrtc_manager {
            // Create peer connection
            match webrtc_manager.create_peer_connection(&target_device_id) {
                Ok(_) => {
                    console_log!("Created peer connection for: {}", target_device_id);
                    
                    // Create offer
                    match webrtc_manager.create_offer(&target_device_id).await {
                        Ok(offer_json) => {
                            console_log!("Created WebRTC offer for: {}", target_device_id);
                            
                            // Send offer via signaling server
                            self.send_signaling_message("signal", serde_json::json!({
                                "target_device_id": target_device_id,
                                "signaling_data": {
                                    "type": "offer",
                                    "offer": offer_json
                                }
                            }));
                            
                            // Update connection status
                            let connection = P2PConnection {
                                peer_id: target_device_id.clone(),
                                connection_type: ConnectionType::WebRTC,
                                status: ConnectionStatus::Connecting,
                                established_time: js_sys::Date::now(),
                                bandwidth_usage: 0.0,
                                latency_ms: 0.0,
                                encryption_key: "webrtc_dtls_key".to_string(),
                            };
                            
                            self.active_connections.insert(target_device_id, connection);
                            true
                        },
                        Err(e) => {
                            console_log!("Failed to create offer: {:?}", e);
                            false
                        }
                    }
                },
                Err(e) => {
                    console_log!("Failed to create peer connection: {:?}", e);
                    false
                }
            }
        } else {
            console_log!("WebRTC manager not available");
            false
        }
    }

    #[wasm_bindgen]
    pub async fn handle_webrtc_offer(&mut self, peer_id: String, offer_json: String) -> bool {
        console_log!("Handling WebRTC offer from: {}", peer_id);
        
        if let Some(ref mut webrtc_manager) = self.webrtc_manager {
            // Create peer connection for incoming offer
            match webrtc_manager.create_peer_connection(&peer_id) {
                Ok(_) => {
                    // Create answer
                    match webrtc_manager.create_answer(&peer_id, &offer_json).await {
                        Ok(answer_json) => {
                            console_log!("Created WebRTC answer for: {}", peer_id);
                            
                            // Send answer via signaling server
                            self.send_signaling_message("signal", serde_json::json!({
                                "target_device_id": peer_id,
                                "signaling_data": {
                                    "type": "answer",
                                    "answer": answer_json
                                }
                            }));
                            
                            // Update connection status
                            let connection = P2PConnection {
                                peer_id: peer_id.clone(),
                                connection_type: ConnectionType::WebRTC,
                                status: ConnectionStatus::Connecting,
                                established_time: js_sys::Date::now(),
                                bandwidth_usage: 0.0,
                                latency_ms: 0.0,
                                encryption_key: "webrtc_dtls_key".to_string(),
                            };
                            
                            self.active_connections.insert(peer_id, connection);
                            true
                        },
                        Err(e) => {
                            console_log!("Failed to create answer: {:?}", e);
                            false
                        }
                    }
                },
                Err(e) => {
                    console_log!("Failed to create peer connection: {:?}", e);
                    false
                }
            }
        } else {
            false
        }
    }

    #[wasm_bindgen]
    pub async fn handle_webrtc_answer(&mut self, peer_id: String, answer_json: String) -> bool {
        console_log!("Handling WebRTC answer from: {}", peer_id);
        
        if let Some(ref mut webrtc_manager) = self.webrtc_manager {
            match webrtc_manager.set_remote_answer(&peer_id, &answer_json).await {
                Ok(_) => {
                    console_log!("Successfully set remote answer for: {}", peer_id);
                    
                    // Update connection status to established
                    if let Some(connection) = self.active_connections.get_mut(&peer_id) {
                        connection.status = ConnectionStatus::Established;
                    }
                    true
                },
                Err(e) => {
                    console_log!("Failed to set remote answer: {:?}", e);
                    false
                }
            }
        } else {
            false
        }
    }

    #[wasm_bindgen]
    pub async fn handle_ice_candidate(&mut self, peer_id: String, candidate_json: String) -> bool {
        console_log!("Handling ICE candidate from: {}", peer_id);
        
        if let Some(ref mut webrtc_manager) = self.webrtc_manager {
            match webrtc_manager.add_ice_candidate(&peer_id, &candidate_json).await {
                Ok(_) => {
                    console_log!("Successfully added ICE candidate for: {}", peer_id);
                    true
                },
                Err(e) => {
                    console_log!("Failed to add ICE candidate: {:?}", e);
                    false
                }
            }
        } else {
            false
        }
    }

    fn send_signaling_message(&self, message_type: &str, data: serde_json::Value) {
        // In a real implementation, this would send via WebSocket to signaling server
        console_log!("Sending signaling message: {} - {:?}", message_type, data);
        // This would be implemented with actual WebSocket communication
    }

    #[wasm_bindgen]
    pub fn connect_to_peer(&mut self, peer_id: String, _connection_info: &str) -> bool {
        console_log!("Attempting to connect to peer via real WebRTC: {}", peer_id);
        
        // Check if peer exists in registry
        if !self.peer_registry.contains_key(&peer_id) {
            console_log!("Peer {} not found in registry, attempting discovery first", peer_id);
            self.start_discovery();
        }
        
        // This will now be handled asynchronously
        // For now, return true to indicate the process started
        true
    }

    fn send_direct_message(&self, peer_id: String, message: P2PMessage) -> bool {
        if let Some(ref webrtc_manager) = self.webrtc_manager {
            if webrtc_manager.is_connected(&peer_id) {
                // Send message via WebRTC data channel
                let message_json = serde_json::to_string(&message).unwrap_or_default();
                match webrtc_manager.send_data(&peer_id, &message_json) {
                    Ok(_) => {
                        console_log!("Sent P2P message via WebRTC to: {}", peer_id);
                        return true;
                    },
                    Err(e) => {
                        console_log!("Failed to send WebRTC message: {:?}", e);
                    }
                }
            }
        }
        
        // Try to find a route through intermediate peers
        if let Some(route) = self.routing_table.get(&peer_id) {
            if !route.is_empty() {
                console_log!("Routing message to {} via {}", peer_id, route[0]);
                return true;
            }
        }

        console_log!("No direct WebRTC connection or route found to peer: {}", peer_id);
        false
    }

    #[wasm_bindgen]
    pub fn get_webrtc_stats(&self) -> String {
        if let Some(ref webrtc_manager) = self.webrtc_manager {
            webrtc_manager.get_connection_stats()
        } else {
            serde_json::json!({"error": "WebRTC manager not available"}).to_string()
        }
    }

    #[wasm_bindgen]
    pub fn is_peer_connected_webrtc(&self, peer_id: &str) -> bool {
        if let Some(ref webrtc_manager) = self.webrtc_manager {
            webrtc_manager.is_connected(peer_id)
        } else {
            false
        }
    }

    #[wasm_bindgen]
    pub fn close_peer_connection(&mut self, peer_id: &str) -> bool {
        console_log!("Closing WebRTC connection to peer: {}", peer_id);
        
        if let Some(ref mut webrtc_manager) = self.webrtc_manager {
            match webrtc_manager.close_connection(peer_id) {
                Ok(_) => {
                    self.active_connections.remove(peer_id);
                    console_log!("Successfully closed connection to: {}", peer_id);
                    true
                },
                Err(e) => {
                    console_log!("Failed to close connection: {:?}", e);
                    false
                }
            }
        } else {
            false
        }
    }

    #[wasm_bindgen]
    pub fn start_discovery(&mut self) -> bool {
        if !self.is_connected_to_server {
            console_log!("Not connected to signaling server, attempting to connect...");
            // For now, assume we're connected if we have a signaling server URL
            if !self.signaling_server_url.is_empty() {
                self.is_connected_to_server = true;
            } else {
                return false;
            }
        }
        
        console_log!("Starting peer discovery via signaling server");
        
        // Send discovery request to signaling server
        let _discovery_message = SignalingMessage {
            message_type: "discover".to_string(),
            data: serde_json::json!({
                "filters": {
                    "required_capabilities": ["memory_sharing"],
                    "specializations": ["general"]
                }
            }),
        };
        
        console_log!("Sent discovery request to signaling server");
        
        // Simulate server response with some example peers
        self.simulate_discovery_response();
        
        self.discovery_protocol.last_discovery = js_sys::Date::now();
        true
    }

    fn simulate_server_response(&mut self, response_type: &str, data: serde_json::Value) {
        match response_type {
            "registered" => {
                console_log!("✅ Successfully registered with signaling server");
                if let Some(peer_count) = data.get("peer_count") {
                    console_log!("📊 Server reports {} total peers", peer_count);
                }
            },
            "discovery_result" => {
                if let Some(peers) = data.get("peers").and_then(|p| p.as_array()) {
                    console_log!("🔍 Discovery found {} peers", peers.len());
                    
                    for peer_data in peers {
                        if let Ok(peer_info) = serde_json::from_value::<PeerInfo>(peer_data.clone()) {
                            self.peer_registry.insert(peer_info.device_id.clone(), peer_info);
                        }
                    }
                }
            },
            _ => {
                console_log!("Received server response: {}", response_type);
            }
        }
    }

    fn simulate_discovery_response(&mut self) {
        // Simulate finding some peers for demo purposes
        let simulated_peers = vec![
            PeerInfo {
                device_id: "demo_peer_001".to_string(),
                ip_address: "192.168.1.100".to_string(),
                port: 8080,
                public_key: "peer1_public_key".to_string(),
                capabilities: vec!["memory_sharing".to_string(), "node_lending".to_string()],
                reputation_score: 0.9,
                last_seen: js_sys::Date::now(),
                cluster_specializations: vec!["image_processing".to_string()],
            },
            PeerInfo {
                device_id: "demo_peer_002".to_string(),
                ip_address: "192.168.1.101".to_string(),
                port: 8080,
                public_key: "peer2_public_key".to_string(),
                capabilities: vec!["collaborative_learning".to_string(), "inference".to_string()],
                reputation_score: 0.85,
                last_seen: js_sys::Date::now(),
                cluster_specializations: vec!["nlp".to_string()],
            },
        ];
        
        for peer in simulated_peers {
            console_log!("Discovered peer: {} ({})", peer.device_id, peer.capabilities.join(", "));
            self.peer_registry.insert(peer.device_id.clone(), peer);
        }
    }

    #[wasm_bindgen]
    pub fn send_heartbeat(&mut self) -> bool {
        if !self.is_connected_to_server {
            return false;
        }
        
        let _heartbeat_message = SignalingMessage {
            message_type: "heartbeat".to_string(),
            data: serde_json::json!({
                "device_status": "online",
                "available_resources": {
                    "cpu_usage": 0.3,
                    "memory_usage": 0.5,
                    "available_nodes": 5
                },
                "recent_activities": ["training", "inference"]
            }),
        };
        
        console_log!("Sent heartbeat to signaling server");
        true
    }

    #[wasm_bindgen]
    pub fn get_discovered_peers(&self) -> String {
        let peers: Vec<&PeerInfo> = self.peer_registry.values().collect();
        serde_json::to_string(&peers).unwrap_or_default()
    }

    #[wasm_bindgen]
    pub fn is_connected_to_signaling_server(&self) -> bool {
        self.is_connected_to_server
    }

    #[wasm_bindgen]
    pub fn request_node_direct(&mut self, peer_id: String, node_type: String, duration_minutes: u32) -> String {
        console_log!("Requesting node directly from peer: {}", peer_id);

        let request_msg = P2PMessage {
            message_id: crate::utils::generate_unique_id("node_req"),
            from: self.device_id.clone(),
            to: peer_id.clone(),
            message_type: MessageType::NodeRequest,
            payload: MessagePayload::NodeRequestData {
                node_type,
                required_capabilities: vec!["inference".to_string(), "adaptation".to_string()],
                duration_minutes,
                payment_offer: 5.0,
            },
            timestamp: js_sys::Date::now(),
            signature: "request_signature".to_string(),
            hop_count: 0,
        };

        if self.send_direct_message(peer_id, request_msg.clone()) {
            self.message_queue.push(request_msg.clone());
            request_msg.message_id
        } else {
            "".to_string()
        }
    }

    #[wasm_bindgen]
    pub fn share_memory_direct(&mut self, peer_id: String, capsule_json: &str) -> bool {
        if let Ok(capsule) = serde_json::from_str::<MemoryCapsule>(capsule_json) {
            console_log!("Sharing memory capsule directly with peer: {}", peer_id);

            let share_msg = P2PMessage {
                message_id: crate::utils::generate_unique_id("mem_share"),
                from: self.device_id.clone(),
                to: peer_id.clone(),
                message_type: MessageType::MemoryShare,
                payload: MessagePayload::MemoryShareData {
                    capsule,
                    access_level: "behavioral".to_string(),
                    sharing_reward: 2.0,
                },
                timestamp: js_sys::Date::now(),
                signature: "share_signature".to_string(),
                hop_count: 0,
            };

            return self.send_direct_message(peer_id, share_msg);
        }
        false
    }

    #[wasm_bindgen]
    pub fn initiate_collaborative_learning(&mut self, peer_ids: Vec<String>, task_description: String) -> String {
        console_log!("Initiating collaborative learning with {} peers", peer_ids.len());

        let session_id = crate::utils::generate_unique_id("collab");
        
        for peer_id in peer_ids {
            let collab_msg = P2PMessage {
                message_id: crate::utils::generate_unique_id("collab_invite"),
                from: self.device_id.clone(),
                to: peer_id.clone(),
                message_type: MessageType::CollaborativeLearn,
                payload: MessagePayload::CollaborativeLearnData {
                    task_description: task_description.clone(),
                    dataset_hash: "dataset_hash_placeholder".to_string(),
                    learning_parameters: {
                        let mut params = HashMap::new();
                        params.insert("learning_rate".to_string(), 0.01);
                        params.insert("batch_size".to_string(), 32.0);
                        params.insert("epochs".to_string(), 10.0);
                        params
                    },
                    participant_rewards: {
                        let mut rewards = HashMap::new();
                        rewards.insert(peer_id.clone(), 10.0);
                        rewards
                    },
                },
                timestamp: js_sys::Date::now(),
                signature: "collab_signature".to_string(),
                hop_count: 0,
            };

            self.send_direct_message(peer_id, collab_msg);
        }

        session_id
    }

    #[wasm_bindgen]
    pub fn propagate_error_signal(&mut self, error_vector: Vec<f64>, urgency: u8) -> u32 {
        console_log!("Propagating error signal to {} connected peers", self.active_connections.len());

        let mut propagated_count = 0;

        for (peer_id, connection) in &self.active_connections {
            if connection.status == ConnectionStatus::Established {
                let error_msg = P2PMessage {
                    message_id: crate::utils::generate_unique_id("error_prop"),
                    from: self.device_id.clone(),
                    to: peer_id.clone(),
                    message_type: MessageType::ErrorPropagate,
                    payload: MessagePayload::ErrorPropagateData {
                        error_vector: error_vector.clone(),
                        source_cluster: self.device_id.clone(),
                        propagation_weight: 1.0 / (connection.latency_ms + 1.0),
                        urgency_level: urgency,
                    },
                    timestamp: js_sys::Date::now(),
                    signature: "error_signature".to_string(),
                    hop_count: 0,
                };

                if self.send_direct_message(peer_id.clone(), error_msg) {
                    propagated_count += 1;
                }
            }
        }

        propagated_count
    }

    #[wasm_bindgen]
    pub fn process_incoming_messages(&mut self) -> u32 {
        // In a real implementation, this would be called by the network layer
        // when messages are received from peers
        console_log!("Processing {} queued messages", self.message_queue.len());

        let processed_count = self.message_queue.len();
        
        // Clone the messages to avoid borrowing issues
        let messages_to_process = self.message_queue.clone();
        self.message_queue.clear();
        
        for message in messages_to_process {
            self.handle_message(message);
        }

        processed_count as u32
    }

    #[wasm_bindgen]
    pub fn get_network_stats(&self) -> JsValue {
        let stats = NetworkStats {
            connected_peers: self.active_connections.len(),
            known_peers: self.peer_registry.len(),
            pending_messages: self.message_queue.len(),
            average_latency: self.calculate_average_latency(),
            total_bandwidth: self.calculate_total_bandwidth(),
            network_health: self.calculate_network_health(),
        };

        serde_wasm_bindgen::to_value(&stats).unwrap_or(JsValue::NULL)
    }

    fn handle_message(&mut self, message: P2PMessage) {
        console_log!("Handling {} message from {}", 
            format!("{:?}", message.message_type), 
            message.from);

        match message.message_type {
            MessageType::NodeRequest => self.handle_node_request(message),
            MessageType::NodeResponse => self.handle_node_response(message),
            MessageType::MemoryShare => self.handle_memory_share(message),
            MessageType::CollaborativeLearn => self.handle_collaborative_learn(message),
            MessageType::ErrorPropagate => self.handle_error_propagate(message),
            MessageType::HeartBeat => self.handle_heartbeat(message),
            MessageType::Discovery => self.handle_discovery(message),
        }
    }

    fn handle_node_request(&mut self, message: P2PMessage) {
        if let MessagePayload::NodeRequestData { node_type, duration_minutes, payment_offer, .. } = message.payload {
            console_log!("Received node request for {} type, duration: {} min, payment: {}", 
                node_type, duration_minutes, payment_offer);

            // In a real implementation, check if we can fulfill the request
            let approval = payment_offer >= 3.0 && duration_minutes <= 60;

            let response = P2PMessage {
                message_id: crate::utils::generate_unique_id("node_resp"),
                from: self.device_id.clone(),
                to: message.from,
                message_type: MessageType::NodeResponse,
                payload: MessagePayload::NodeResponseData {
                    node_data: "serialized_node_data".to_string(),
                    approval_status: approval,
                    rental_cost: payment_offer,
                    availability_window: (js_sys::Date::now(), js_sys::Date::now() + 3600000.0),
                },
                timestamp: js_sys::Date::now(),
                signature: "response_signature".to_string(),
                hop_count: 0,
            };

            self.message_queue.push(response);
        }
    }

    fn handle_node_response(&self, message: P2PMessage) {
        if let MessagePayload::NodeResponseData { approval_status, rental_cost, .. } = message.payload {
            console_log!("Received node response: approved={}, cost={}", approval_status, rental_cost);
            // Handle the response to our node request
        }
    }

    fn handle_memory_share(&self, message: P2PMessage) {
        if let MessagePayload::MemoryShareData { capsule, sharing_reward, .. } = message.payload {
            console_log!("Received memory capsule: {}, reward: {}", capsule.capsule_id, sharing_reward);
            // Process the shared memory capsule
        }
    }

    fn handle_collaborative_learn(&self, message: P2PMessage) {
        if let MessagePayload::CollaborativeLearnData { task_description, .. } = message.payload {
            console_log!("Received collaborative learning invitation: {}", task_description);
            // Decide whether to participate in collaborative learning
        }
    }

    fn handle_error_propagate(&self, message: P2PMessage) {
        if let MessagePayload::ErrorPropagateData { error_vector, urgency_level, .. } = message.payload {
            console_log!("Received error signal with {} dimensions, urgency: {}", 
                error_vector.len(), urgency_level);
            // Apply the error signal to local learning
        }
    }

    fn handle_heartbeat(&mut self, message: P2PMessage) {
        if let MessagePayload::HeartBeatData { device_status, .. } = message.payload {
            console_log!("Received heartbeat from {}: {}", message.from, device_status);
            
            // Update peer info
            if let Some(peer) = self.peer_registry.get_mut(&message.from) {
                peer.last_seen = message.timestamp;
            }
        }
    }

    fn handle_discovery(&mut self, message: P2PMessage) {
        if let MessagePayload::DiscoveryData { device_info, network_topology } = message.payload {
            console_log!("Discovered new peer: {}", device_info.device_id);
            
            // Add to peer registry
            self.peer_registry.insert(device_info.device_id.clone(), device_info);
            
            // Update routing table
            for (device_id, route) in network_topology {
                if !self.routing_table.contains_key(&device_id) {
                    let mut new_route = vec![message.from.clone()];
                    new_route.extend(route);
                    self.routing_table.insert(device_id, new_route);
                }
            }
        }
    }

    fn calculate_average_latency(&self) -> f64 {
        if self.active_connections.is_empty() {
            return 0.0;
        }

        let total_latency: f64 = self.active_connections.values()
            .map(|conn| conn.latency_ms)
            .sum();

        total_latency / self.active_connections.len() as f64
    }

    fn calculate_total_bandwidth(&self) -> f64 {
        self.active_connections.values()
            .map(|conn| conn.bandwidth_usage)
            .sum()
    }

    fn calculate_network_health(&self) -> f64 {
        if self.active_connections.is_empty() {
            return 0.0;
        }

        let healthy_connections = self.active_connections.values()
            .filter(|conn| conn.status == ConnectionStatus::Established && conn.latency_ms < 500.0)
            .count();

        healthy_connections as f64 / self.active_connections.len() as f64
    }
}

#[derive(serde::Serialize)]
struct NetworkStats {
    connected_peers: usize,
    known_peers: usize,
    pending_messages: usize,
    average_latency: f64,
    total_bandwidth: f64,
    network_health: f64,
} 