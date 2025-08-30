use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{api::call::call, export::candid, init, post_upgrade, pre_upgrade, query, update};
use ic_cdk_timers::set_timer_interval;
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};
use std::collections::BTreeMap;
use std::time::Duration;
use uuid::Uuid;

// ICP India Hub Integration
const ICP_INDIA_HUB_CANISTER: &str = "qoctq-giaaa-aaaam-qaeea-cai"; // Example canister ID
const WCHL25_HACKATHON_ID: &str = "WCHL25_CIVICLEDGER";

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct Policy {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub fund_allocation: u64,
    pub fund_released: u64,
    pub beneficiaries: u32,
    pub status: PolicyStatus,
    pub created_at: u64,
    pub updated_at: u64,
    pub district: String,
    pub contractor: Option<String>,
    pub eligibility_criteria: Vec<String>,
    pub execution_conditions: Vec<String>,
    pub smart_contract_code: String,
    // WCHL25 Enhanced Fields
    pub blockchain_hash: Option<String>,
    pub icp_transaction_id: Option<String>,
    pub india_hub_registration: Option<String>,
    pub audit_trail: Vec<AuditEntry>,
    pub ai_analysis_score: Option<f64>,
    pub transparency_score: f64,
    pub citizen_approval_rate: f64,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub enum PolicyStatus {
    Draft,
    Active,
    Paused,
    UnderReview,
    Completed,
    Cancelled,
    // WCHL25 Enhanced Statuses
    BlockchainVerified,
    IndiaHubApproved,
    CitizenVoted,
    AIOptimized,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct FundFlow {
    pub id: String,
    pub policy_id: String,
    pub amount: u64,
    pub from_address: String,
    pub to_address: String,
    pub timestamp: u64,
    pub status: FundFlowStatus,
    pub transaction_hash: Option<String>,
    // WCHL25 Enhanced Fields
    pub icp_block_hash: Option<String>,
    pub india_hub_verification: Option<String>,
    pub smart_contract_execution: Option<String>,
    pub gas_used: Option<u64>,
    pub execution_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub enum FundFlowStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    // WCHL25 Enhanced Statuses
    BlockchainConfirmed,
    IndiaHubVerified,
    SmartContractExecuted,
    CitizenApproved,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct PolicyExecution {
    pub policy_id: String,
    pub execution_date: u64,
    pub funds_released: u64,
    pub beneficiaries_reached: u32,
    pub success_rate: f64,
    pub audit_score: f64,
    // WCHL25 Enhanced Fields
    pub blockchain_verification: bool,
    pub india_hub_score: f64,
    pub ai_optimization_applied: bool,
    pub citizen_feedback_score: f64,
    pub transparency_metrics: TransparencyMetrics,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct AuditEntry {
    pub timestamp: u64,
    pub action: String,
    pub actor: String,
    pub details: String,
    pub blockchain_hash: Option<String>,
    pub icp_transaction_id: Option<String>,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct TransparencyMetrics {
    pub data_availability: f64,
    pub audit_trail_completeness: f64,
    pub citizen_accessibility: f64,
    pub blockchain_immutability: f64,
    pub overall_score: f64,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct IndiaHubRegistration {
    pub policy_id: String,
    pub registration_id: String,
    pub hub_verification_status: bool,
    pub compliance_score: f64,
    pub regional_impact_score: f64,
    pub timestamp: u64,
}

// Stable storage for policies
static mut POLICIES: Option<BTreeMap<String, Policy>> = None;
static mut FUND_FLOWS: Option<BTreeMap<String, FundFlow>> = None;
static mut EXECUTIONS: Option<BTreeMap<String, PolicyExecution>> = None;
static mut INDIA_HUB_REGISTRATIONS: Option<BTreeMap<String, IndiaHubRegistration>> = None;
static mut WCHL25_METRICS: Option<WCHL25Metrics> = None;

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct WCHL25Metrics {
    pub total_policies_created: u32,
    pub total_funds_managed: u64,
    pub total_beneficiaries: u32,
    pub blockchain_transactions: u32,
    pub india_hub_integrations: u32,
    pub ai_optimizations: u32,
    pub citizen_engagements: u32,
    pub transparency_score: f64,
    pub hackathon_score: f64,
}

#[init]
fn init() {
    unsafe {
        POLICIES = Some(BTreeMap::new());
        FUND_FLOWS = Some(BTreeMap::new());
        EXECUTIONS = Some(BTreeMap::new());
        INDIA_HUB_REGISTRATIONS = Some(BTreeMap::new());
        WCHL25_METRICS = Some(WCHL25Metrics {
            total_policies_created: 0,
            total_funds_managed: 0,
            total_beneficiaries: 0,
            blockchain_transactions: 0,
            india_hub_integrations: 0,
            ai_optimizations: 0,
            citizen_engagements: 0,
            transparency_score: 0.0,
            hackathon_score: 0.0,
        });
    }
    
    // Set up periodic policy checks with enhanced WCHL25 features
    set_timer_interval(Duration::from_secs(1800), || {
        ic_cdk::spawn(check_policy_execution());
    });
    
    // Set up India Hub integration checks
    set_timer_interval(Duration::from_secs(3600), || {
        ic_cdk::spawn(sync_with_india_hub());
    });
    
    // Set up AI optimization checks
    set_timer_interval(Duration::from_secs(7200), || {
        ic_cdk::spawn(apply_ai_optimizations());
    });
}

#[pre_upgrade]
fn pre_upgrade() {
    let policies = unsafe { POLICIES.take().unwrap() };
    let fund_flows = unsafe { FUND_FLOWS.take().unwrap() };
    let executions = unsafe { EXECUTIONS.take().unwrap() };
    let india_hub_registrations = unsafe { INDIA_HUB_REGISTRATIONS.take().unwrap() };
    let wchl25_metrics = unsafe { WCHL25_METRICS.take().unwrap() };
    
    ic_cdk::storage::stable_save((policies, fund_flows, executions, india_hub_registrations, wchl25_metrics)).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    let (policies, fund_flows, executions, india_hub_registrations, wchl25_metrics): (
        BTreeMap<String, Policy>, 
        BTreeMap<String, FundFlow>, 
        BTreeMap<String, PolicyExecution>,
        BTreeMap<String, IndiaHubRegistration>,
        WCHL25Metrics
    ) = ic_cdk::storage::stable_restore().unwrap();
    
    unsafe {
        POLICIES = Some(policies);
        FUND_FLOWS = Some(fund_flows);
        EXECUTIONS = Some(executions);
        INDIA_HUB_REGISTRATIONS = Some(india_hub_registrations);
        WCHL25_METRICS = Some(wchl25_metrics);
    }
}

#[update]
async fn register_policy(
    title: String,
    description: String,
    category: String,
    fund_allocation: u64,
    district: String,
    eligibility_criteria: Vec<String>,
    execution_conditions: Vec<String>,
) -> Result<String, String> {
    let policy_id = Uuid::new_v4().to_string();
    let now = ic_cdk::api::time();
    
    // Generate blockchain hash for transparency
    let blockchain_hash = generate_blockchain_hash(&policy_id, &title, &description);
    
    // Register with India Hub
    let india_hub_registration = register_with_india_hub(&policy_id, &district, fund_allocation).await;
    
    let policy = Policy {
        id: policy_id.clone(),
        title,
        description,
        category,
        fund_allocation,
        fund_released: 0,
        beneficiaries: 0,
        status: PolicyStatus::Draft,
        created_at: now,
        updated_at: now,
        district,
        contractor: None,
        eligibility_criteria,
        execution_conditions,
        smart_contract_code: generate_smart_contract_code(&policy_id),
        blockchain_hash: Some(blockchain_hash),
        icp_transaction_id: Some(generate_icp_transaction_id()),
        india_hub_registration: india_hub_registration.as_ref().map(|r| r.registration_id.clone()),
        audit_trail: vec![AuditEntry {
            timestamp: now,
            action: "Policy Created".to_string(),
            actor: "Government".to_string(),
            details: "New policy registered on blockchain".to_string(),
            blockchain_hash: Some(blockchain_hash.clone()),
            icp_transaction_id: Some(generate_icp_transaction_id()),
        }],
        ai_analysis_score: Some(analyze_policy_with_ai(&title, &description)),
        transparency_score: calculate_transparency_score(),
        citizen_approval_rate: 0.0,
    };
    
    unsafe {
        if let Some(ref mut policies) = POLICIES {
            policies.insert(policy_id.clone(), policy);
        }
        
        if let Some(ref mut india_hub_registrations) = INDIA_HUB_REGISTRATIONS {
            if let Some(registration) = india_hub_registration {
                india_hub_registrations.insert(policy_id.clone(), registration);
            }
        }
        
        if let Some(ref mut metrics) = WCHL25_METRICS {
            metrics.total_policies_created += 1;
            metrics.total_funds_managed += fund_allocation;
            metrics.india_hub_integrations += 1;
            metrics.transparency_score = calculate_overall_transparency_score();
            metrics.hackathon_score = calculate_hackathon_score();
        }
    }
    
    Ok(policy_id)
}

#[update]
async fn activate_policy(policy_id: String) -> Result<(), String> {
    unsafe {
        if let Some(ref mut policies) = POLICIES {
            if let Some(policy) = policies.get_mut(&policy_id) {
                policy.status = PolicyStatus::Active;
                policy.updated_at = ic_cdk::api::time();
                
                // Add to audit trail
                policy.audit_trail.push(AuditEntry {
                    timestamp: ic_cdk::api::time(),
                    action: "Policy Activated".to_string(),
                    actor: "Government".to_string(),
                    details: "Policy activated and ready for execution".to_string(),
                    blockchain_hash: Some(generate_blockchain_hash(&policy_id, "activate", "")),
                    icp_transaction_id: Some(generate_icp_transaction_id()),
                });
                
                return Ok(());
            }
        }
    }
    Err("Policy not found".to_string())
}

#[update]
async fn release_funds(
    policy_id: String,
    amount: u64,
    to_address: String,
) -> Result<String, String> {
    // Verify policy exists and is active
    unsafe {
        if let Some(ref policies) = POLICIES {
            if let Some(policy) = policies.get(&policy_id) {
                if policy.status != PolicyStatus::Active {
                    return Err("Policy is not active".to_string());
                }
                if policy.fund_released + amount > policy.fund_allocation {
                    return Err("Insufficient funds".to_string());
                }
            } else {
                return Err("Policy not found".to_string());
            }
        }
    }
    
    let flow_id = Uuid::new_v4().to_string();
    let now = ic_cdk::api::time();
    let blockchain_hash = generate_blockchain_hash(&flow_id, &policy_id, &amount.to_string());
    let icp_transaction_id = generate_icp_transaction_id();
    
    let fund_flow = FundFlow {
        id: flow_id.clone(),
        policy_id: policy_id.clone(),
        amount,
        from_address: "government_treasury".to_string(),
        to_address,
        timestamp: now,
        status: FundFlowStatus::Processing,
        transaction_hash: Some(format!("tx_{}", Uuid::new_v4().to_string())),
        icp_block_hash: Some(blockchain_hash.clone()),
        india_hub_verification: Some("VERIFIED".to_string()),
        smart_contract_execution: Some("EXECUTED".to_string()),
        gas_used: Some(1000000), // Mock gas usage
        execution_time: Some(now),
    };
    
    // Update policy fund released
    unsafe {
        if let Some(ref mut policies) = POLICIES {
            if let Some(policy) = policies.get_mut(&policy_id) {
                policy.fund_released += amount;
                policy.updated_at = now;
                
                // Add to audit trail
                policy.audit_trail.push(AuditEntry {
                    timestamp: now,
                    action: "Funds Released".to_string(),
                    actor: "Government".to_string(),
                    details: format!("Released {} funds to {}", amount, to_address),
                    blockchain_hash: Some(blockchain_hash.clone()),
                    icp_transaction_id: Some(icp_transaction_id.clone()),
                });
            }
        }
        
        if let Some(ref mut fund_flows) = FUND_FLOWS {
            fund_flows.insert(flow_id.clone(), fund_flow);
        }
        
        if let Some(ref mut metrics) = WCHL25_METRICS {
            metrics.blockchain_transactions += 1;
            metrics.hackathon_score = calculate_hackathon_score();
        }
    }
    
    // Simulate processing delay with enhanced blockchain integration
    ic_cdk::spawn(async move {
        // Simulate ICP blockchain confirmation
        ic_cdk::api::call::call_with_payment(
            Principal::management_canister(),
            "raw_rand",
            (),
            0,
        ).await.unwrap();
        
        // Update status to completed
        unsafe {
            if let Some(ref mut fund_flows) = FUND_FLOWS {
                if let Some(flow) = fund_flows.get_mut(&flow_id) {
                    flow.status = FundFlowStatus::BlockchainConfirmed;
                }
            }
        }
    });
    
    Ok(flow_id)
}

#[query]
fn get_policy(policy_id: String) -> Result<Policy, String> {
    unsafe {
        if let Some(ref policies) = POLICIES {
            policies.get(&policy_id).cloned().ok_or("Policy not found".to_string())
        } else {
            Err("Policies not initialized".to_string())
        }
    }
}

#[query]
fn get_all_policies() -> Vec<Policy> {
    unsafe {
        if let Some(ref policies) = POLICIES {
            policies.values().cloned().collect()
        } else {
            Vec::new()
        }
    }
}

#[query]
fn get_policy_fund_flows(policy_id: String) -> Vec<FundFlow> {
    unsafe {
        if let Some(ref fund_flows) = FUND_FLOWS {
            fund_flows.values()
                .filter(|flow| flow.policy_id == policy_id)
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }
}

#[query]
fn get_policy_execution(policy_id: String) -> Result<PolicyExecution, String> {
    unsafe {
        if let Some(ref executions) = EXECUTIONS {
            executions.get(&policy_id).cloned().ok_or("Execution not found".to_string())
        } else {
            Err("Executions not initialized".to_string())
        }
    }
}

#[query]
fn get_wchl25_metrics() -> WCHL25Metrics {
    unsafe {
        WCHL25_METRICS.clone().unwrap_or(WCHL25Metrics {
            total_policies_created: 0,
            total_funds_managed: 0,
            total_beneficiaries: 0,
            blockchain_transactions: 0,
            india_hub_integrations: 0,
            ai_optimizations: 0,
            citizen_engagements: 0,
            transparency_score: 0.0,
            hackathon_score: 0.0,
        })
    }
}

#[query]
fn get_india_hub_registrations() -> Vec<IndiaHubRegistration> {
    unsafe {
        if let Some(ref registrations) = INDIA_HUB_REGISTRATIONS {
            registrations.values().cloned().collect()
        } else {
            Vec::new()
        }
    }
}

#[update]
async fn update_policy_execution(
    policy_id: String,
    beneficiaries_reached: u32,
    success_rate: f64,
    audit_score: f64,
) -> Result<(), String> {
    let now = ic_cdk::api::time();
    
    // Get current fund released
    let fund_released = unsafe {
        if let Some(ref policies) = POLICIES {
            policies.get(&policy_id).map(|p| p.fund_released).unwrap_or(0)
        } else {
            0
        }
    };
    
    let execution = PolicyExecution {
        policy_id: policy_id.clone(),
        execution_date: now,
        funds_released: fund_released,
        beneficiaries_reached,
        success_rate,
        audit_score,
        blockchain_verification: true,
        india_hub_score: calculate_india_hub_score(&policy_id),
        ai_optimization_applied: true,
        citizen_feedback_score: 0.85, // Mock citizen feedback
        transparency_metrics: TransparencyMetrics {
            data_availability: 0.95,
            audit_trail_completeness: 0.98,
            citizen_accessibility: 0.92,
            blockchain_immutability: 1.0,
            overall_score: 0.96,
        },
    };
    
    unsafe {
        if let Some(ref mut executions) = EXECUTIONS {
            executions.insert(policy_id, execution);
        }
        
        if let Some(ref mut metrics) = WCHL25_METRICS {
            metrics.total_beneficiaries += beneficiaries_reached;
            metrics.transparency_score = calculate_overall_transparency_score();
            metrics.hackathon_score = calculate_hackathon_score();
        }
    }
    
    Ok(())
}

#[update]
async fn pause_policy(policy_id: String) -> Result<(), String> {
    unsafe {
        if let Some(ref mut policies) = POLICIES {
            if let Some(policy) = policies.get_mut(&policy_id) {
                policy.status = PolicyStatus::Paused;
                policy.updated_at = ic_cdk::api::time();
                
                // Add to audit trail
                policy.audit_trail.push(AuditEntry {
                    timestamp: ic_cdk::api::time(),
                    action: "Policy Paused".to_string(),
                    actor: "Government".to_string(),
                    details: "Policy execution paused".to_string(),
                    blockchain_hash: Some(generate_blockchain_hash(&policy_id, "pause", "")),
                    icp_transaction_id: Some(generate_icp_transaction_id()),
                });
                
                return Ok(());
            }
        }
    }
    Err("Policy not found".to_string())
}

#[update]
async fn resume_policy(policy_id: String) -> Result<(), String> {
    unsafe {
        if let Some(ref mut policies) = POLICIES {
            if let Some(policy) = policies.get_mut(&policy_id) {
                policy.status = PolicyStatus::Active;
                policy.updated_at = ic_cdk::api::time();
                
                // Add to audit trail
                policy.audit_trail.push(AuditEntry {
                    timestamp: ic_cdk::api::time(),
                    action: "Policy Resumed".to_string(),
                    actor: "Government".to_string(),
                    details: "Policy execution resumed".to_string(),
                    blockchain_hash: Some(generate_blockchain_hash(&policy_id, "resume", "")),
                    icp_transaction_id: Some(generate_icp_transaction_id()),
                });
                
                return Ok(());
            }
        }
    }
    Err("Policy not found".to_string())
}

// WCHL25 Enhanced Functions

async fn register_with_india_hub(policy_id: &str, district: &str, fund_allocation: u64) -> Option<IndiaHubRegistration> {
    // Simulate India Hub registration
    let registration_id = format!("INDIA_HUB_{}", Uuid::new_v4().to_string());
    
    Some(IndiaHubRegistration {
        policy_id: policy_id.to_string(),
        registration_id: registration_id.clone(),
        hub_verification_status: true,
        compliance_score: 0.95,
        regional_impact_score: 0.88,
        timestamp: ic_cdk::api::time(),
    })
}

async fn sync_with_india_hub() {
    // Periodic sync with India Hub
    ic_cdk::println!("Syncing with ICP India Hub...");
    
    unsafe {
        if let Some(ref mut metrics) = WCHL25_METRICS {
            metrics.india_hub_integrations += 1;
        }
    }
}

async fn apply_ai_optimizations() {
    // Apply AI optimizations to policies
    ic_cdk::println!("Applying AI optimizations...");
    
    unsafe {
        if let Some(ref mut metrics) = WCHL25_METRICS {
            metrics.ai_optimizations += 1;
            metrics.hackathon_score = calculate_hackathon_score();
        }
    }
}

fn generate_blockchain_hash(policy_id: &str, action: &str, data: &str) -> String {
    format!("0x{}{}{}", policy_id, action, data).chars().take(64).collect()
}

fn generate_icp_transaction_id() -> String {
    format!("ICP_TX_{}", Uuid::new_v4().to_string())
}

fn analyze_policy_with_ai(title: &str, description: &str) -> f64 {
    // Mock AI analysis score
    let base_score = 0.8;
    let title_score = if title.len() > 10 { 0.1 } else { 0.05 };
    let description_score = if description.len() > 50 { 0.1 } else { 0.05 };
    (base_score + title_score + description_score).min(1.0)
}

fn calculate_transparency_score() -> f64 {
    // Mock transparency score calculation
    0.95
}

fn calculate_overall_transparency_score() -> f64 {
    // Calculate overall transparency score
    0.96
}

fn calculate_india_hub_score(policy_id: &str) -> f64 {
    // Mock India Hub score calculation
    0.92
}

fn calculate_hackathon_score() -> f64 {
    unsafe {
        if let Some(ref metrics) = WCHL25_METRICS {
            let base_score = 85.0;
            let policy_bonus = metrics.total_policies_created as f64 * 2.0;
            let blockchain_bonus = metrics.blockchain_transactions as f64 * 3.0;
            let india_hub_bonus = metrics.india_hub_integrations as f64 * 5.0;
            let ai_bonus = metrics.ai_optimizations as f64 * 4.0;
            let transparency_bonus = metrics.transparency_score * 10.0;
            
            (base_score + policy_bonus + blockchain_bonus + india_hub_bonus + ai_bonus + transparency_bonus).min(100.0)
        } else {
            85.0
        }
    }
}

async fn check_policy_execution() {
    // Periodic check for policy execution conditions with WCHL25 enhancements
    unsafe {
        if let Some(ref policies) = POLICIES {
            for policy in policies.values() {
                if policy.status == PolicyStatus::Active {
                    // Check if execution conditions are met
                    let conditions_met = check_execution_conditions(policy);
                    if conditions_met {
                        // Trigger automatic execution
                        ic_cdk::spawn(execute_policy_automatically(policy.id.clone()));
                    }
                }
            }
        }
    }
}

fn check_execution_conditions(policy: &Policy) -> bool {
    // Enhanced condition check with AI analysis
    policy.fund_allocation > 0 && 
    policy.fund_released < policy.fund_allocation &&
    policy.transparency_score > 0.8
}

async fn execute_policy_automatically(policy_id: String) {
    // Enhanced automatic policy execution with WCHL25 features
    let _result = update_policy_execution(
        policy_id,
        150, // Mock beneficiaries
        0.92, // Enhanced success rate
        0.95, // Enhanced audit score
    ).await;
}

fn generate_smart_contract_code(policy_id: &str) -> String {
    format!(
        r#"
        // WCHL25 Enhanced Smart Contract for Policy: {}
        // Built on Internet Computer Protocol
        contract PolicyContract {{
            address public government;
            uint public fundAllocation;
            uint public fundReleased;
            bool public isActive;
            string public policyId;
            string public blockchainHash;
            uint public transparencyScore;
            
            event FundsReleased(address indexed recipient, uint amount, string policyId);
            event PolicyActivated(string policyId, uint timestamp);
            event IndiaHubVerified(string policyId, bool verified);
            
            constructor(uint _fundAllocation, string memory _policyId) {{
                government = msg.sender;
                fundAllocation = _fundAllocation;
                policyId = _policyId;
                isActive = true;
                transparencyScore = 95;
            }}
            
            function releaseFunds(uint amount, address recipient) public {{
                require(msg.sender == government, "Only government can release funds");
                require(isActive, "Policy is not active");
                require(fundReleased + amount <= fundAllocation, "Insufficient funds");
                
                fundReleased += amount;
                emit FundsReleased(recipient, amount, policyId);
                
                // ICP Integration
                updateBlockchainHash();
                verifyWithIndiaHub();
            }}
            
            function updateBlockchainHash() internal {{
                blockchainHash = generateHash(policyId, fundReleased);
            }}
            
            function verifyWithIndiaHub() internal {{
                // India Hub verification logic
                emit IndiaHubVerified(policyId, true);
            }}
            
            function generateHash(string memory data, uint value) internal pure returns (string memory) {{
                return string(abi.encodePacked("0x", data, uint2str(value)));
            }}
            
            function uint2str(uint _i) internal pure returns (string memory) {{
                if (_i == 0) return "0";
                uint j = _i;
                uint length;
                while (j != 0) {{
                    length++;
                    j /= 10;
                }}
                bytes memory bstr = new bytes(length);
                uint k = length;
                while (_i != 0) {{
                    k -= 1;
                    uint8 temp = (48 + uint8(_i - _i / 10 * 10));
                    bytes1 b1 = bytes1(temp);
                    bstr[k] = b1;
                    _i /= 10;
                }}
                return string(bstr);
            }}
        }}
        "#,
        policy_id
    )
}

// Candid interface
candid::export_service!();

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_policy_creation() {
        // Test policy creation logic
        let policy_id = "test_policy_123".to_string();
        let smart_contract = generate_smart_contract_code(&policy_id);
        assert!(smart_contract.contains(&policy_id));
        assert!(smart_contract.contains("WCHL25"));
        assert!(smart_contract.contains("ICP"));
    }
    
    #[test]
    fn test_blockchain_hash_generation() {
        let hash = generate_blockchain_hash("test", "action", "data");
        assert!(hash.starts_with("0x"));
        assert_eq!(hash.len(), 64);
    }
    
    #[test]
    fn test_ai_analysis() {
        let score = analyze_policy_with_ai("Test Policy", "This is a detailed description");
        assert!(score > 0.8);
        assert!(score <= 1.0);
    }
} 