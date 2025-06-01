use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use rand::Rng;
use std::collections::VecDeque;

/// Core threshold-gating node implementing forward-only learning
/// Based on the research paper's specifications for biological plausibility
#[wasm_bindgen]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThresholdGatingNode {
    // Core accumulator and threshold mechanism
    accumulator: f64,
    threshold: f64,
    timer: f64,
    time_to_release: f64,
    
    // Adaptation parameters
    threshold_adaptation_rate: f64,
    timer_adaptation_rate: f64,
    
    // Eligibility trace for temporal credit assignment
    eligibility_trace: f64,
    eligibility_decay: f64,
    
    // Error signal input (forward-only, no backprop)
    error_input: f64,
    error_sensitivity: f64,
    
    // Memory and history
    activation_history: VecDeque<f64>,
    firing_history: VecDeque<(f64, FiringType)>, // (time, type)
    last_firing_time: f64,
    
    // Node parameters
    node_id: String,
    weights: Vec<f64>,
    bias: f64,
    
    // Adaptation statistics
    threshold_fires: u32,
    timer_fires: u32,
    total_activations: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FiringType {
    Threshold,
    Timer,
}

#[wasm_bindgen]
impl ThresholdGatingNode {
    #[wasm_bindgen(constructor)]
    pub fn new(node_id: String, input_size: usize) -> ThresholdGatingNode {
        let mut rng = rand::thread_rng();
        
        ThresholdGatingNode {
            accumulator: 0.0,
            threshold: rng.gen_range(0.5..2.0), // Initial random threshold
            timer: 0.0,
            time_to_release: rng.gen_range(5.0..15.0), // Initial timer interval
            
            threshold_adaptation_rate: 0.01,
            timer_adaptation_rate: 0.005,
            
            eligibility_trace: 0.0,
            eligibility_decay: 0.95, // Exponential decay factor
            
            error_input: 0.0,
            error_sensitivity: 0.1,
            
            activation_history: VecDeque::with_capacity(100),
            firing_history: VecDeque::with_capacity(50),
            last_firing_time: 0.0,
            
            node_id,
            weights: (0..input_size).map(|_| rng.gen_range(-0.5..0.5)).collect(),
            bias: rng.gen_range(-0.1..0.1),
            
            threshold_fires: 0,
            timer_fires: 0,
            total_activations: 0,
        }
    }

    /// Process input and return output (fires if threshold/timer condition met)
    #[wasm_bindgen]
    pub fn process_input(&mut self, inputs: &[f64], current_time: f64, delta_time: f64) -> f64 {
        self.total_activations += 1;
        
        // Compute weighted input
        let weighted_sum: f64 = inputs.iter()
            .zip(self.weights.iter())
            .map(|(input, weight)| input * weight)
            .sum::<f64>() + self.bias;
        
        // Add to accumulator
        self.accumulator += weighted_sum;
        
        // Update timer
        self.timer += delta_time;
        
        // Update eligibility trace (temporal decay)
        self.eligibility_trace *= self.eligibility_decay;
        
        // Store activation in history
        self.activation_history.push_back(weighted_sum);
        if self.activation_history.len() > 100 {
            self.activation_history.pop_front();
        }
        
        // Check firing conditions
        let mut output = 0.0;
        let mut fired = false;
        let mut firing_type = FiringType::Threshold;
        
        // Check threshold firing
        if self.accumulator >= self.threshold {
            output = self.fire(FiringType::Threshold, current_time);
            fired = true;
            firing_type = FiringType::Threshold;
        }
        // Check timer firing
        else if self.timer >= self.time_to_release {
            output = self.fire(FiringType::Timer, current_time);
            fired = true;
            firing_type = FiringType::Timer;
        }
        
        // Update eligibility trace if fired
        if fired {
            self.eligibility_trace = 1.0; // Reset to maximum on firing
            self.firing_history.push_back((current_time, firing_type));
            if self.firing_history.len() > 50 {
                self.firing_history.pop_front();
            }
        }
        
        output
    }

    /// Fire the node and adapt parameters according to paper's equations
    fn fire(&mut self, firing_type: FiringType, current_time: f64) -> f64 {
        let output = self.accumulator; // Output is the accumulated value
        
        // Adaptation based on firing type (from paper's equations)
        match firing_type {
            FiringType::Threshold => {
                // Increase threshold (node becomes less sensitive)
                self.threshold += self.threshold_adaptation_rate;
                // Increase timer interval (longer patience)
                self.time_to_release += self.timer_adaptation_rate;
                self.threshold_fires += 1;
            },
            FiringType::Timer => {
                // Decrease threshold (node becomes more sensitive)
                self.threshold -= 0.5 * self.threshold_adaptation_rate;
                // Decrease timer interval (less patience), but keep minimum
                self.time_to_release = (self.time_to_release - 0.5 * self.timer_adaptation_rate).max(1.0);
                self.timer_fires += 1;
            }
        }
        
        // Ensure threshold stays positive
        self.threshold = self.threshold.max(0.1);
        
        // Reset accumulator and timer
        self.accumulator = 0.0;
        self.timer = 0.0;
        self.last_firing_time = current_time;
        
        output
    }

    /// Update error signal and adapt learning rates (forward-only)
    #[wasm_bindgen]
    pub fn update_error(&mut self, error: f64) {
        self.error_input = error;
        
        // Modulate adaptation rates based on error and eligibility trace
        let modulation = self.error_sensitivity * error * self.eligibility_trace;
        
        // Update adaptation rates
        self.threshold_adaptation_rate += modulation;
        self.timer_adaptation_rate += 0.5 * modulation;
        
        // Keep adaptation rates in reasonable bounds
        self.threshold_adaptation_rate = self.threshold_adaptation_rate.clamp(0.001, 0.1);
        self.timer_adaptation_rate = self.timer_adaptation_rate.clamp(0.001, 0.05);
        
        // Hebbian-like weight updates based on eligibility and error
        if self.eligibility_trace > 0.1 {
            for weight in &mut self.weights {
                *weight += 0.001 * error * self.eligibility_trace * (*weight).signum();
                *weight = weight.clamp(-2.0, 2.0);
            }
        }
    }

    /// Get current node state for monitoring and debugging
    #[wasm_bindgen]
    pub fn get_state(&self) -> String {
        serde_json::to_string(&NodeState {
            node_id: self.node_id.clone(),
            accumulator: self.accumulator,
            threshold: self.threshold,
            timer: self.timer,
            time_to_release: self.time_to_release,
            eligibility_trace: self.eligibility_trace,
            error_input: self.error_input,
            threshold_fires: self.threshold_fires,
            timer_fires: self.timer_fires,
            total_activations: self.total_activations,
            adaptation_rate: self.threshold_adaptation_rate,
        }).unwrap_or_default()
    }

    // Getters for JavaScript access
    #[wasm_bindgen(getter)]
    pub fn accumulator(&self) -> f64 { self.accumulator }
    
    #[wasm_bindgen(getter)]
    pub fn threshold(&self) -> f64 { self.threshold }
    
    #[wasm_bindgen(getter)]
    pub fn timer(&self) -> f64 { self.timer }
    
    #[wasm_bindgen(getter)]
    pub fn eligibility_trace(&self) -> f64 { self.eligibility_trace }
    
    #[wasm_bindgen(getter)]
    pub fn threshold_fires(&self) -> u32 { self.threshold_fires }
    
    #[wasm_bindgen(getter)]
    pub fn timer_fires(&self) -> u32 { self.timer_fires }
}

#[derive(Serialize, Deserialize)]
struct NodeState {
    node_id: String,
    accumulator: f64,
    threshold: f64,
    timer: f64,
    time_to_release: f64,
    eligibility_trace: f64,
    error_input: f64,
    threshold_fires: u32,
    timer_fires: u32,
    total_activations: u32,
    adaptation_rate: f64,
} 