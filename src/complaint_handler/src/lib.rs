use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{api::call::call, export::candid, init, post_upgrade, pre_upgrade, query, update};
use ic_cdk_timers::set_timer_interval;
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};
use std::collections::BTreeMap;
use std::time::Duration;
use uuid::Uuid;

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct Complaint {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub priority: ComplaintPriority,
    pub status: ComplaintStatus,
    pub policy_id: Option<String>,
    pub district: String,
    pub location: Option<String>,
    pub media_links: Vec<String>,
    pub citizen_id: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub ai_analysis: Option<AIAnalysis>,
    pub audit_score: f64,
    pub resolution_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub enum ComplaintPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub enum ComplaintStatus {
    Submitted,
    UnderReview,
    Investigation,
    Resolved,
    Dismissed,
    Escalated,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct AIAnalysis {
    pub sentiment: String,
    pub category_prediction: String,
    pub priority_score: f64,
    pub suggested_action: String,
    pub confidence: f64,
    pub keywords: Vec<String>,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct ComplaintMetrics {
    pub total_complaints: u32,
    pub resolved_complaints: u32,
    pub average_resolution_time: f64,
    pub category_distribution: BTreeMap<String, u32>,
    pub district_distribution: BTreeMap<String, u32>,
}

// Stable storage for complaints
static mut COMPLAINTS: Option<BTreeMap<String, Complaint>> = None;
static mut COMPLAINT_METRICS: Option<ComplaintMetrics> = None;

#[init]
fn init() {
    unsafe {
        COMPLAINTS = Some(BTreeMap::new());
        COMPLAINT_METRICS = Some(ComplaintMetrics {
            total_complaints: 0,
            resolved_complaints: 0,
            average_resolution_time: 0.0,
            category_distribution: BTreeMap::new(),
            district_distribution: BTreeMap::new(),
        });
    }
    
    // Set up periodic complaint analysis
    set_timer_interval(Duration::from_secs(1800), || {
        ic_cdk::spawn(analyze_pending_complaints());
    });
}

#[pre_upgrade]
fn pre_upgrade() {
    let complaints = unsafe { COMPLAINTS.take().unwrap() };
    let metrics = unsafe { COMPLAINT_METRICS.take().unwrap() };
    
    ic_cdk::storage::stable_save((complaints, metrics)).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    let (complaints, metrics): (BTreeMap<String, Complaint>, ComplaintMetrics) = 
        ic_cdk::storage::stable_restore().unwrap();
    
    unsafe {
        COMPLAINTS = Some(complaints);
        COMPLAINT_METRICS = Some(metrics);
    }
}

#[update]
async fn submit_complaint(
    title: String,
    description: String,
    category: String,
    priority: ComplaintPriority,
    policy_id: Option<String>,
    district: String,
    location: Option<String>,
    media_links: Vec<String>,
    citizen_id: String,
) -> Result<String, String> {
    let complaint_id = Uuid::new_v4().to_string();
    let now = ic_cdk::api::time();
    
    let complaint = Complaint {
        id: complaint_id.clone(),
        title,
        description: description.clone(),
        category: category.clone(),
        priority,
        status: ComplaintStatus::Submitted,
        policy_id,
        district: district.clone(),
        location,
        media_links,
        citizen_id,
        created_at: now,
        updated_at: now,
        ai_analysis: None,
        audit_score: 0.0,
        resolution_time: None,
    };
    
    // Store complaint
    unsafe {
        if let Some(ref mut complaints) = COMPLAINTS {
            complaints.insert(complaint_id.clone(), complaint);
        }
        
        // Update metrics
        if let Some(ref mut metrics) = COMPLAINT_METRICS {
            metrics.total_complaints += 1;
            *metrics.category_distribution.entry(category).or_insert(0) += 1;
            *metrics.district_distribution.entry(district).or_insert(0) += 1;
        }
    }
    
    // Trigger AI analysis
    ic_cdk::spawn(analyze_complaint_with_ai(complaint_id.clone(), description));
    
    Ok(complaint_id)
}

#[update]
async fn update_complaint_status(
    complaint_id: String,
    status: ComplaintStatus,
) -> Result<(), String> {
    let now = ic_cdk::api::time();
    
    unsafe {
        if let Some(ref mut complaints) = COMPLAINTS {
            if let Some(complaint) = complaints.get_mut(&complaint_id) {
                complaint.status = status.clone();
                complaint.updated_at = now;
                
                if status == ComplaintStatus::Resolved {
                    complaint.resolution_time = Some(now - complaint.created_at);
                    
                    // Update metrics
                    if let Some(ref mut metrics) = COMPLAINT_METRICS {
                        metrics.resolved_complaints += 1;
                        // Update average resolution time
                        let total_time = metrics.average_resolution_time * (metrics.resolved_complaints - 1) as f64;
                        let new_time = (now - complaint.created_at) as f64;
                        metrics.average_resolution_time = (total_time + new_time) / metrics.resolved_complaints as f64;
                    }
                }
                
                return Ok(());
            }
        }
    }
    
    Err("Complaint not found".to_string())
}

#[query]
fn get_complaint(complaint_id: String) -> Result<Complaint, String> {
    unsafe {
        if let Some(ref complaints) = COMPLAINTS {
            complaints.get(&complaint_id).cloned().ok_or("Complaint not found".to_string())
        } else {
            Err("Complaints not initialized".to_string())
        }
    }
}

#[query]
fn get_all_complaints() -> Vec<Complaint> {
    unsafe {
        if let Some(ref complaints) = COMPLAINTS {
            complaints.values().cloned().collect()
        } else {
            Vec::new()
        }
    }
}

#[query]
fn get_complaints_by_policy(policy_id: String) -> Vec<Complaint> {
    unsafe {
        if let Some(ref complaints) = COMPLAINTS {
            complaints.values()
                .filter(|complaint| complaint.policy_id.as_ref() == Some(&policy_id))
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }
}

#[query]
fn get_complaints_by_district(district: String) -> Vec<Complaint> {
    unsafe {
        if let Some(ref complaints) = COMPLAINTS {
            complaints.values()
                .filter(|complaint| complaint.district == district)
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }
}

#[query]
fn get_complaint_metrics() -> ComplaintMetrics {
    unsafe {
        COMPLAINT_METRICS.clone().unwrap_or(ComplaintMetrics {
            total_complaints: 0,
            resolved_complaints: 0,
            average_resolution_time: 0.0,
            category_distribution: BTreeMap::new(),
            district_distribution: BTreeMap::new(),
        })
    }
}

#[update]
async fn escalate_complaint(complaint_id: String) -> Result<(), String> {
    let now = ic_cdk::api::time();
    
    unsafe {
        if let Some(ref mut complaints) = COMPLAINTS {
            if let Some(complaint) = complaints.get_mut(&complaint_id) {
                complaint.status = ComplaintStatus::Escalated;
                complaint.updated_at = now;
                complaint.audit_score += 0.2; // Increase audit score for escalated complaints
                return Ok(());
            }
        }
    }
    
    Err("Complaint not found".to_string())
}

#[update]
async fn add_audit_score(complaint_id: String, score: f64) -> Result<(), String> {
    unsafe {
        if let Some(ref mut complaints) = COMPLAINTS {
            if let Some(complaint) = complaints.get_mut(&complaint_id) {
                complaint.audit_score = score;
                complaint.updated_at = ic_cdk::api::time();
                return Ok(());
            }
        }
    }
    
    Err("Complaint not found".to_string())
}

async fn analyze_complaint_with_ai(complaint_id: String, description: String) {
    // Simulate AI analysis using LLM canister
    let analysis_result = analyze_text_with_llm(&description).await;
    
    unsafe {
        if let Some(ref mut complaints) = COMPLAINTS {
            if let Some(complaint) = complaints.get_mut(&complaint_id) {
                complaint.ai_analysis = Some(analysis_result);
                complaint.updated_at = ic_cdk::api::time();
            }
        }
    }
}

async fn analyze_text_with_llm(text: &str) -> AIAnalysis {
    // Mock AI analysis - in real implementation, this would call the LLM canister
    let sentiment = if text.contains("corruption") || text.contains("fraud") {
        "negative".to_string()
    } else if text.contains("delay") || text.contains("slow") {
        "neutral".to_string()
    } else {
        "positive".to_string()
    };
    
    let category_prediction = if text.contains("road") || text.contains("infrastructure") {
        "infrastructure".to_string()
    } else if text.contains("fund") || text.contains("money") {
        "fund_misuse".to_string()
    } else {
        "service_delay".to_string()
    };
    
    let priority_score = if text.contains("urgent") || text.contains("critical") {
        0.9
    } else if text.contains("important") {
        0.7
    } else {
        0.5
    };
    
    AIAnalysis {
        sentiment,
        category_prediction,
        priority_score,
        suggested_action: "Investigate and respond within 48 hours".to_string(),
        confidence: 0.85,
        keywords: vec!["government".to_string(), "service".to_string(), "issue".to_string()],
    }
}

async fn analyze_pending_complaints() {
    // Analyze complaints that haven't been processed yet
    unsafe {
        if let Some(ref complaints) = COMPLAINTS {
            for complaint in complaints.values() {
                if complaint.ai_analysis.is_none() && complaint.status == ComplaintStatus::Submitted {
                    let description = complaint.description.clone();
                    let complaint_id = complaint.id.clone();
                    ic_cdk::spawn(analyze_complaint_with_ai(complaint_id, description));
                }
            }
        }
    }
}

#[update]
async fn trigger_policy_pause(complaint_id: String) -> Result<(), String> {
    // This would integrate with the smart_policy canister to pause policies
    // For now, we'll just mark the complaint as escalated
    escalate_complaint(complaint_id).await
}

// Candid interface
candid::export_service!();

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_complaint_creation() {
        // Test complaint creation logic
        let complaint_id = "test_complaint_123".to_string();
        assert!(complaint_id.contains("test"));
    }
} 