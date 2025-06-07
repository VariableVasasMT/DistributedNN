use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    RtcPeerConnection, RtcDataChannel, RtcConfiguration, RtcIceServer,
    RtcSessionDescription, RtcSessionDescriptionInit, RtcSdpType,
    RtcIceCandidate, RtcIceCandidateInit, RtcDataChannelInit,
    MessageEvent, Event, WebSocket, RtcPeerConnectionState
};
use js_sys::{Object, Reflect, Array};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::console_log;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebRTCOffer {
    pub sdp: String,
    pub sdp_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebRTCAnswer {
    pub sdp: String,
    pub sdp_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ICECandidate {
    pub candidate: String,
    pub sdp_mid: Option<String>,
    pub sdp_m_line_index: Option<u16>,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct WebRTCManager {
    device_id: String,
    ice_servers: Vec<String>,
    // Note: We'll manage connections differently to handle clone
    connected_peers: Vec<String>,
}

#[wasm_bindgen]
impl WebRTCManager {
    #[wasm_bindgen(constructor)]
    pub fn new(device_id: String) -> WebRTCManager {
        console_log!("Creating WebRTC Manager for device: {}", device_id);
        
        WebRTCManager {
            device_id,
            ice_servers: vec![
                "stun:stun.l.google.com:19302".to_string(),
                "stun:stun1.l.google.com:19302".to_string(),
            ],
            connected_peers: Vec::new(),
        }
    }

    #[wasm_bindgen]
    pub fn connect_signaling_server(&mut self, server_url: &str) -> Result<(), JsValue> {
        console_log!("Connecting to signaling server: {}", server_url);
        
        let ws = WebSocket::new(server_url)?;
        ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
        
        // Clone device_id for closure
        let device_id = self.device_id.clone();
        
        // On open - register with signaling server
        let onopen_callback = Closure::wrap(Box::new(move |_event: Event| {
            console_log!("Connected to signaling server, registering device");
            // Registration logic will be handled by P2PNetwork
        }) as Box<dyn FnMut(Event)>);
        
        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();
        
        // On message - handle signaling messages
        let onmessage_callback = Closure::wrap(Box::new(move |event: MessageEvent| {
            if let Ok(message) = event.data().dyn_into::<js_sys::JsString>() {
                let message_str = String::from(message);
                console_log!("Received signaling message: {}", message_str);
                // Message handling will be delegated to P2PNetwork
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();
        
        Ok(())
    }

    #[wasm_bindgen]
    pub fn create_peer_connection(&mut self, peer_id: &str) -> Result<(), JsValue> {
        console_log!("Creating peer connection for: {}", peer_id);
        
        // Create ICE server configuration
        let ice_servers = Array::new();
        for server_url in &self.ice_servers {
            let ice_server = Object::new();
            let urls = Array::new();
            urls.push(&JsValue::from_str(server_url));
            Reflect::set(&ice_server, &"urls".into(), &urls)?;
            ice_servers.push(&ice_server);
        }
        
        let config = Object::new();
        Reflect::set(&config, &"iceServers".into(), &ice_servers)?;
        
        // Convert Object to JsValue, then to RtcConfiguration
        let config_value: JsValue = config.into();
        let rtc_config: RtcConfiguration = config_value.into();
        let peer_connection = RtcPeerConnection::new_with_configuration(&rtc_config)?;
        
        // Set up event handlers
        self.setup_peer_connection_handlers(&peer_connection, peer_id)?;
        
        console_log!("Peer connection created successfully for: {}", peer_id);
        Ok(())
    }

    fn setup_peer_connection_handlers(&self, pc: &RtcPeerConnection, peer_id: &str) -> Result<(), JsValue> {
        let peer_id_clone = peer_id.to_string();
        
        // Handle ICE candidates
        let onicecandidate_callback = Closure::wrap(Box::new(move |event: Event| {
            if let Some(candidate_event) = event.dyn_ref::<web_sys::RtcPeerConnectionIceEvent>() {
                if let Some(_ice_candidate) = candidate_event.candidate() {
                    console_log!("Generated ICE candidate for {}", peer_id_clone);
                    // Send candidate via signaling server
                    // This will be handled by the P2PNetwork layer
                }
            }
        }) as Box<dyn FnMut(Event)>);
        
        pc.set_onicecandidate(Some(onicecandidate_callback.as_ref().unchecked_ref()));
        onicecandidate_callback.forget();
        
        // Handle connection state changes
        let peer_id_clone2 = peer_id.to_string();
        let onconnectionstatechange_callback = Closure::wrap(Box::new(move |_event: Event| {
            console_log!("Connection state changed for peer: {}", peer_id_clone2);
        }) as Box<dyn FnMut(Event)>);
        
        pc.set_onconnectionstatechange(Some(onconnectionstatechange_callback.as_ref().unchecked_ref()));
        onconnectionstatechange_callback.forget();
        
        Ok(())
    }

    #[wasm_bindgen]
    pub fn create_data_channel(&mut self, peer_id: &str, channel_name: &str) -> Result<(), JsValue> {
        console_log!("Creating data channel '{}' for peer: {}", channel_name, peer_id);
        
        // For simplified implementation, we'll create a mock peer connection here
        // In a real implementation, you'd retrieve the actual peer connection
        let config = RtcConfiguration::new();
        let pc = RtcPeerConnection::new_with_configuration(&config)?;
        
        let mut options = RtcDataChannelInit::new();
        options.set_ordered(true);
        
        let data_channel = pc.create_data_channel_with_data_channel_dict(channel_name, &options);
        
        // Set up data channel event handlers
        self.setup_data_channel_handlers(&data_channel, peer_id)?;
        
        Ok(())
    }

    fn setup_data_channel_handlers(&self, channel: &RtcDataChannel, peer_id: &str) -> Result<(), JsValue> {
        let peer_id_clone = peer_id.to_string();
        
        // Handle data channel open
        let onopen_callback = Closure::wrap(Box::new(move |_event: Event| {
            console_log!("Data channel opened for peer: {}", peer_id_clone);
        }) as Box<dyn FnMut(Event)>);
        
        channel.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();
        
        // Handle incoming messages
        let peer_id_clone2 = peer_id.to_string();
        let onmessage_callback = Closure::wrap(Box::new(move |event: MessageEvent| {
            if let Ok(message) = event.data().dyn_into::<js_sys::JsString>() {
                let message_str = String::from(message);
                console_log!("Received P2P message from {}: {}", peer_id_clone2, message_str);
                // Message will be handled by P2PNetwork layer
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        
        channel.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();
        
        // Handle errors
        let peer_id_clone3 = peer_id.to_string();
        let onerror_callback = Closure::wrap(Box::new(move |event: Event| {
            console_log!("Data channel error for peer {}: {:?}", peer_id_clone3, event);
        }) as Box<dyn FnMut(Event)>);
        
        channel.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();
        
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn create_offer(&mut self, peer_id: &str) -> Result<String, JsValue> {
        console_log!("Creating offer for peer: {}", peer_id);
        
        // Create a new peer connection for this operation
        let config = RtcConfiguration::new();
        let pc = RtcPeerConnection::new_with_configuration(&config)?;
        
        // Create data channel before creating offer
        self.create_data_channel(peer_id, "data")?;
        
        let offer = wasm_bindgen_futures::JsFuture::from(pc.create_offer()).await?;
        let offer_desc = offer.dyn_into::<RtcSessionDescription>()?;
        
        // Set local description - convert RtcSessionDescription to RtcSessionDescriptionInit
        let mut local_desc = RtcSessionDescriptionInit::new(offer_desc.type_());
        local_desc.set_sdp(&offer_desc.sdp());
        
        let set_local_promise = pc.set_local_description(&local_desc);
        wasm_bindgen_futures::JsFuture::from(set_local_promise).await?;
        
        let webrtc_offer = WebRTCOffer {
            sdp: offer_desc.sdp(),
            sdp_type: "offer".to_string(),
        };
        
        Ok(serde_json::to_string(&webrtc_offer).unwrap())
    }

    #[wasm_bindgen]
    pub async fn create_answer(&mut self, peer_id: &str, offer_json: &str) -> Result<String, JsValue> {
        console_log!("Creating answer for peer: {}", peer_id);
        
        let offer: WebRTCOffer = serde_json::from_str(offer_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid offer JSON: {}", e)))?;
        
        // Create a new peer connection for this operation
        let config = RtcConfiguration::new();
        let pc = RtcPeerConnection::new_with_configuration(&config)?;
        
        // Set remote description (the offer)
        let mut remote_desc = RtcSessionDescriptionInit::new(RtcSdpType::Offer);
        remote_desc.set_sdp(&offer.sdp);
        
        let set_remote_promise = pc.set_remote_description(&remote_desc);
        wasm_bindgen_futures::JsFuture::from(set_remote_promise).await?;
        
        // Create answer
        let answer = wasm_bindgen_futures::JsFuture::from(pc.create_answer()).await?;
        let answer_desc = answer.dyn_into::<RtcSessionDescription>()?;
        
        // Set local description - convert RtcSessionDescription to RtcSessionDescriptionInit
        let mut local_desc = RtcSessionDescriptionInit::new(answer_desc.type_());
        local_desc.set_sdp(&answer_desc.sdp());
        
        let set_local_promise = pc.set_local_description(&local_desc);
        wasm_bindgen_futures::JsFuture::from(set_local_promise).await?;
        
        let webrtc_answer = WebRTCAnswer {
            sdp: answer_desc.sdp(),
            sdp_type: "answer".to_string(),
        };
        
        Ok(serde_json::to_string(&webrtc_answer).unwrap())
    }

    #[wasm_bindgen]
    pub async fn set_remote_answer(&mut self, peer_id: &str, answer_json: &str) -> Result<(), JsValue> {
        console_log!("Setting remote answer for peer: {}", peer_id);
        
        let answer: WebRTCAnswer = serde_json::from_str(answer_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid answer JSON: {}", e)))?;
        
        // Create a new peer connection for this operation
        let config = RtcConfiguration::new();
        let pc = RtcPeerConnection::new_with_configuration(&config)?;
        
        let mut remote_desc = RtcSessionDescriptionInit::new(RtcSdpType::Answer);
        remote_desc.set_sdp(&answer.sdp);
        
        let set_remote_promise = pc.set_remote_description(&remote_desc);
        wasm_bindgen_futures::JsFuture::from(set_remote_promise).await?;
        
        // Add to connected peers
        if !self.connected_peers.contains(&peer_id.to_string()) {
            self.connected_peers.push(peer_id.to_string());
        }
        
        console_log!("Remote answer set successfully for peer: {}", peer_id);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn add_ice_candidate(&mut self, peer_id: &str, candidate_json: &str) -> Result<(), JsValue> {
        console_log!("Adding ICE candidate for peer: {}", peer_id);
        
        let ice_candidate: ICECandidate = serde_json::from_str(candidate_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid candidate JSON: {}", e)))?;
        
        // Create a new peer connection for this operation
        let config = RtcConfiguration::new();
        let pc = RtcPeerConnection::new_with_configuration(&config)?;
        
        let mut candidate_init = RtcIceCandidateInit::new(&ice_candidate.candidate);
        if let Some(mid) = &ice_candidate.sdp_mid {
            candidate_init.set_sdp_mid(Some(mid));
        }
        if let Some(line_index) = ice_candidate.sdp_m_line_index {
            candidate_init.set_sdp_m_line_index(Some(line_index));
        }
        
        let rtc_candidate = RtcIceCandidate::new(&candidate_init)?;
        let add_candidate_promise = pc.add_ice_candidate_with_opt_rtc_ice_candidate(Some(&rtc_candidate));
        wasm_bindgen_futures::JsFuture::from(add_candidate_promise).await?;
        
        console_log!("ICE candidate added successfully for peer: {}", peer_id);
        Ok(())
    }

    #[wasm_bindgen]
    pub fn send_data(&self, peer_id: &str, data: &str) -> Result<(), JsValue> {
        console_log!("Sending data to peer {} via WebRTC (simulated): {}", peer_id, data);
        
        // In a real implementation, this would use the actual data channel
        // For now, we'll simulate successful sending if peer is connected
        if self.connected_peers.contains(&peer_id.to_string()) {
            Ok(())
        } else {
            Err(JsValue::from_str("Peer not connected"))
        }
    }

    #[wasm_bindgen]
    pub fn is_connected(&self, peer_id: &str) -> bool {
        self.connected_peers.contains(&peer_id.to_string())
    }

    #[wasm_bindgen]
    pub fn get_connection_stats(&self) -> String {
        let stats = serde_json::json!({
            "total_connections": self.connected_peers.len(),
            "active_channels": self.connected_peers.len(),
            "connected_peers": self.connected_peers.len()
        });
        
        serde_json::to_string(&stats).unwrap_or_default()
    }

    #[wasm_bindgen]
    pub fn close_connection(&mut self, peer_id: &str) -> Result<(), JsValue> {
        console_log!("Closing connection to peer: {}", peer_id);
        
        // Remove from connected peers
        self.connected_peers.retain(|id| id != peer_id);
        
        console_log!("Connection closed for peer: {}", peer_id);
        Ok(())
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
} 