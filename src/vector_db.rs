use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::memory::MemoryCapsule;
use crate::utils::{cosine_similarity, euclidean_distance};

// Import the console_log macro
use crate::console_log;

/// Blockchain-backed Vector Database for Long-term Memory Storage
/// Implements distributed, persistent memory with semantic search capabilities
#[wasm_bindgen]
#[derive(Clone)]
pub struct VectorMemoryDatabase {
    // Vector index: high-dimensional embeddings for semantic search
    vector_index: HashMap<String, VectorEntry>, // capsule_id -> vector_entry
    
    // Blockchain integration for persistence and auditability
    blockchain_hashes: HashMap<String, String>, // capsule_id -> blockchain_hash
    
    // Hierarchical indexing for fast retrieval
    semantic_clusters: HashMap<String, Vec<String>>, // cluster_tag -> capsule_ids
    temporal_index: Vec<(f64, String)>, // (timestamp, capsule_id) sorted by time
    
    // Quality and importance metrics
    quality_rankings: Vec<(f64, String)>, // (quality_score, capsule_id) sorted by quality
    usage_frequencies: HashMap<String, u32>, // capsule_id -> access_count
    
    // Network-wide statistics
    total_memory_size: usize,
    average_vector_dimension: usize,
    last_consolidation_time: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VectorEntry {
    pub capsule_id: String,
    pub embedding_vector: Vec<f64>, // High-dimensional semantic embedding
    pub metadata_vector: Vec<f64>,  // Compressed metadata features
    pub context_tags: Vec<String>,
    pub timestamp: f64,
    pub quality_score: f64,
    pub importance_score: f64,
    pub access_pattern: AccessPattern,
    pub compression_ratio: f64,
    pub original_size: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccessPattern {
    pub total_accesses: u32,
    pub recent_accesses: Vec<f64>, // Recent access timestamps
    pub access_contexts: Vec<String>, // Context tags when accessed
    pub collaborative_filters: Vec<String>, // Related capsule IDs
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VectorSearchQuery {
    pub query_vector: Vec<f64>,
    pub context_filter: Vec<String>,
    pub time_range: Option<(f64, f64)>,
    pub quality_threshold: f64,
    pub max_results: usize,
    pub search_algorithm: SearchAlgorithm,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SearchAlgorithm {
    CosineSimilarity,
    EuclideanDistance,
    DotProduct,
    Hybrid, // Combines multiple metrics
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub capsule_id: String,
    pub similarity_score: f64,
    pub quality_score: f64,
    pub relevance_score: f64, // Combined similarity + quality + recency
    pub context_match: f64,
    pub blockchain_verified: bool,
}

#[wasm_bindgen]
impl VectorMemoryDatabase {
    #[wasm_bindgen(constructor)]
    pub fn new() -> VectorMemoryDatabase {
        console_log!("Initializing Blockchain Vector Memory Database");
        
        VectorMemoryDatabase {
            vector_index: HashMap::new(),
            blockchain_hashes: HashMap::new(),
            semantic_clusters: HashMap::new(),
            temporal_index: Vec::new(),
            quality_rankings: Vec::new(),
            usage_frequencies: HashMap::new(),
            total_memory_size: 0,
            average_vector_dimension: 0,
            last_consolidation_time: js_sys::Date::now(),
        }
    }

    #[wasm_bindgen]
    pub fn store_memory_capsule(&mut self, capsule_json: &str, blockchain_hash: String) -> bool {
        if let Ok(capsule) = serde_json::from_str::<MemoryCapsule>(capsule_json) {
            // Generate high-dimensional semantic embedding
            let embedding_vector = self.generate_semantic_embedding(&capsule);
            let metadata_vector = self.generate_metadata_vector(&capsule);
            
            let vector_entry = VectorEntry {
                capsule_id: capsule.capsule_id.clone(),
                embedding_vector,
                metadata_vector,
                context_tags: capsule.semantic_tags.clone(),
                timestamp: capsule.timestamp,
                quality_score: self.calculate_enhanced_quality_score(&capsule),
                importance_score: capsule.importance_score,
                access_pattern: AccessPattern {
                    total_accesses: 0,
                    recent_accesses: Vec::new(),
                    access_contexts: Vec::new(),
                    collaborative_filters: Vec::new(),
                },
                compression_ratio: self.calculate_compression_ratio(&capsule),
                original_size: capsule.compressed_data.len(),
            };
            
            // Store in vector index
            self.vector_index.insert(capsule.capsule_id.clone(), vector_entry.clone());
            
            // Store blockchain reference
            self.blockchain_hashes.insert(capsule.capsule_id.clone(), blockchain_hash);
            
            // Update semantic clusters
            self.update_semantic_clusters(&capsule.capsule_id, &capsule.semantic_tags);
            
            // Update temporal index
            self.temporal_index.push((capsule.timestamp, capsule.capsule_id.clone()));
            self.temporal_index.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            
            // Update quality rankings
            self.quality_rankings.push((vector_entry.quality_score, capsule.capsule_id.clone()));
            self.quality_rankings.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
            
            // Update statistics
            self.total_memory_size += capsule.compressed_data.len();
            self.update_average_vector_dimension(&vector_entry.embedding_vector);
            
            console_log!("Stored memory capsule {} in vector database", capsule.capsule_id);
            true
        } else {
            false
        }
    }

    #[wasm_bindgen]
    pub fn semantic_search(&mut self, query_json: &str) -> String {
        if let Ok(query) = serde_json::from_str::<VectorSearchQuery>(query_json) {
            let mut results = Vec::new();
            let mut accessed_capsules = Vec::new();
            
            for (capsule_id, vector_entry) in &self.vector_index {
                // Skip if doesn't match context filter
                if !query.context_filter.is_empty() {
                    let context_match = self.calculate_context_match(&query.context_filter, &vector_entry.context_tags);
                    if context_match < 0.3 {
                        continue;
                    }
                }
                
                // Skip if outside time range
                if let Some((start_time, end_time)) = query.time_range {
                    if vector_entry.timestamp < start_time || vector_entry.timestamp > end_time {
                        continue;
                    }
                }
                
                // Skip if below quality threshold
                if vector_entry.quality_score < query.quality_threshold {
                    continue;
                }
                
                // Calculate similarity based on algorithm
                let similarity_score = match query.search_algorithm {
                    SearchAlgorithm::CosineSimilarity => {
                        cosine_similarity(&query.query_vector, &vector_entry.embedding_vector)
                    },
                    SearchAlgorithm::EuclideanDistance => {
                        1.0 / (1.0 + euclidean_distance(&query.query_vector, &vector_entry.embedding_vector))
                    },
                    SearchAlgorithm::DotProduct => {
                        query.query_vector.iter()
                            .zip(vector_entry.embedding_vector.iter())
                            .map(|(a, b)| a * b)
                            .sum::<f64>()
                    },
                    SearchAlgorithm::Hybrid => {
                        let cosine = cosine_similarity(&query.query_vector, &vector_entry.embedding_vector);
                        let euclidean = 1.0 / (1.0 + euclidean_distance(&query.query_vector, &vector_entry.embedding_vector));
                        (cosine * 0.7) + (euclidean * 0.3)
                    }
                };
                
                // Calculate context match
                let context_match = if query.context_filter.is_empty() {
                    1.0
                } else {
                    self.calculate_context_match(&query.context_filter, &vector_entry.context_tags)
                };
                
                // Calculate recency boost
                let current_time = js_sys::Date::now();
                let age_hours = (current_time - vector_entry.timestamp) / (1000.0 * 3600.0);
                let recency_score = (-age_hours / 168.0).exp(); // Decay over a week
                
                // Calculate combined relevance score
                let relevance_score = (similarity_score * 0.5) + 
                                    (vector_entry.quality_score * 0.3) + 
                                    (context_match * 0.1) + 
                                    (recency_score * 0.1);
                
                // Check blockchain verification
                let blockchain_verified = self.blockchain_hashes.contains_key(capsule_id);
                
                results.push(SearchResult {
                    capsule_id: capsule_id.clone(),
                    similarity_score,
                    quality_score: vector_entry.quality_score,
                    relevance_score,
                    context_match,
                    blockchain_verified,
                });
                
                // Track accessed capsules for later update
                accessed_capsules.push(capsule_id.clone());
            }
            
            // Update access patterns after the search loop
            let current_time = js_sys::Date::now();
            for capsule_id in accessed_capsules {
                if let Some(entry) = self.vector_index.get_mut(&capsule_id) {
                    entry.access_pattern.total_accesses += 1;
                    entry.access_pattern.recent_accesses.push(current_time);
                    
                    // Keep only recent accesses (last 24 hours)
                    entry.access_pattern.recent_accesses.retain(|&time| current_time - time < 86400000.0);
                    
                    // Update usage frequency
                    *self.usage_frequencies.entry(capsule_id).or_insert(0) += 1;
                }
            }
            
            // Sort by relevance score
            results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
            
            // Limit results
            results.truncate(query.max_results);
            
            console_log!("Semantic search returned {} results", results.len());
            serde_json::to_string(&results).unwrap_or_default()
        } else {
            console_log!("Failed to parse search query");
            "[]".to_string()
        }
    }

    #[wasm_bindgen]
    pub fn get_memory_trends(&self) -> String {
        let trends = MemoryTrends {
            total_capsules: self.vector_index.len(),
            total_memory_size: self.total_memory_size,
            average_quality: self.calculate_average_quality(),
            most_accessed_capsules: self.get_most_accessed_capsules(5),
            semantic_cluster_distribution: self.get_cluster_distribution(),
            temporal_distribution: self.get_temporal_distribution(),
            quality_distribution: self.get_quality_distribution(),
            blockchain_verification_rate: self.calculate_blockchain_verification_rate(),
        };
        
        serde_json::to_string(&trends).unwrap_or_default()
    }

    #[wasm_bindgen]
    pub fn consolidate_memory(&mut self) -> bool {
        console_log!("Starting memory consolidation process");
        
        let current_time = js_sys::Date::now();
        
        // Remove old, unused memories (older than 30 days with no recent access)
        let cutoff_time = current_time - (30.0 * 24.0 * 3600.0 * 1000.0);
        let mut to_remove = Vec::new();
        
        for (capsule_id, vector_entry) in &self.vector_index {
            if vector_entry.timestamp < cutoff_time && vector_entry.access_pattern.total_accesses < 3 {
                to_remove.push(capsule_id.clone());
            }
        }
        
        // Track count for logging
        let removed_count = to_remove.len();
        
        // Remove obsolete entries
        for capsule_id in to_remove {
            self.vector_index.remove(&capsule_id);
            self.blockchain_hashes.remove(&capsule_id);
            self.usage_frequencies.remove(&capsule_id);
            
            // Clean up indices
            for cluster_capsules in self.semantic_clusters.values_mut() {
                cluster_capsules.retain(|id| id != &capsule_id);
            }
            
            self.temporal_index.retain(|(_, id)| id != &capsule_id);
            self.quality_rankings.retain(|(_, id)| id != &capsule_id);
        }
        
        // Recompute semantic clusters based on current vectors
        self.recompute_semantic_clusters();
        
        // Update statistics
        self.total_memory_size = self.vector_index.values()
            .map(|entry| entry.original_size)
            .sum();
        
        self.last_consolidation_time = current_time;
        
        console_log!("Memory consolidation completed. Removed {} obsolete entries", removed_count);
        true
    }

    fn generate_semantic_embedding(&self, capsule: &MemoryCapsule) -> Vec<f64> {
        // Generate high-dimensional embedding from memory capsule content
        let mut embedding = vec![0.0; 128]; // 128-dimensional embedding
        
        // Encode context vector
        for (i, &val) in capsule.context_vector.iter().enumerate() {
            if i < 16 {
                embedding[i] = val;
            }
        }
        
        // Encode semantic tags using simple hash-based embedding
        for (i, tag) in capsule.semantic_tags.iter().enumerate() {
            if i < 8 {
                let hash = crate::utils::simple_hash(tag) as f64;
                embedding[16 + i * 14] = (hash % 1000.0) / 1000.0; // Normalize
                
                // Add tag character features
                for (j, byte) in tag.bytes().take(13).enumerate() {
                    embedding[16 + i * 14 + j + 1] = (byte as f64) / 255.0;
                }
            }
        }
        
        // Encode adaptation summary features
        embedding[112] = capsule.adaptation_summary.threshold_adaptations as f64 / 1000.0;
        embedding[113] = capsule.adaptation_summary.timer_adaptations as f64 / 1000.0;
        embedding[114] = capsule.adaptation_summary.weight_changes.abs();
        embedding[115] = capsule.adaptation_summary.error_magnitude;
        embedding[116] = capsule.adaptation_summary.learning_rate_changes;
        
        // Encode temporal and importance features
        embedding[117] = capsule.novelty_score;
        embedding[118] = capsule.importance_score;
        embedding[119] = (capsule.timestamp % 86400000.0) / 86400000.0; // Time of day
        embedding[120] = ((capsule.timestamp / 86400000.0) % 7.0) / 7.0; // Day of week
        
        // Add noise for privacy protection
        for val in embedding.iter_mut().skip(121) {
            *val = rand::random::<f64>() * 0.01; // Small random noise
        }
        
        // Normalize the embedding vector
        let magnitude: f64 = embedding.iter().map(|x| x * x).sum::<f64>().sqrt();
        if magnitude > 0.0 {
            for val in embedding.iter_mut() {
                *val /= magnitude;
            }
        }
        
        embedding
    }

    fn generate_metadata_vector(&self, capsule: &MemoryCapsule) -> Vec<f64> {
        // Generate compressed metadata features
        vec![
            capsule.compressed_data.len() as f64 / 10000.0, // Size feature
            capsule.semantic_tags.len() as f64 / 10.0, // Tag count feature
            match capsule.privacy_level {
                crate::memory::PrivacyLevel::Personal => 1.0,
                crate::memory::PrivacyLevel::Behavioral => 0.5,
                crate::memory::PrivacyLevel::Public => 0.0,
            },
            capsule.novelty_score,
            capsule.importance_score,
        ]
    }

    fn calculate_enhanced_quality_score(&self, capsule: &MemoryCapsule) -> f64 {
        let mut quality = capsule.novelty_score * 0.3 + capsule.importance_score * 0.3;
        
        // Bonus for rich semantic tags
        if capsule.semantic_tags.len() > 3 {
            quality += 0.1;
        }
        
        // Bonus for diverse adaptation metrics
        let adaptation_diversity = (capsule.adaptation_summary.threshold_adaptations > 0) as u8 +
                                 (capsule.adaptation_summary.timer_adaptations > 0) as u8 +
                                 (capsule.adaptation_summary.weight_changes.abs() > 0.1) as u8;
        quality += (adaptation_diversity as f64) * 0.1;
        
        // Penalty for very large or very small capsules
        let size_score = if capsule.compressed_data.len() < 100 || capsule.compressed_data.len() > 100000 {
            0.9
        } else {
            1.0
        };
        
        (quality * size_score).min(1.0).max(0.0)
    }

    fn calculate_compression_ratio(&self, capsule: &MemoryCapsule) -> f64 {
        // Estimate compression ratio (simplified)
        let estimated_uncompressed = capsule.compressed_data.len() as f64 * 3.0; // Assume 3:1 compression
        capsule.compressed_data.len() as f64 / estimated_uncompressed
    }

    fn update_semantic_clusters(&mut self, capsule_id: &str, tags: &[String]) {
        for tag in tags {
            self.semantic_clusters
                .entry(tag.clone())
                .or_insert_with(Vec::new)
                .push(capsule_id.to_string());
        }
    }

    fn calculate_context_match(&self, query_contexts: &[String], entry_contexts: &[String]) -> f64 {
        if query_contexts.is_empty() || entry_contexts.is_empty() {
            return 1.0;
        }
        
        let matches = query_contexts.iter()
            .filter(|q| entry_contexts.contains(q))
            .count();
        
        matches as f64 / query_contexts.len().max(entry_contexts.len()) as f64
    }

    fn update_average_vector_dimension(&mut self, _vector: &[f64]) {
        let total_dims: usize = self.vector_index.values()
            .map(|entry| entry.embedding_vector.len())
            .sum();
        let total_entries = self.vector_index.len();
        
        if total_entries > 0 {
            self.average_vector_dimension = total_dims / total_entries;
        }
    }

    fn calculate_average_quality(&self) -> f64 {
        if self.vector_index.is_empty() {
            return 0.0;
        }
        
        let total_quality: f64 = self.vector_index.values()
            .map(|entry| entry.quality_score)
            .sum();
        
        total_quality / self.vector_index.len() as f64
    }

    fn get_most_accessed_capsules(&self, limit: usize) -> Vec<(String, u32)> {
        let mut usage_vec: Vec<_> = self.usage_frequencies.iter()
            .map(|(id, &count)| (id.clone(), count))
            .collect();
        
        usage_vec.sort_by(|a, b| b.1.cmp(&a.1));
        usage_vec.truncate(limit);
        usage_vec
    }

    fn get_cluster_distribution(&self) -> HashMap<String, usize> {
        self.semantic_clusters.iter()
            .map(|(tag, capsules)| (tag.clone(), capsules.len()))
            .collect()
    }

    fn get_temporal_distribution(&self) -> Vec<(String, usize)> {
        // Group by day
        let mut day_counts: HashMap<String, usize> = HashMap::new();
        
        for &(timestamp, _) in &self.temporal_index {
            let date = js_sys::Date::new(&timestamp.into());
            let day_key = format!("{:04}-{:02}-{:02}", 
                date.get_full_year(), 
                date.get_month() + 1, 
                date.get_date());
            
            *day_counts.entry(day_key).or_insert(0) += 1;
        }
        
        day_counts.into_iter().collect()
    }

    fn get_quality_distribution(&self) -> Vec<(String, usize)> {
        let mut quality_bins = vec![0; 10]; // 10 quality bins (0.0-0.1, 0.1-0.2, etc.)
        
        for entry in self.vector_index.values() {
            let bin = (entry.quality_score * 10.0).floor() as usize;
            if bin < 10 {
                quality_bins[bin] += 1;
            }
        }
        
        quality_bins.into_iter()
            .enumerate()
            .map(|(i, count)| (format!("{:.1}-{:.1}", i as f64 / 10.0, (i + 1) as f64 / 10.0), count))
            .collect()
    }

    pub fn calculate_blockchain_verification_rate(&self) -> f64 {
        if self.vector_index.is_empty() {
            return 0.0;
        }
        
        let verified_count = self.vector_index.keys()
            .filter(|id| self.blockchain_hashes.contains_key(*id))
            .count();
        
        verified_count as f64 / self.vector_index.len() as f64
    }

    fn recompute_semantic_clusters(&mut self) {
        self.semantic_clusters.clear();
        
        for (capsule_id, vector_entry) in &self.vector_index {
            for tag in &vector_entry.context_tags {
                self.semantic_clusters
                    .entry(tag.clone())
                    .or_insert_with(Vec::new)
                    .push(capsule_id.clone());
            }
        }
    }

    // Accessor methods for internal use
    pub fn get_vector_count(&self) -> usize {
        self.vector_index.len()
    }
    
    pub fn get_total_memory_size(&self) -> usize {
        self.total_memory_size
    }
    
    pub fn get_average_vector_dimension(&self) -> usize {
        self.average_vector_dimension
    }
    
    pub fn get_semantic_cluster_count(&self) -> usize {
        self.semantic_clusters.len()
    }
    
    pub fn get_temporal_entry_count(&self) -> usize {
        self.temporal_index.len()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryTrends {
    pub total_capsules: usize,
    pub total_memory_size: usize,
    pub average_quality: f64,
    pub most_accessed_capsules: Vec<(String, u32)>,
    pub semantic_cluster_distribution: HashMap<String, usize>,
    pub temporal_distribution: Vec<(String, usize)>,
    pub quality_distribution: Vec<(String, usize)>,
    pub blockchain_verification_rate: f64,
} 