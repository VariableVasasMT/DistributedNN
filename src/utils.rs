use wasm_bindgen::prelude::*;

// Import the `console.log` function from the browser
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Define a macro for easier console logging
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (crate::utils::console_log_impl(&format_args!($($t)*).to_string()))
}

// Re-export log function for the macro with a different name to avoid conflict
pub fn console_log_impl(s: &str) {
    log(s);
}

// Set up better panic messages for debugging
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// Utility functions for vector operations and data processing
pub fn normalize_vector(vec: &mut [f64]) {
    let magnitude: f64 = vec.iter().map(|x| x * x).sum::<f64>().sqrt();
    if magnitude > 0.0 {
        for val in vec.iter_mut() {
            *val /= magnitude;
        }
    }
}

pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

pub fn tanh_activation(x: f64) -> f64 {
    x.tanh()
}

pub fn relu(x: f64) -> f64 {
    x.max(0.0)
}

pub fn leaky_relu(x: f64, alpha: f64) -> f64 {
    if x > 0.0 { x } else { alpha * x }
}

// Simple hash function for generating semantic tags
pub fn simple_hash(input: &str) -> u64 {
    let mut hash = 0u64;
    for byte in input.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
    }
    hash
}

// Generate a unique ID based on timestamp and random component
pub fn generate_unique_id(prefix: &str) -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let timestamp = js_sys::Date::now() as u64;
    let random_part: u32 = rng.gen();
    format!("{}_{:x}_{:x}", prefix, timestamp, random_part)
}

// Semantic masking for privacy protection
pub fn apply_semantic_mask(text: &str) -> String {
    let mut masked = text.to_string();
    
    // Simple patterns for demonstration
    // In a real implementation, this would use NLP models
    let patterns = vec![
        (r"\b[A-Z][a-z]+ [A-Z][a-z]+\b", "[PERSON_NAME]"),
        (r"\b\d{3}-\d{2}-\d{4}\b", "[SSN]"),
        (r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b", "[IP_ADDRESS]"),
        (r"\b[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}\b", "[EMAIL]"),
        (r"\b\d{4}[\s-]?\d{4}[\s-]?\d{4}[\s-]?\d{4}\b", "[CREDIT_CARD]"),
    ];
    
    for (_pattern, replacement) in patterns {
        // Note: This is a simplified version - real implementation would use regex
        if text.contains("@") && replacement == "[EMAIL]" {
            masked = masked.replace(&text[text.find("@").unwrap_or(0)..], replacement);
        }
    }
    
    masked
}

// Context vector generation utilities
pub fn generate_context_vector(
    activation_history: &[f64], 
    error_history: &[f64], 
    timestamps: &[f64]
) -> Vec<f64> {
    let mut context = vec![0.0; 16];
    
    if !activation_history.is_empty() {
        // Basic statistics
        context[0] = activation_history.iter().sum::<f64>() / activation_history.len() as f64;
        context[1] = activation_history.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        context[2] = activation_history.iter().cloned().fold(f64::INFINITY, f64::min);
        
        // Variance
        let mean = context[0];
        context[3] = activation_history.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / activation_history.len() as f64;
    }
    
    if !error_history.is_empty() {
        context[4] = error_history.iter().sum::<f64>() / error_history.len() as f64;
        context[5] = error_history.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        context[6] = error_history.iter().cloned().fold(f64::INFINITY, f64::min);
    }
    
    if !timestamps.is_empty() && timestamps.len() > 1 {
        // Temporal features
        let mut intervals = Vec::new();
        for i in 1..timestamps.len() {
            intervals.push(timestamps[i] - timestamps[i-1]);
        }
        
        if !intervals.is_empty() {
            context[7] = intervals.iter().sum::<f64>() / intervals.len() as f64; // Avg interval
            context[8] = intervals.iter().cloned().fold(f64::NEG_INFINITY, f64::max); // Max interval
            context[9] = intervals.iter().cloned().fold(f64::INFINITY, f64::min); // Min interval
        }
    }
    
    // Normalize the context vector
    normalize_vector(&mut context);
    context
}

// Time-based decay function for eligibility traces
pub fn exponential_decay(value: f64, decay_rate: f64, time_delta: f64) -> f64 {
    value * (-decay_rate * time_delta).exp()
}

// Calculate entropy for information-theoretic measures
pub fn calculate_entropy(probabilities: &[f64]) -> f64 {
    probabilities.iter()
        .filter(|&&p| p > 0.0)
        .map(|&p| -p * p.log2())
        .sum()
}

// Novelty detection based on distance metrics
pub fn calculate_novelty_score(
    new_pattern: &[f64], 
    historical_patterns: &[Vec<f64>],
    threshold: f64
) -> f64 {
    if historical_patterns.is_empty() {
        return 1.0; // Maximum novelty for first pattern
    }
    
    let min_distance = historical_patterns.iter()
        .map(|pattern| euclidean_distance(new_pattern, pattern))
        .fold(f64::INFINITY, f64::min);
    
    // Convert distance to novelty score (0-1 range)
    (min_distance / threshold).min(1.0)
}

pub fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

pub fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    let dot_product: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let magnitude_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let magnitude_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();
    
    if magnitude_a == 0.0 || magnitude_b == 0.0 {
        0.0
    } else {
        dot_product / (magnitude_a * magnitude_b)
    }
}

// Compress data for memory capsules
pub fn compress_data(data: &[u8]) -> Vec<u8> {
    // Simplified compression - in real implementation would use proper compression
    // For now, just return the data as-is
    data.to_vec()
}

// Decompress data from memory capsules  
pub fn decompress_data(compressed: &[u8]) -> Vec<u8> {
    // Simplified decompression - in real implementation would use proper decompression
    compressed.to_vec()
} 