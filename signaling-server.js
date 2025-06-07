#!/usr/bin/env node

const WebSocket = require('ws');
const http = require('http');
const url = require('url');

/**
 * Signaling Server for Distributed Neural Network P2P Discovery
 * Handles peer registration, discovery, and WebRTC signaling ONLY
 * After initial handshake, all data flows directly between peers via WebRTC
 */

const PORT = process.env.PORT || 8080;
const server = http.createServer();
const wss = new WebSocket.Server({ server });

// Store connected peers
const peers = new Map(); // device_id -> peer_info
const connections = new Map(); // ws -> device_id

// Store peer capabilities and status
const peerRegistry = new Map(); // device_id -> { ws, info, lastSeen, status }

console.log(`🌐 Distributed Neural Network Signaling Server`);
console.log(`🚀 Starting on port ${PORT}...`);

wss.on('connection', (ws, request) => {
    const clientIP = request.socket.remoteAddress;
    console.log(`📱 New connection from ${clientIP}`);
    
    ws.on('message', (data) => {
        try {
            const message = JSON.parse(data);
            handleMessage(ws, message);
        } catch (error) {
            console.error('❌ Invalid JSON:', error);
            ws.send(JSON.stringify({
                type: 'error',
                message: 'Invalid JSON format'
            }));
        }
    });
    
    ws.on('close', () => {
        handleDisconnection(ws);
    });
    
    ws.on('error', (error) => {
        console.error('❌ WebSocket error:', error);
        handleDisconnection(ws);
    });
});

function handleMessage(ws, message) {
    const { type, data } = message;
    
    switch (type) {
        case 'register':
            handlePeerRegistration(ws, data);
            break;
            
        case 'discover':
            handlePeerDiscovery(ws, data);
            break;
            
        case 'signal':
            handleWebRTCSignaling(ws, data);
            break;
            
        case 'announce_capability':
            handleCapabilityAnnouncement(ws, data);
            break;
            
        case 'heartbeat':
            handleHeartbeat(ws, data);
            break;
            
        default:
            console.log(`⚠️ Unknown message type: ${type}`);
            ws.send(JSON.stringify({
                type: 'error',
                message: `Unknown message type: ${type}. Note: Data messages should go directly via WebRTC after initial connection.`
            }));
    }
}

function handlePeerRegistration(ws, data) {
    const { device_id, peer_info } = data;
    
    if (!device_id || !peer_info) {
        ws.send(JSON.stringify({
            type: 'error',
            message: 'Missing device_id or peer_info'
        }));
        return;
    }
    
    // Register the peer
    const registrationInfo = {
        ws,
        info: {
            ...peer_info,
            device_id,
            registered_at: Date.now(),
            last_seen: Date.now()
        },
        status: 'online'
    };
    
    peerRegistry.set(device_id, registrationInfo);
    connections.set(ws, device_id);
    
    console.log(`✅ Registered peer: ${device_id}`);
    console.log(`📊 Total peers: ${peerRegistry.size}`);
    
    // Send confirmation
    ws.send(JSON.stringify({
        type: 'registered',
        data: {
            device_id,
            server_time: Date.now(),
            peer_count: peerRegistry.size
        }
    }));
    
    // Broadcast new peer to others
    broadcastToAllExcept(ws, {
        type: 'peer_joined',
        data: {
            device_id,
            peer_info: registrationInfo.info,
            total_peers: peerRegistry.size
        }
    });
}

function handlePeerDiscovery(ws, data) {
    const device_id = connections.get(ws);
    if (!device_id) {
        ws.send(JSON.stringify({
            type: 'error',
            message: 'Not registered'
        }));
        return;
    }
    
    const { filters } = data || {};
    
    // Get all peers except the requesting one
    const availablePeers = [];
    for (const [peerId, peerData] of peerRegistry) {
        if (peerId !== device_id && peerData.status === 'online') {
            let matchesFilter = true;
            
            // Apply capability filters
            if (filters?.required_capabilities) {
                const hasCapabilities = filters.required_capabilities.every(cap =>
                    peerData.info.capabilities?.includes(cap)
                );
                if (!hasCapabilities) matchesFilter = false;
            }
            
            // Apply specialization filters
            if (filters?.specializations) {
                const hasSpecialization = filters.specializations.some(spec =>
                    peerData.info.cluster_specializations?.includes(spec)
                );
                if (!hasSpecialization) matchesFilter = false;
            }
            
            if (matchesFilter) {
                availablePeers.push({
                    device_id: peerId,
                    ...peerData.info,
                    // Don't send WebSocket connection info
                    ws: undefined
                });
            }
        }
    }
    
    console.log(`🔍 Discovery request from ${device_id}, found ${availablePeers.length} peers`);
    
    ws.send(JSON.stringify({
        type: 'discovery_result',
        data: {
            peers: availablePeers,
            total_found: availablePeers.length,
            timestamp: Date.now()
        }
    }));
}

function handleWebRTCSignaling(ws, data) {
    const { target_device_id, signaling_data } = data;
    const source_device_id = connections.get(ws);
    
    if (!source_device_id) {
        ws.send(JSON.stringify({
            type: 'error',
            message: 'Not registered'
        }));
        return;
    }
    
    const targetPeer = peerRegistry.get(target_device_id);
    if (!targetPeer || targetPeer.status !== 'online') {
        ws.send(JSON.stringify({
            type: 'error',
            message: 'Target peer not found or offline'
        }));
        return;
    }
    
    // Forward WebRTC signaling data to target peer
    const signalType = signaling_data.type || 'unknown';
    console.log(`📡 Forwarding WebRTC ${signalType}: ${source_device_id} → ${target_device_id}`);
    
    targetPeer.ws.send(JSON.stringify({
        type: 'webrtc_signal',
        data: {
            from_device_id: source_device_id,
            signaling_data
        }
    }));
}

function handleCapabilityAnnouncement(ws, data) {
    const device_id = connections.get(ws);
    if (!device_id) return;
    
    const peer = peerRegistry.get(device_id);
    if (peer) {
        peer.info.capabilities = data.capabilities || [];
        peer.info.cluster_specializations = data.specializations || [];
        peer.info.available_resources = data.resources || {};
        
        console.log(`📢 Updated capabilities for ${device_id}`);
        
        // Broadcast capability update
        broadcastToAllExcept(ws, {
            type: 'peer_updated',
            data: {
                device_id,
                capabilities: peer.info.capabilities,
                specializations: peer.info.cluster_specializations
            }
        });
    }
}

function handleHeartbeat(ws, data) {
    const device_id = connections.get(ws);
    if (!device_id) return;
    
    const peer = peerRegistry.get(device_id);
    if (peer) {
        peer.info.last_seen = Date.now();
        peer.status = 'online';
        
        // Send heartbeat response
        ws.send(JSON.stringify({
            type: 'heartbeat_ack',
            data: {
                server_time: Date.now(),
                peer_count: peerRegistry.size
            }
        }));
    }
}

function handleDisconnection(ws) {
    const device_id = connections.get(ws);
    if (device_id) {
        const peer = peerRegistry.get(device_id);
        if (peer) {
            peer.status = 'offline';
            console.log(`📴 Peer disconnected: ${device_id}`);
            
            // Broadcast disconnection
            broadcastToAllExcept(ws, {
                type: 'peer_left',
                data: {
                    device_id,
                    total_peers: Array.from(peerRegistry.values())
                        .filter(p => p.status === 'online').length
                }
            });
        }
        
        connections.delete(ws);
    }
}

function broadcastToAllExcept(excludeWs, message) {
    for (const [peerId, peerData] of peerRegistry) {
        if (peerData.ws !== excludeWs && peerData.status === 'online') {
            try {
                peerData.ws.send(JSON.stringify(message));
            } catch (error) {
                console.error(`❌ Failed to send to ${peerId}:`, error);
                peerData.status = 'offline';
            }
        }
    }
}

// Cleanup offline peers periodically
setInterval(() => {
    const now = Date.now();
    const timeout = 60000; // 1 minute timeout
    
    for (const [device_id, peer] of peerRegistry) {
        if (now - peer.info.last_seen > timeout && peer.status === 'online') {
            peer.status = 'offline';
            console.log(`⏰ Peer timed out: ${device_id}`);
            
            broadcastToAllExcept(peer.ws, {
                type: 'peer_left',
                data: {
                    device_id,
                    reason: 'timeout'
                }
            });
        }
    }
}, 30000); // Check every 30 seconds

// Server statistics endpoint
server.on('request', (req, res) => {
    const pathname = url.parse(req.url).pathname;
    
    if (pathname === '/stats') {
        const onlinePeers = Array.from(peerRegistry.values())
            .filter(p => p.status === 'online');
        
        const stats = {
            total_peers: peerRegistry.size,
            online_peers: onlinePeers.length,
            server_uptime: process.uptime(),
            memory_usage: process.memoryUsage(),
            peers: onlinePeers.map(p => ({
                device_id: p.info.device_id,
                capabilities: p.info.capabilities,
                specializations: p.info.cluster_specializations,
                last_seen: p.info.last_seen
            }))
        };
        
        res.writeHead(200, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify(stats, null, 2));
    } else {
        res.writeHead(404);
        res.end('Not Found');
    }
});

server.listen(PORT, () => {
    console.log(`✅ Signaling server running on port ${PORT}`);
    console.log(`📊 Stats available at http://localhost:${PORT}/stats`);
    console.log(`🔗 WebSocket endpoint: ws://localhost:${PORT}`);
});

// Graceful shutdown
process.on('SIGTERM', () => {
    console.log('🛑 Shutting down gracefully...');
    server.close(() => {
        process.exit(0);
    });
}); 