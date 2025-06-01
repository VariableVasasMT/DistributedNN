use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::memory::MemoryCapsule;
use crate::utils::generate_unique_id;

// Import the console_log macro
use crate::console_log;

/// Blockchain-based smart contract system for distributed neural network
/// Handles incentives, permissions, auditability, and memory registration
#[wasm_bindgen]
#[derive(Clone)]
pub struct BlockchainLedger {
    blocks: Vec<Block>,
    pending_transactions: Vec<Transaction>,
    smart_contracts: HashMap<String, SmartContract>,
    account_balances: HashMap<String, f64>, // device_id -> credits
    memory_registry: HashMap<String, MemoryRecord>, // capsule_id -> record
    node_borrowing_registry: HashMap<String, BorrowingRecord>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: f64,
    pub previous_hash: String,
    pub hash: String,
    pub transactions: Vec<Transaction>,
    pub merkle_root: String,
    pub nonce: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub tx_id: String,
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub tx_type: TransactionType,
    pub timestamp: f64,
    pub signature: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TransactionType {
    MemoryUpload,     // Reward for uploading memory capsules
    NodeBorrowing,    // Payment for borrowing nodes
    ContributionReward, // Reward for network contributions
    PenaltyCharge,    // Penalty for network violations
    ContractExecution, // Smart contract execution
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SmartContract {
    pub contract_id: String,
    pub contract_type: ContractType,
    pub creator: String,
    pub code: String, // Simplified contract logic
    pub state: HashMap<String, String>,
    pub is_active: bool,
    pub execution_cost: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ContractType {
    IncentiveDistribution,
    MemoryValidation,
    NodeBorrowingPermission,
    QualityAssessment,
    NetworkGovernance,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryRecord {
    pub capsule_id: String,
    pub uploader: String,
    pub timestamp: f64,
    pub hash: String,
    pub privacy_level: String,
    pub incentive_earned: f64,
    pub access_permissions: Vec<String>,
    pub quality_score: f64,
    pub usage_count: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BorrowingRecord {
    pub borrowing_id: String,
    pub borrower: String,
    pub node_owner: String,
    pub node_id: String,
    pub start_time: f64,
    pub duration: f64,
    pub cost: f64,
    pub status: BorrowingStatus,
    pub performance_metrics: HashMap<String, f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BorrowingStatus {
    Requested,
    Approved,
    Active,
    Completed,
    Disputed,
}

#[wasm_bindgen]
impl BlockchainLedger {
    #[wasm_bindgen(constructor)]
    pub fn new() -> BlockchainLedger {
        let mut ledger = BlockchainLedger {
            blocks: Vec::new(),
            pending_transactions: Vec::new(),
            smart_contracts: HashMap::new(),
            account_balances: HashMap::new(),
            memory_registry: HashMap::new(),
            node_borrowing_registry: HashMap::new(),
        };

        // Create genesis block
        ledger.create_genesis_block();
        
        // Deploy default smart contracts
        ledger.deploy_default_contracts();
        
        console_log!("Blockchain ledger initialized with genesis block");
        ledger
    }

    fn create_genesis_block(&mut self) {
        let genesis_block = Block {
            index: 0,
            timestamp: js_sys::Date::now(),
            previous_hash: "0".to_string(),
            hash: "genesis_hash".to_string(),
            transactions: Vec::new(),
            merkle_root: "genesis_merkle".to_string(),
            nonce: 0,
        };
        
        self.blocks.push(genesis_block);
    }

    fn deploy_default_contracts(&mut self) {
        // Incentive distribution contract
        let incentive_contract = SmartContract {
            contract_id: "incentive_distributor".to_string(),
            contract_type: ContractType::IncentiveDistribution,
            creator: "system".to_string(),
            code: "if memory_quality > 0.7 { reward = base_reward * quality_multiplier }".to_string(),
            state: HashMap::new(),
            is_active: true,
            execution_cost: 0.01,
        };
        self.smart_contracts.insert("incentive_distributor".to_string(), incentive_contract);

        // Memory validation contract
        let memory_contract = SmartContract {
            contract_id: "memory_validator".to_string(),
            contract_type: ContractType::MemoryValidation,
            creator: "system".to_string(),
            code: "validate_memory_capsule_integrity_and_privacy".to_string(),
            state: HashMap::new(),
            is_active: true,
            execution_cost: 0.005,
        };
        self.smart_contracts.insert("memory_validator".to_string(), memory_contract);

        // Node borrowing permission contract
        let borrowing_contract = SmartContract {
            contract_id: "node_borrowing_manager".to_string(),
            contract_type: ContractType::NodeBorrowingPermission,
            creator: "system".to_string(),
            code: "check_borrower_credits_and_reputation_before_approval".to_string(),
            state: HashMap::new(),
            is_active: true,
            execution_cost: 0.02,
        };
        self.smart_contracts.insert("node_borrowing_manager".to_string(), borrowing_contract);
    }

    #[wasm_bindgen]
    pub fn register_device(&mut self, device_id: String, initial_credits: f64) -> bool {
        self.account_balances.insert(device_id.clone(), initial_credits);
        
        let tx = Transaction {
            tx_id: generate_unique_id("genesis"),
            from: "system".to_string(),
            to: device_id.clone(),
            amount: initial_credits,
            tx_type: TransactionType::ContributionReward,
            timestamp: js_sys::Date::now(),
            signature: "system_signature".to_string(),
            metadata: HashMap::new(),
        };
        
        self.pending_transactions.push(tx);
        console_log!("Registered device {} with {} initial credits", device_id, initial_credits);
        true
    }

    #[wasm_bindgen]
    pub fn register_memory_capsule(&mut self, capsule_json: &str, uploader: String) -> String {
        if let Ok(capsule) = serde_json::from_str::<MemoryCapsule>(capsule_json) {
            // Execute memory validation contract
            let quality_score = self.execute_memory_validation_contract(&capsule);
            
            // Calculate incentive based on quality and novelty
            let base_reward = 1.0;
            let quality_multiplier = quality_score;
            let novelty_multiplier = capsule.novelty_score;
            let incentive = base_reward * quality_multiplier * novelty_multiplier;
            
            // Create memory record
            let memory_record = MemoryRecord {
                capsule_id: capsule.capsule_id.clone(),
                uploader: uploader.clone(),
                timestamp: capsule.timestamp,
                hash: self.calculate_hash(&capsule_json),
                privacy_level: format!("{:?}", capsule.privacy_level),
                incentive_earned: incentive,
                access_permissions: vec![uploader.clone()], // Default: only uploader can access
                quality_score,
                usage_count: 0,
            };
            
            self.memory_registry.insert(capsule.capsule_id.clone(), memory_record);
            
            // Create incentive transaction
            let tx = Transaction {
                tx_id: generate_unique_id("mem"),
                from: "system".to_string(),
                to: uploader.clone(),
                amount: incentive,
                tx_type: TransactionType::MemoryUpload,
                timestamp: js_sys::Date::now(),
                signature: "contract_signature".to_string(),
                metadata: {
                    let mut meta = HashMap::new();
                    meta.insert("capsule_id".to_string(), capsule.capsule_id.clone());
                    meta.insert("quality_score".to_string(), quality_score.to_string());
                    meta
                },
            };
            
            self.pending_transactions.push(tx);
            
            // Update account balance
            *self.account_balances.entry(uploader.clone()).or_insert(0.0) += incentive;
            
            console_log!("Registered memory capsule {} with incentive {}", capsule.capsule_id, incentive);
            capsule.capsule_id
        } else {
            "".to_string()
        }
    }

    #[wasm_bindgen]
    pub fn request_node_borrowing(&mut self, borrower: String, node_owner: String, node_id: String, duration: f64) -> String {
        // Check borrower's credits and reputation
        let borrower_balance = self.account_balances.get(&borrower).copied().unwrap_or(0.0);
        let cost_per_hour = 0.5;
        let total_cost = cost_per_hour * duration;
        
        if borrower_balance < total_cost {
            console_log!("Insufficient credits for borrowing. Required: {}, Available: {}", total_cost, borrower_balance);
            return "".to_string();
        }
        
        // Execute borrowing permission contract
        let approval = self.execute_borrowing_permission_contract(&borrower, &node_owner, &node_id);
        
        if !approval {
            console_log!("Borrowing request denied by smart contract");
            return "".to_string();
        }
        
        let borrowing_id = generate_unique_id("borrow");
        let borrowing_record = BorrowingRecord {
            borrowing_id: borrowing_id.clone(),
            borrower: borrower.clone(),
            node_owner: node_owner.clone(),
            node_id: node_id.clone(),
            start_time: js_sys::Date::now(),
            duration,
            cost: total_cost,
            status: BorrowingStatus::Approved,
            performance_metrics: HashMap::new(),
        };
        
        self.node_borrowing_registry.insert(borrowing_id.clone(), borrowing_record);
        
        // Create payment transaction
        let tx = Transaction {
            tx_id: generate_unique_id("borrow_pay"),
            from: borrower.clone(),
            to: node_owner.clone(),
            amount: total_cost,
            tx_type: TransactionType::NodeBorrowing,
            timestamp: js_sys::Date::now(),
            signature: "borrower_signature".to_string(),
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("borrowing_id".to_string(), borrowing_id.clone());
                meta.insert("node_id".to_string(), node_id);
                meta.insert("duration".to_string(), duration.to_string());
                meta
            },
        };
        
        self.pending_transactions.push(tx);
        
        // Update balances
        *self.account_balances.entry(borrower).or_insert(0.0) -= total_cost;
        *self.account_balances.entry(node_owner).or_insert(0.0) += total_cost;
        
        console_log!("Approved node borrowing request: {}", borrowing_id);
        borrowing_id
    }

    #[wasm_bindgen]
    pub fn complete_node_borrowing(&mut self, borrowing_id: String, performance_data: &str) -> bool {
        if let Some(mut record) = self.node_borrowing_registry.get(&borrowing_id).cloned() {
            record.status = BorrowingStatus::Completed;
            
            // Parse performance data
            if let Ok(metrics) = serde_json::from_str::<HashMap<String, f64>>(performance_data) {
                record.performance_metrics = metrics;
                
                // Calculate performance bonus/penalty
                let avg_performance = record.performance_metrics.values().sum::<f64>() 
                    / record.performance_metrics.len() as f64;
                
                if avg_performance > 0.8 {
                    // Bonus for good performance
                    let bonus = record.cost * 0.1;
                    *self.account_balances.entry(record.borrower.clone()).or_insert(0.0) += bonus;
                    
                    let bonus_tx = Transaction {
                        tx_id: generate_unique_id("bonus"),
                        from: "system".to_string(),
                        to: record.borrower.clone(),
                        amount: bonus,
                        tx_type: TransactionType::ContributionReward,
                        timestamp: js_sys::Date::now(),
                        signature: "system_signature".to_string(),
                        metadata: HashMap::new(),
                    };
                    
                    self.pending_transactions.push(bonus_tx);
                    console_log!("Performance bonus awarded: {}", bonus);
                }
            }
            
            self.node_borrowing_registry.insert(borrowing_id, record);
            true
        } else {
            false
        }
    }

    #[wasm_bindgen]
    pub fn mine_block(&mut self) -> String {
        if self.pending_transactions.is_empty() {
            return "".to_string();
        }
        
        let previous_block = self.blocks.last().unwrap();
        let new_block = Block {
            index: previous_block.index + 1,
            timestamp: js_sys::Date::now(),
            previous_hash: previous_block.hash.clone(),
            hash: self.calculate_block_hash(previous_block.index + 1, &self.pending_transactions),
            transactions: self.pending_transactions.clone(),
            merkle_root: self.calculate_merkle_root(&self.pending_transactions),
            nonce: self.find_nonce(),
        };
        
        self.blocks.push(new_block.clone());
        self.pending_transactions.clear();
        
        console_log!("Mined new block #{} with {} transactions", new_block.index, new_block.transactions.len());
        new_block.hash
    }

    fn execute_memory_validation_contract(&self, capsule: &MemoryCapsule) -> f64 {
        // Simplified validation logic
        let mut quality_score: f64 = 0.5; // Base score
        
        // Check novelty
        if capsule.novelty_score > 0.7 {
            quality_score += 0.2;
        }
        
        // Check importance
        if capsule.importance_score > 0.8 {
            quality_score += 0.2;
        }
        
        // Check semantic tags richness
        if capsule.semantic_tags.len() > 3 {
            quality_score += 0.1;
        }
        
        quality_score.min(1.0)
    }

    fn execute_borrowing_permission_contract(&self, borrower: &str, _node_owner: &str, _node_id: &str) -> bool {
        // Check borrower's credit history and reputation
        let borrower_balance = self.account_balances.get(borrower).copied().unwrap_or(0.0);
        
        // Simple approval logic
        borrower_balance > 1.0 // Must have at least 1 credit
    }

    fn calculate_hash(&self, data: &str) -> String {
        // Simplified hash function
        format!("hash_{}", crate::utils::simple_hash(data))
    }

    fn calculate_block_hash(&self, index: u64, transactions: &[Transaction]) -> String {
        let tx_data = serde_json::to_string(transactions).unwrap_or_default();
        format!("block_{}_{}", index, crate::utils::simple_hash(&tx_data))
    }

    fn calculate_merkle_root(&self, transactions: &[Transaction]) -> String {
        if transactions.is_empty() {
            return "empty_merkle".to_string();
        }
        
        // Simplified merkle root calculation
        let tx_hashes: Vec<String> = transactions.iter()
            .map(|tx| self.calculate_hash(&tx.tx_id))
            .collect();
        
        format!("merkle_{}", crate::utils::simple_hash(&tx_hashes.join("")))
    }

    fn find_nonce(&self) -> u64 {
        // Simplified proof-of-work (not secure, just for demonstration)
        use rand::Rng;
        rand::thread_rng().gen_range(1000..9999)
    }

    #[wasm_bindgen]
    pub fn get_account_balance(&self, device_id: &str) -> f64 {
        self.account_balances.get(device_id).copied().unwrap_or(0.0)
    }

    #[wasm_bindgen]
    pub fn get_memory_record(&self, capsule_id: &str) -> String {
        if let Some(record) = self.memory_registry.get(capsule_id) {
            serde_json::to_string(record).unwrap_or_default()
        } else {
            "".to_string()
        }
    }

    #[wasm_bindgen]
    pub fn get_blockchain_stats(&self) -> JsValue {
        let stats = BlockchainStats {
            total_blocks: self.blocks.len(),
            total_transactions: self.blocks.iter().map(|b| b.transactions.len()).sum(),
            pending_transactions: self.pending_transactions.len(),
            total_accounts: self.account_balances.len(),
            total_memory_capsules: self.memory_registry.len(),
            total_borrowing_records: self.node_borrowing_registry.len(),
            total_smart_contracts: self.smart_contracts.len(),
        };
        
        serde_wasm_bindgen::to_value(&stats).unwrap_or(JsValue::NULL)
    }

    #[wasm_bindgen]
    pub fn validate_chain(&self) -> bool {
        if self.blocks.len() < 2 {
            return true;
        }
        
        for i in 1..self.blocks.len() {
            let current = &self.blocks[i];
            let previous = &self.blocks[i - 1];
            
            if current.previous_hash != previous.hash {
                return false;
            }
            
            if current.index != previous.index + 1 {
                return false;
            }
        }
        
        true
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockchainStats {
    pub total_blocks: usize,
    pub total_transactions: usize,
    pub pending_transactions: usize,
    pub total_accounts: usize,
    pub total_memory_capsules: usize,
    pub total_borrowing_records: usize,
    pub total_smart_contracts: usize,
} 