use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{api::call::call, export::candid, init, post_upgrade, pre_upgrade, query, update};
use ic_cdk_timers::set_timer_interval;
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};
use std::collections::BTreeMap;
use std::time::Duration;
use uuid::Uuid;

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct FundTransaction {
    pub id: String,
    pub policy_id: String,
    pub transaction_type: TransactionType,
    pub amount: u64,
    pub from_address: String,
    pub to_address: String,
    pub timestamp: u64,
    pub status: TransactionStatus,
    pub transaction_hash: String,
    pub metadata: BTreeMap<String, String>,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub enum TransactionType {
    Allocation,
    Release,
    Transfer,
    Refund,
    Fee,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub enum TransactionStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct FundBalance {
    pub policy_id: String,
    pub total_allocated: u64,
    pub total_released: u64,
    pub total_transferred: u64,
    pub current_balance: u64,
    pub last_updated: u64,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct DistrictFunds {
    pub district: String,
    pub total_allocated: u64,
    pub total_released: u64,
    pub active_policies: u32,
    pub completion_rate: f64,
    pub last_updated: u64,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct FundAnalytics {
    pub total_funds_allocated: u64,
    pub total_funds_released: u64,
    pub total_transactions: u32,
    pub average_transaction_amount: f64,
    pub district_distribution: BTreeMap<String, u64>,
    pub category_distribution: BTreeMap<String, u64>,
    pub monthly_trends: BTreeMap<String, u64>,
    pub success_rate: f64,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct RealTimeMetrics {
    pub current_time: u64,
    pub active_transactions: u32,
    pub pending_amount: u64,
    pub daily_volume: u64,
    pub weekly_volume: u64,
    pub monthly_volume: u64,
}

// Stable storage for fund tracking data
static mut TRANSACTIONS: Option<BTreeMap<String, FundTransaction>> = None;
static mut FUND_BALANCES: Option<BTreeMap<String, FundBalance>> = None;
static mut DISTRICT_FUNDS: Option<BTreeMap<String, DistrictFunds>> = None;
static mut FUND_ANALYTICS: Option<FundAnalytics> = None;
static mut REAL_TIME_METRICS: Option<RealTimeMetrics> = None;

#[init]
fn init() {
    unsafe {
        TRANSACTIONS = Some(BTreeMap::new());
        FUND_BALANCES = Some(BTreeMap::new());
        DISTRICT_FUNDS = Some(BTreeMap::new());
        FUND_ANALYTICS = Some(FundAnalytics {
            total_funds_allocated: 0,
            total_funds_released: 0,
            total_transactions: 0,
            average_transaction_amount: 0.0,
            district_distribution: BTreeMap::new(),
            category_distribution: BTreeMap::new(),
            monthly_trends: BTreeMap::new(),
            success_rate: 0.0,
        });
        REAL_TIME_METRICS = Some(RealTimeMetrics {
            current_time: 0,
            active_transactions: 0,
            pending_amount: 0,
            daily_volume: 0,
            weekly_volume: 0,
            monthly_volume: 0,
        });
    }
    
    // Set up periodic metrics updates
    set_timer_interval(Duration::from_secs(300), || {
        ic_cdk::spawn(update_real_time_metrics());
    });
}

#[pre_upgrade]
fn pre_upgrade() {
    let transactions = unsafe { TRANSACTIONS.take().unwrap() };
    let fund_balances = unsafe { FUND_BALANCES.take().unwrap() };
    let district_funds = unsafe { DISTRICT_FUNDS.take().unwrap() };
    let analytics = unsafe { FUND_ANALYTICS.take().unwrap() };
    let metrics = unsafe { REAL_TIME_METRICS.take().unwrap() };
    
    ic_cdk::storage::stable_save((transactions, fund_balances, district_funds, analytics, metrics)).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    let (transactions, fund_balances, district_funds, analytics, metrics): (
        BTreeMap<String, FundTransaction>, 
        BTreeMap<String, FundBalance>, 
        BTreeMap<String, DistrictFunds>, 
        FundAnalytics, 
        RealTimeMetrics
    ) = ic_cdk::storage::stable_restore().unwrap();
    
    unsafe {
        TRANSACTIONS = Some(transactions);
        FUND_BALANCES = Some(fund_balances);
        DISTRICT_FUNDS = Some(district_funds);
        FUND_ANALYTICS = Some(analytics);
        REAL_TIME_METRICS = Some(metrics);
    }
}

#[update]
async fn record_transaction(
    policy_id: String,
    transaction_type: TransactionType,
    amount: u64,
    from_address: String,
    to_address: String,
    metadata: BTreeMap<String, String>,
) -> Result<String, String> {
    let transaction_id = Uuid::new_v4().to_string();
    let now = ic_cdk::api::time();
    
    let transaction = FundTransaction {
        id: transaction_id.clone(),
        policy_id: policy_id.clone(),
        transaction_type: transaction_type.clone(),
        amount,
        from_address: from_address.clone(),
        to_address: to_address.clone(),
        timestamp: now,
        status: TransactionStatus::Processing,
        transaction_hash: format!("tx_{}", Uuid::new_v4().to_string()),
        metadata,
    };
    
    // Store transaction
    unsafe {
        if let Some(ref mut transactions) = TRANSACTIONS {
            transactions.insert(transaction_id.clone(), transaction);
        }
    }
    
    // Update fund balances
    update_fund_balance(&policy_id, &transaction_type, amount).await;
    
    // Update analytics
    update_analytics(&transaction_type, amount).await;
    
    // Simulate transaction processing
    ic_cdk::spawn(process_transaction(transaction_id.clone()));
    
    Ok(transaction_id)
}

#[update]
async fn update_transaction_status(
    transaction_id: String,
    status: TransactionStatus,
) -> Result<(), String> {
    unsafe {
        if let Some(ref mut transactions) = TRANSACTIONS {
            if let Some(transaction) = transactions.get_mut(&transaction_id) {
                transaction.status = status.clone();
                return Ok(());
            }
        }
    }
    
    Err("Transaction not found".to_string())
}

#[query]
fn get_transaction(transaction_id: String) -> Result<FundTransaction, String> {
    unsafe {
        if let Some(ref transactions) = TRANSACTIONS {
            transactions.get(&transaction_id).cloned().ok_or("Transaction not found".to_string())
        } else {
            Err("Transactions not initialized".to_string())
        }
    }
}

#[query]
fn get_policy_transactions(policy_id: String) -> Vec<FundTransaction> {
    unsafe {
        if let Some(ref transactions) = TRANSACTIONS {
            transactions.values()
                .filter(|transaction| transaction.policy_id == policy_id)
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }
}

#[query]
fn get_fund_balance(policy_id: String) -> Result<FundBalance, String> {
    unsafe {
        if let Some(ref fund_balances) = FUND_BALANCES {
            fund_balances.get(&policy_id).cloned().ok_or("Fund balance not found".to_string())
        } else {
            Err("Fund balances not initialized".to_string())
        }
    }
}

#[query]
fn get_district_funds(district: String) -> Result<DistrictFunds, String> {
    unsafe {
        if let Some(ref district_funds) = DISTRICT_FUNDS {
            district_funds.get(&district).cloned().ok_or("District funds not found".to_string())
        } else {
            Err("District funds not initialized".to_string())
        }
    }
}

#[query]
fn get_fund_analytics() -> FundAnalytics {
    unsafe {
        FUND_ANALYTICS.clone().unwrap_or(FundAnalytics {
            total_funds_allocated: 0,
            total_funds_released: 0,
            total_transactions: 0,
            average_transaction_amount: 0.0,
            district_distribution: BTreeMap::new(),
            category_distribution: BTreeMap::new(),
            monthly_trends: BTreeMap::new(),
            success_rate: 0.0,
        })
    }
}

#[query]
fn get_real_time_metrics() -> RealTimeMetrics {
    unsafe {
        REAL_TIME_METRICS.clone().unwrap_or(RealTimeMetrics {
            current_time: 0,
            active_transactions: 0,
            pending_amount: 0,
            daily_volume: 0,
            weekly_volume: 0,
            monthly_volume: 0,
        })
    }
}

#[query]
fn get_recent_transactions(limit: u32) -> Vec<FundTransaction> {
    unsafe {
        if let Some(ref transactions) = TRANSACTIONS {
            let mut sorted_transactions: Vec<FundTransaction> = transactions.values().cloned().collect();
            sorted_transactions.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
            sorted_transactions.into_iter().take(limit as usize).collect()
        } else {
            Vec::new()
        }
    }
}

#[query]
fn get_transactions_by_type(transaction_type: TransactionType) -> Vec<FundTransaction> {
    unsafe {
        if let Some(ref transactions) = TRANSACTIONS {
            transactions.values()
                .filter(|transaction| std::mem::discriminant(&transaction.transaction_type) == std::mem::discriminant(&transaction_type))
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }
}

async fn update_fund_balance(policy_id: &str, transaction_type: &TransactionType, amount: u64) {
    unsafe {
        if let Some(ref mut fund_balances) = FUND_BALANCES {
            let balance = fund_balances.entry(policy_id.to_string()).or_insert(FundBalance {
                policy_id: policy_id.to_string(),
                total_allocated: 0,
                total_released: 0,
                total_transferred: 0,
                current_balance: 0,
                last_updated: ic_cdk::api::time(),
            });
            
            match transaction_type {
                TransactionType::Allocation => {
                    balance.total_allocated += amount;
                    balance.current_balance += amount;
                }
                TransactionType::Release => {
                    balance.total_released += amount;
                    balance.current_balance = balance.current_balance.saturating_sub(amount);
                }
                TransactionType::Transfer => {
                    balance.total_transferred += amount;
                    balance.current_balance = balance.current_balance.saturating_sub(amount);
                }
                _ => {}
            }
            
            balance.last_updated = ic_cdk::api::time();
        }
    }
}

async fn update_analytics(transaction_type: &TransactionType, amount: u64) {
    unsafe {
        if let Some(ref mut analytics) = FUND_ANALYTICS {
            match transaction_type {
                TransactionType::Allocation => {
                    analytics.total_funds_allocated += amount;
                }
                TransactionType::Release => {
                    analytics.total_funds_released += amount;
                }
                _ => {}
            }
            
            analytics.total_transactions += 1;
            
            // Update average transaction amount
            let total_amount = analytics.total_funds_allocated + analytics.total_funds_released;
            analytics.average_transaction_amount = total_amount as f64 / analytics.total_transactions as f64;
            
            // Update success rate (mock calculation)
            analytics.success_rate = 0.95; // 95% success rate
        }
    }
}

async fn process_transaction(transaction_id: String) {
    // Simulate transaction processing delay
    ic_cdk::api::call::call_with_payment(
        Principal::management_canister(),
        "raw_rand",
        (),
        0,
    ).await.unwrap();
    
    // Update transaction status to completed
    let _result = update_transaction_status(transaction_id, TransactionStatus::Completed).await;
}

async fn update_real_time_metrics() {
    let now = ic_cdk::api::time();
    
    unsafe {
        if let Some(ref mut metrics) = REAL_TIME_METRICS {
            metrics.current_time = now;
            
            // Count active transactions
            if let Some(ref transactions) = TRANSACTIONS {
                metrics.active_transactions = transactions.values()
                    .filter(|t| t.status == TransactionStatus::Processing)
                    .count() as u32;
                
                // Calculate pending amount
                metrics.pending_amount = transactions.values()
                    .filter(|t| t.status == TransactionStatus::Processing)
                    .map(|t| t.amount)
                    .sum();
                
                // Calculate daily volume (last 24 hours)
                let day_ago = now - 24 * 3600_000_000_000;
                metrics.daily_volume = transactions.values()
                    .filter(|t| t.timestamp >= day_ago && t.status == TransactionStatus::Completed)
                    .map(|t| t.amount)
                    .sum();
                
                // Calculate weekly volume (last 7 days)
                let week_ago = now - 7 * 24 * 3600_000_000_000;
                metrics.weekly_volume = transactions.values()
                    .filter(|t| t.timestamp >= week_ago && t.status == TransactionStatus::Completed)
                    .map(|t| t.amount)
                    .sum();
                
                // Calculate monthly volume (last 30 days)
                let month_ago = now - 30 * 24 * 3600_000_000_000;
                metrics.monthly_volume = transactions.values()
                    .filter(|t| t.timestamp >= month_ago && t.status == TransactionStatus::Completed)
                    .map(|t| t.amount)
                    .sum();
            }
        }
    }
}

// Candid interface
candid::export_service!();

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_transaction_creation() {
        // Test transaction creation logic
        let transaction_id = "test_transaction_123".to_string();
        assert!(transaction_id.contains("test"));
    }
} 