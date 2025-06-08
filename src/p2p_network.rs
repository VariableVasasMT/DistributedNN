use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::memory::MemoryCapsule;
use crate::webrtc::WebRTCManager;
use web_sys::{WebSocket, MessageEvent, CloseEvent, ErrorEvent};
use wasm_bindgen::closure::Closure;

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
    websocket: Option<WebSocket>,
    websocket_callbacks: Option<WebSocketCallbacks>,
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
    pub node_status: NodeStatus,
    pub active_connections: u32,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub available_nodes: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeStatus {
    pub is_processing: bool,
    pub active_queries: u32,
    pub last_activity: f64,
    pub processing_load: f64, // 0.0 to 1.0
    pub is_available: bool,
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

#[derive(Clone)]
struct WebSocketCallbacks {
    // We'll store callback handles here
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
            websocket: None,
            websocket_callbacks: None,
        }
    }

    #[wasm_bindgen]
    pub fn configure_signaling_server(&mut self, server_url: String) -> bool {
        console_log!("Connecting to real signaling server: {}", server_url);
        
        // Close existing WebSocket if any
        if let Some(ref ws) = self.websocket {
            ws.close().ok();
        }
        
        // Create new WebSocket connection
        match WebSocket::new(&server_url) {
            Ok(ws) => {
                self.websocket = Some(ws.clone());
                self.signaling_server_url = server_url.clone();
                
                // Set up event handlers
                self.setup_websocket_handlers(&ws);
                
                console_log!("WebSocket connection initiated to: {}", server_url);
                true
            },
            Err(e) => {
                console_log!("Failed to create WebSocket: {:?}", e);
                false
            }
        }
    }
    
    fn setup_websocket_handlers(&mut self, ws: &WebSocket) {
        let device_id = self.device_id.clone();
        
        // OnOpen handler
        let device_id_clone = device_id.clone();
        let ws_for_registration = ws.clone();
        let onopen = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            console_log!("✅ Connected to signaling server");
            
            // Register with the server including node status
            let registration_message = serde_json::json!({
                "type": "register",
                "data": {
                    "device_id": device_id_clone,
                    "peer_info": {
                        "device_id": device_id_clone,
                        "ip_address": "browser_client",
                        "port": 0,
                        "public_key": format!("{}_public_key", device_id_clone),
                        "capabilities": ["memory_sharing", "collaborative_learning", "webrtc_p2p", "neural_processing"],
                        "reputation_score": 1.0,
                        "cluster_specializations": ["general", "browser_based"],
                        "node_status": {
                            "is_processing": false,
                            "active_queries": 0,
                            "last_activity": js_sys::Date::now(),
                            "processing_load": 0.0,
                            "is_available": true
                        },
                        "active_connections": 0,
                        "cpu_usage": 0.2, // Simulated low usage for browser
                        "memory_usage": 0.3, // Simulated low usage for browser  
                        "available_nodes": 5 // Number of neural nodes available for processing
                    }
                }
            });
            
            // Send registration via WebSocket
            if let Ok(message_str) = serde_json::to_string(&registration_message) {
                if let Err(e) = ws_for_registration.send_with_str(&message_str) {
                    console_log!("❌ Failed to send registration: {:?}", e);
                } else {
                    console_log!("📡 Sent registration for device: {}", device_id_clone);
                }
            }
        }) as Box<dyn FnMut(web_sys::Event)>);
        
        ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
        onopen.forget();
        
        // OnMessage handler - use a separate WebSocket clone
        let ws_for_discovery = ws.clone();
        let onmessage = Closure::wrap(Box::new(move |event: MessageEvent| {
            if let Ok(text) = event.data().dyn_into::<js_sys::JsString>() {
                let message_str = text.as_string().unwrap_or_default();
                console_log!("📨 Received signaling message: {}", message_str);
                
                // Parse and handle the message
                if let Ok(message) = serde_json::from_str::<serde_json::Value>(&message_str) {
                    if let Some(msg_type) = message.get("type").and_then(|t| t.as_str()) {
                        console_log!("📋 Processing message type: {}", msg_type);
                        
                        match msg_type {
                            "registered" => {
                                console_log!("✅ Successfully registered with signaling server");
                                if let Some(peer_count) = message.get("data")
                                    .and_then(|d| d.get("peer_count"))
                                    .and_then(|p| p.as_u64()) {
                                    console_log!("📊 Server reports {} total peers", peer_count);
                                    
                                    // Automatically request discovery after successful registration
                                    if peer_count > 1 { // More than just us
                                        console_log!("🔍 Auto-requesting peer discovery after registration");
                                        let discovery_message = serde_json::json!({
                                            "type": "discover",
                                            "data": {
                                                "filters": {
                                                    "required_capabilities": ["memory_sharing"],
                                                    "specializations": ["general"]
                                                }
                                            }
                                        });
                                        
                                        // Send discovery request via the same WebSocket
                                        if let Ok(message_str) = serde_json::to_string(&discovery_message) {
                                            if let Err(e) = ws_for_discovery.send_with_str(&message_str) {
                                                console_log!("❌ Failed to send auto-discovery request: {:?}", e);
                                            } else {
                                                console_log!("✅ Sent auto-discovery request after registration");
                                            }
                                        }
                                    }
                                }
                            },
                            "heartbeat_ack" => {
                                console_log!("💓 Heartbeat acknowledged by signaling server");
                                if let Some(data) = message.get("data") {
                                    if let Some(peer_count) = data.get("peer_count").and_then(|p| p.as_u64()) {
                                        console_log!("📊 Server reports {} total peers online", peer_count);
                                    }
                                    if let Some(status_updated) = data.get("status_updated").and_then(|s| s.as_bool()) {
                                        if status_updated {
                                            console_log!("✅ Node status successfully updated on server");
                                        }
                                    }
                                }
                            },
                            "discovery_result" => {
                                console_log!("🔍 Received real peer discovery results");
                                if let Some(data) = message.get("data") {
                                    if let Some(auto_triggered) = data.get("auto_triggered").and_then(|a| a.as_bool()) {
                                        if auto_triggered {
                                            console_log!("🤖 Auto-triggered discovery from server");
                                            if let Some(reason) = data.get("reason").and_then(|r| r.as_str()) {
                                                console_log!("📋 Reason: {}", reason);
                                            }
                                        }
                                    }
                                    
                                    if let Some(peers) = data.get("peers").and_then(|p| p.as_array()) {
                                        console_log!("🔍 Discovery found {} real peers", peers.len());
                                        
                                        // Convert peers array to JSON string and call JavaScript processor
                                        if let Ok(peers_json) = serde_json::to_string(peers) {
                                            console_log!("📞 Calling JavaScript to process discovery results");
                                            
                                            // Call JavaScript function to handle discovery results
                                            let js_code = format!("window.processDiscoveryResults && window.processDiscoveryResults({})", peers_json);
                                            let _ = js_sys::eval(&js_code);
                                        } else {
                                            console_log!("❌ Failed to serialize peers to JSON");
                                        }
                                        
                                        // For debugging: Process each discovered peer and log details
                                        for peer_data in peers {
                                            if let Ok(peer_info) = serde_json::from_value::<PeerInfo>(peer_data.clone()) {
                                                console_log!("👤 Discovery found peer: {} with capabilities: {:?}", 
                                                    peer_info.device_id, peer_info.capabilities);
                                            }
                                        }
                                    }
                                }
                            },
                            "peer_joined" => {
                                if let Some(device_id) = message.get("data")
                                    .and_then(|d| d.get("device_id"))
                                    .and_then(|id| id.as_str()) {
                                    console_log!("👋 New peer joined: {}", device_id);
                                }
                            },
                            "peer_left" => {
                                if let Some(device_id) = message.get("data")
                                    .and_then(|d| d.get("device_id"))
                                    .and_then(|id| id.as_str()) {
                                    console_log!("👋 Peer left: {}", device_id);
                                }
                            },
                            "webrtc_signal" => {
                                console_log!("📡 Received WebRTC signaling data");
                                // Handle WebRTC signaling (offer/answer/ICE candidates)
                            },
                            _ => {
                                console_log!("❓ Unknown message type: {}", msg_type);
                            }
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        
        ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
        onmessage.forget();
        
        // OnClose handler
        let onclose = Closure::wrap(Box::new(move |_event: CloseEvent| {
            console_log!("🔌 Disconnected from signaling server");
        }) as Box<dyn FnMut(CloseEvent)>);
        
        ws.set_onclose(Some(onclose.as_ref().unchecked_ref()));
        onclose.forget();
        
        // OnError handler
        let onerror = Closure::wrap(Box::new(move |_event: ErrorEvent| {
            console_log!("❌ WebSocket error occurred");
        }) as Box<dyn FnMut(ErrorEvent)>);
        
        ws.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        onerror.forget();
        
        self.is_connected_to_server = true;
    }

    fn send_websocket_message(&self, message: serde_json::Value) -> bool {
        if let Some(ref ws) = self.websocket {
            if let Ok(message_str) = serde_json::to_string(&message) {
                match ws.send_with_str(&message_str) {
                    Ok(_) => {
                        console_log!("📤 Sent WebSocket message: {}", message_str);
                        true
                    },
                    Err(e) => {
                        console_log!("❌ Failed to send WebSocket message: {:?}", e);
                        false
                    }
                }
            } else {
                false
            }
        } else {
            console_log!("❌ No WebSocket connection available");
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
                            self.send_websocket_message(serde_json::json!({
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
                            self.send_websocket_message(serde_json::json!({
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
            console_log!("❌ Cannot start discovery - not connected to signaling server");
            return false;
        }
        
        console_log!("🔍 Starting real peer discovery via signaling server");
        
        // Send discovery request to signaling server
        let discovery_message = serde_json::json!({
            "type": "discover",
            "data": {
                "filters": {
                    "required_capabilities": ["memory_sharing"],
                    "specializations": ["general"]
                }
            }
        });
        
        if self.send_websocket_message(discovery_message) {
            console_log!("✅ Sent discovery request to signaling server");
            self.discovery_protocol.last_discovery = js_sys::Date::now();
            true
        } else {
            console_log!("❌ Failed to send discovery request");
            false
        }
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

    #[wasm_bindgen]
    pub fn send_heartbeat(&mut self) -> bool {
        if !self.is_connected_to_server {
            return false;
        }
        
        // Create comprehensive heartbeat with current node status
        let heartbeat_message = serde_json::json!({
            "type": "heartbeat",
            "data": {
                "device_status": "online",
                "node_status": {
                    "is_processing": false,
                    "active_queries": 0,
                    "last_activity": js_sys::Date::now(),
                    "processing_load": js_sys::Math::random() * 0.2, // 0-20% load
                    "is_available": true
                },
                "available_resources": {
                    "cpu_usage": 0.1 + js_sys::Math::random() * 0.3, // 10-40% CPU
                    "memory_usage": 0.2 + js_sys::Math::random() * 0.3, // 20-50% memory
                    "available_nodes": 8
                },
                "recent_activities": ["neural_processing", "peer_discovery"],
                "capabilities": ["memory_sharing", "collaborative_learning", "webrtc_p2p", "neural_processing"],
                "cluster_specializations": ["general", "browser_based"]
            }
        });
        
        if self.send_websocket_message(heartbeat_message) {
            console_log!("💓 Sent comprehensive heartbeat with node status");
            true
        } else {
            console_log!("⚠️ Failed to send heartbeat");
            false
        }
    }

    #[wasm_bindgen]
    pub fn get_discovered_peers(&self) -> String {
        let peers: Vec<&PeerInfo> = self.peer_registry.values().collect();
        serde_json::to_string(&peers).unwrap_or_default()
    }

    #[wasm_bindgen]
    pub fn handle_discovery_results(&mut self, peers_json: &str) -> bool {
        console_log!("Processing real discovery results: {}", peers_json);
        
        match serde_json::from_str::<Vec<PeerInfo>>(peers_json) {
            Ok(peers) => {
                console_log!("✅ Parsed {} real peers from discovery", peers.len());
                
                // Clear existing peers and add the new ones
                self.peer_registry.clear();
                
                for peer in peers {
                    console_log!("👤 Adding real peer: {} with capabilities: [{}]", 
                        peer.device_id, peer.capabilities.join(", "));
                    
                    // Debug: Log the complete peer data structure
                    console_log!("🔍 Peer {} details:", peer.device_id);
                    console_log!("   - Node Status: is_available={}, is_processing={}, load={:.1}%", 
                        peer.node_status.is_available, peer.node_status.is_processing, 
                        peer.node_status.processing_load * 100.0);
                    console_log!("   - Resources: CPU={:.1}%, Memory={:.1}%, Available nodes={}", 
                        peer.cpu_usage * 100.0, peer.memory_usage * 100.0, peer.available_nodes);
                    console_log!("   - Last seen: {}", peer.last_seen);
                    
                    // Check if this peer would be considered "free"
                    let is_free = peer.node_status.is_available &&
                        !peer.node_status.is_processing &&
                        peer.node_status.active_queries == 0 &&
                        peer.node_status.processing_load < 0.3 &&
                        peer.available_nodes > 0 &&
                        peer.cpu_usage < 0.7 &&
                        peer.memory_usage < 0.8 &&
                        peer.device_id != self.device_id;
                    
                    console_log!("   - Free node check: {} (available={}, not_processing={}, low_queries={}, low_load={}, has_nodes={}, low_cpu={}, low_memory={}, not_self={})",
                        is_free,
                        peer.node_status.is_available,
                        !peer.node_status.is_processing,
                        peer.node_status.active_queries == 0,
                        peer.node_status.processing_load < 0.3,
                        peer.available_nodes > 0,
                        peer.cpu_usage < 0.7,
                        peer.memory_usage < 0.8,
                        peer.device_id != self.device_id
                    );
                    
                    self.peer_registry.insert(peer.device_id.clone(), peer);
                }
                
                true
            },
            Err(e) => {
                console_log!("❌ Failed to parse discovery results: {:?}", e);
                console_log!("❌ Raw JSON was: {}", peers_json);
                false
            }
        }
    }

    #[wasm_bindgen]
    pub fn is_connected_to_signaling_server(&self) -> bool {
        self.is_connected_to_server && self.websocket.is_some()
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

    #[wasm_bindgen]
    pub fn find_free_nodes(&self) -> String {
        console_log!("🔍 Searching for free nodes among {} peers", self.peer_registry.len());
        
        let free_peers: Vec<&PeerInfo> = self.peer_registry.values()
            .filter(|peer| {
                // A node is considered "free" if:
                // 1. It's available and online
                // 2. Not actively processing
                // 3. Has low processing load
                // 4. Has available nodes
                // 5. Low CPU/memory usage
                peer.node_status.is_available &&
                !peer.node_status.is_processing &&
                peer.node_status.active_queries == 0 &&
                peer.node_status.processing_load < 0.3 &&
                peer.available_nodes > 0 &&
                peer.cpu_usage < 0.7 &&
                peer.memory_usage < 0.8 &&
                peer.device_id != self.device_id // Don't connect to ourselves
            })
            .collect();
        
        console_log!("✅ Found {} free nodes available for processing", free_peers.len());
        
        for peer in &free_peers {
            console_log!("🟢 Free node: {} - Load: {:.1}%, CPU: {:.1}%, Memory: {:.1}%, Available nodes: {}", 
                peer.device_id, 
                peer.node_status.processing_load * 100.0,
                peer.cpu_usage * 100.0, 
                peer.memory_usage * 100.0,
                peer.available_nodes
            );
        }
        
        serde_json::to_string(&free_peers).unwrap_or_default()
    }
    
    #[wasm_bindgen]
    pub async fn auto_connect_to_free_node(&mut self) -> String {
        console_log!("🎯 Auto-selecting random free node for connection");
        
        let free_nodes_json = self.find_free_nodes();
        match serde_json::from_str::<Vec<PeerInfo>>(&free_nodes_json) {
            Ok(free_nodes) => {
                if free_nodes.is_empty() {
                    console_log!("❌ No free nodes available for connection");
                    return "".to_string();
                }
                
                // Select random free node
                let random_index = (js_sys::Math::random() * free_nodes.len() as f64) as usize;
                let selected_node = &free_nodes[random_index];
                
                console_log!("🎯 Auto-selected free node: {} (Load: {:.1}%, Available nodes: {})", 
                    selected_node.device_id,
                    selected_node.node_status.processing_load * 100.0,
                    selected_node.available_nodes
                );
                
                // Initiate WebRTC connection to the selected free node
                if self.initiate_webrtc_connection(selected_node.device_id.clone()).await {
                    console_log!("✅ Successfully initiated connection to free node: {}", selected_node.device_id);
                    selected_node.device_id.clone()
                } else {
                    console_log!("❌ Failed to connect to free node: {}", selected_node.device_id);
                    "".to_string()
                }
            },
            Err(e) => {
                console_log!("❌ Failed to parse free nodes: {:?}", e);
                "".to_string()
            }
        }
    }
    
    #[wasm_bindgen]
    pub fn get_node_availability_stats(&self) -> String {
        let total_peers = self.peer_registry.len();
        let free_nodes = serde_json::from_str::<Vec<PeerInfo>>(&self.find_free_nodes())
            .unwrap_or_default()
            .len();
        let busy_nodes = total_peers - free_nodes;
        
        let stats = serde_json::json!({
            "total_peers": total_peers,
            "free_nodes": free_nodes,
            "busy_nodes": busy_nodes,
            "availability_ratio": if total_peers > 0 { free_nodes as f64 / total_peers as f64 } else { 0.0 }
        });
        
        serde_json::to_string(&stats).unwrap_or_default()
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