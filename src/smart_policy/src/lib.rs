use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{api::call::call, export::candid, init, post_upgrade, pre_upgrade, query, update};
use ic_cdk_timers::set_timer_interval;
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};
use std::collections::BTreeMap;
use std::time::Duration;
use uuid::Uuid;

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
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub enum PolicyStatus {
    Draft,
    Active,
    Paused,
    UnderReview,
    Completed,
    Cancelled,
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
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub enum FundFlowStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct PolicyExecution {
    pub policy_id: String,
    pub execution_date: u64,
    pub funds_released: u64,
    pub beneficiaries_reached: u32,
    pub success_rate: f64,
    pub audit_score: f64,
}

// Stable storage for policies
static mut POLICIES: Option<BTreeMap<String, Policy>> = None;
static mut FUND_FLOWS: Option<BTreeMap<String, FundFlow>> = None;
static mut EXECUTIONS: Option<BTreeMap<String, PolicyExecution>> = None;

#[init]
fn init() {
    unsafe {
        POLICIES = Some(BTreeMap::new());
        FUND_FLOWS = Some(BTreeMap::new());
        EXECUTIONS = Some(BTreeMap::new());
    }
    
    // Set up periodic policy checks
    set_timer_interval(Duration::from_secs(3600), || {
        ic_cdk::spawn(check_policy_execution());
    });
}

#[pre_upgrade]
fn pre_upgrade() {
    let policies = unsafe { POLICIES.take().unwrap() };
    let fund_flows = unsafe { FUND_FLOWS.take().unwrap() };
    let executions = unsafe { EXECUTIONS.take().unwrap() };
    
    ic_cdk::storage::stable_save((policies, fund_flows, executions)).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    let (policies, fund_flows, executions): (BTreeMap<String, Policy>, BTreeMap<String, FundFlow>, BTreeMap<String, PolicyExecution>) = 
        ic_cdk::storage::stable_restore().unwrap();
    
    unsafe {
        POLICIES = Some(policies);
        FUND_FLOWS = Some(fund_flows);
        EXECUTIONS = Some(executions);
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
    };
    
    unsafe {
        if let Some(ref mut policies) = POLICIES {
            policies.insert(policy_id.clone(), policy);
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
    
    let fund_flow = FundFlow {
        id: flow_id.clone(),
        policy_id: policy_id.clone(),
        amount,
        from_address: "government_treasury".to_string(),
        to_address,
        timestamp: now,
        status: FundFlowStatus::Processing,
        transaction_hash: Some(format!("tx_{}", Uuid::new_v4().to_string())),
    };
    
    // Update policy fund released
    unsafe {
        if let Some(ref mut policies) = POLICIES {
            if let Some(policy) = policies.get_mut(&policy_id) {
                policy.fund_released += amount;
                policy.updated_at = now;
            }
        }
        
        if let Some(ref mut fund_flows) = FUND_FLOWS {
            fund_flows.insert(flow_id.clone(), fund_flow);
        }
    }
    
    // Simulate processing delay
    ic_cdk::spawn(async move {
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
                    flow.status = FundFlowStatus::Completed;
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
    };
    
    unsafe {
        if let Some(ref mut executions) = EXECUTIONS {
            executions.insert(policy_id, execution);
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
                return Ok(());
            }
        }
    }
    Err("Policy not found".to_string())
}

async fn check_policy_execution() {
    // Periodic check for policy execution conditions
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
    // Simple condition check - can be enhanced with AI analysis
    policy.fund_allocation > 0 && policy.fund_released < policy.fund_allocation
}

async fn execute_policy_automatically(policy_id: String) {
    // Automatic policy execution logic
    let _result = update_policy_execution(
        policy_id,
        100, // Mock beneficiaries
        0.85, // Mock success rate
        0.92, // Mock audit score
    ).await;
}

fn generate_smart_contract_code(policy_id: &str) -> String {
    format!(
        r#"
        // Smart Contract for Policy: {}
        contract PolicyContract {{
            address public government;
            uint public fundAllocation;
            uint public fundReleased;
            bool public isActive;
            
            constructor(uint _fundAllocation) {{
                government = msg.sender;
                fundAllocation = _fundAllocation;
                isActive = true;
            }}
            
            function releaseFunds(uint amount, address recipient) public {{
                require(msg.sender == government, "Only government can release funds");
                require(isActive, "Policy is not active");
                require(fundReleased + amount <= fundAllocation, "Insufficient funds");
                
                fundReleased += amount;
                // Transfer logic would go here
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
    }
} 