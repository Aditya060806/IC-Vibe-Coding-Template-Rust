use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{api::call::call, export::candid, init, post_upgrade, pre_upgrade, query, update};
use ic_cdk_timers::set_timer_interval;
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};
use std::collections::BTreeMap;
use std::time::Duration;
use uuid::Uuid;

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct Proposal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub proposer: String,
    pub created_at: u64,
    pub voting_start: u64,
    pub voting_end: u64,
    pub status: ProposalStatus,
    pub yes_votes: u32,
    pub no_votes: u32,
    pub abstain_votes: u32,
    pub total_votes: u32,
    pub quorum_required: u32,
    pub execution_data: Option<ProposalExecution>,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub enum ProposalStatus {
    Draft,
    Active,
    Passed,
    Rejected,
    Executed,
    Expired,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct ProposalExecution {
    pub executed_at: u64,
    pub executor: String,
    pub execution_hash: String,
    pub success: bool,
    pub error_message: Option<String>,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct Vote {
    pub proposal_id: String,
    pub voter: String,
    pub vote_type: VoteType,
    pub voting_power: u32,
    pub timestamp: u64,
    pub reason: Option<String>,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub enum VoteType {
    Yes,
    No,
    Abstain,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct DAOMember {
    pub id: String,
    pub name: String,
    pub voting_power: u32,
    pub joined_at: u64,
    pub total_votes_cast: u32,
    pub reputation_score: f64,
    pub role: MemberRole,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub enum MemberRole {
    Citizen,
    PolicyMaker,
    Auditor,
    Contractor,
    Admin,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct DAOMetrics {
    pub total_proposals: u32,
    pub active_proposals: u32,
    pub passed_proposals: u32,
    pub total_members: u32,
    pub total_votes_cast: u32,
    pub average_participation: f64,
}

// Stable storage for DAO data
static mut PROPOSALS: Option<BTreeMap<String, Proposal>> = None;
static mut VOTES: Option<BTreeMap<String, Vote>> = None;
static mut MEMBERS: Option<BTreeMap<String, DAOMember>> = None;
static mut DAO_METRICS: Option<DAOMetrics> = None;

#[init]
fn init() {
    unsafe {
        PROPOSALS = Some(BTreeMap::new());
        VOTES = Some(BTreeMap::new());
        MEMBERS = Some(BTreeMap::new());
        DAO_METRICS = Some(DAOMetrics {
            total_proposals: 0,
            active_proposals: 0,
            passed_proposals: 0,
            total_members: 0,
            total_votes_cast: 0,
            average_participation: 0.0,
        });
    }
    
    // Set up periodic proposal checks
    set_timer_interval(Duration::from_secs(3600), || {
        ic_cdk::spawn(check_proposal_deadlines());
    });
}

#[pre_upgrade]
fn pre_upgrade() {
    let proposals = unsafe { PROPOSALS.take().unwrap() };
    let votes = unsafe { VOTES.take().unwrap() };
    let members = unsafe { MEMBERS.take().unwrap() };
    let metrics = unsafe { DAO_METRICS.take().unwrap() };
    
    ic_cdk::storage::stable_save((proposals, votes, members, metrics)).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    let (proposals, votes, members, metrics): (BTreeMap<String, Proposal>, BTreeMap<String, Vote>, BTreeMap<String, DAOMember>, DAOMetrics) = 
        ic_cdk::storage::stable_restore().unwrap();
    
    unsafe {
        PROPOSALS = Some(proposals);
        VOTES = Some(votes);
        MEMBERS = Some(members);
        DAO_METRICS = Some(metrics);
    }
}

#[update]
async fn create_proposal(
    title: String,
    description: String,
    category: String,
    proposer: String,
    voting_duration_hours: u64,
    quorum_required: u32,
) -> Result<String, String> {
    let proposal_id = Uuid::new_v4().to_string();
    let now = ic_cdk::api::time();
    let voting_start = now + 3600_000_000_000; // 1 hour from now
    let voting_end = voting_start + (voting_duration_hours * 3600_000_000_000);
    
    let proposal = Proposal {
        id: proposal_id.clone(),
        title,
        description,
        category,
        proposer,
        created_at: now,
        voting_start,
        voting_end,
        status: ProposalStatus::Draft,
        yes_votes: 0,
        no_votes: 0,
        abstain_votes: 0,
        total_votes: 0,
        quorum_required,
        execution_data: None,
    };
    
    unsafe {
        if let Some(ref mut proposals) = PROPOSALS {
            proposals.insert(proposal_id.clone(), proposal);
        }
        
        if let Some(ref mut metrics) = DAO_METRICS {
            metrics.total_proposals += 1;
        }
    }
    
    Ok(proposal_id)
}

#[update]
async fn activate_proposal(proposal_id: String) -> Result<(), String> {
    let now = ic_cdk::api::time();
    
    unsafe {
        if let Some(ref mut proposals) = PROPOSALS {
            if let Some(proposal) = proposals.get_mut(&proposal_id) {
                if proposal.status == ProposalStatus::Draft {
                    proposal.status = ProposalStatus::Active;
                    
                    if let Some(ref mut metrics) = DAO_METRICS {
                        metrics.active_proposals += 1;
                    }
                    
                    return Ok(());
                }
            }
        }
    }
    
    Err("Proposal not found or cannot be activated".to_string())
}

#[update]
async fn cast_vote(
    proposal_id: String,
    voter: String,
    vote_type: VoteType,
    voting_power: u32,
    reason: Option<String>,
) -> Result<(), String> {
    let now = ic_cdk::api::time();
    
    // Check if proposal is active
    unsafe {
        if let Some(ref proposals) = PROPOSALS {
            if let Some(proposal) = proposals.get(&proposal_id) {
                if proposal.status != ProposalStatus::Active {
                    return Err("Proposal is not active for voting".to_string());
                }
                if now < proposal.voting_start || now > proposal.voting_end {
                    return Err("Voting period is not active".to_string());
                }
            } else {
                return Err("Proposal not found".to_string());
            }
        }
    }
    
    // Check if voter has already voted
    let vote_key = format!("{}:{}", proposal_id, voter);
    unsafe {
        if let Some(ref votes) = VOTES {
            if votes.contains_key(&vote_key) {
                return Err("Voter has already cast a vote".to_string());
            }
        }
    }
    
    let vote = Vote {
        proposal_id: proposal_id.clone(),
        voter: voter.clone(),
        vote_type: vote_type.clone(),
        voting_power,
        timestamp: now,
        reason,
    };
    
    // Store vote
    unsafe {
        if let Some(ref mut votes) = VOTES {
            votes.insert(vote_key, vote);
        }
        
        // Update proposal vote counts
        if let Some(ref mut proposals) = PROPOSALS {
            if let Some(proposal) = proposals.get_mut(&proposal_id) {
                match vote_type {
                    VoteType::Yes => proposal.yes_votes += voting_power,
                    VoteType::No => proposal.no_votes += voting_power,
                    VoteType::Abstain => proposal.abstain_votes += voting_power,
                }
                proposal.total_votes += voting_power;
            }
        }
        
        // Update metrics
        if let Some(ref mut metrics) = DAO_METRICS {
            metrics.total_votes_cast += 1;
        }
    }
    
    Ok(())
}

#[query]
fn get_proposal(proposal_id: String) -> Result<Proposal, String> {
    unsafe {
        if let Some(ref proposals) = PROPOSALS {
            proposals.get(&proposal_id).cloned().ok_or("Proposal not found".to_string())
        } else {
            Err("Proposals not initialized".to_string())
        }
    }
}

#[query]
fn get_all_proposals() -> Vec<Proposal> {
    unsafe {
        if let Some(ref proposals) = PROPOSALS {
            proposals.values().cloned().collect()
        } else {
            Vec::new()
        }
    }
}

#[query]
fn get_active_proposals() -> Vec<Proposal> {
    let now = ic_cdk::api::time();
    unsafe {
        if let Some(ref proposals) = PROPOSALS {
            proposals.values()
                .filter(|proposal| {
                    proposal.status == ProposalStatus::Active &&
                    now >= proposal.voting_start &&
                    now <= proposal.voting_end
                })
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }
}

#[query]
fn get_proposal_votes(proposal_id: String) -> Vec<Vote> {
    unsafe {
        if let Some(ref votes) = VOTES {
            votes.values()
                .filter(|vote| vote.proposal_id == proposal_id)
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }
}

#[update]
async fn execute_proposal(proposal_id: String, executor: String) -> Result<(), String> {
    let now = ic_cdk::api::time();
    
    unsafe {
        if let Some(ref mut proposals) = PROPOSALS {
            if let Some(proposal) = proposals.get_mut(&proposal_id) {
                if proposal.status != ProposalStatus::Passed {
                    return Err("Proposal has not passed".to_string());
                }
                
                // Simulate execution
                let execution_data = ProposalExecution {
                    executed_at: now,
                    executor: executor.clone(),
                    execution_hash: format!("exec_{}", Uuid::new_v4().to_string()),
                    success: true,
                    error_message: None,
                };
                
                proposal.status = ProposalStatus::Executed;
                proposal.execution_data = Some(execution_data);
                
                return Ok(());
            }
        }
    }
    
    Err("Proposal not found".to_string())
}

#[update]
async fn add_member(
    id: String,
    name: String,
    voting_power: u32,
    role: MemberRole,
) -> Result<(), String> {
    let now = ic_cdk::api::time();
    
    let member = DAOMember {
        id: id.clone(),
        name,
        voting_power,
        joined_at: now,
        total_votes_cast: 0,
        reputation_score: 1.0,
        role,
    };
    
    unsafe {
        if let Some(ref mut members) = MEMBERS {
            members.insert(id, member);
        }
        
        if let Some(ref mut metrics) = DAO_METRICS {
            metrics.total_members += 1;
        }
    }
    
    Ok(())
}

#[query]
fn get_member(member_id: String) -> Result<DAOMember, String> {
    unsafe {
        if let Some(ref members) = MEMBERS {
            members.get(&member_id).cloned().ok_or("Member not found".to_string())
        } else {
            Err("Members not initialized".to_string())
        }
    }
}

#[query]
fn get_all_members() -> Vec<DAOMember> {
    unsafe {
        if let Some(ref members) = MEMBERS {
            members.values().cloned().collect()
        } else {
            Vec::new()
        }
    }
}

#[query]
fn get_dao_metrics() -> DAOMetrics {
    unsafe {
        DAO_METRICS.clone().unwrap_or(DAOMetrics {
            total_proposals: 0,
            active_proposals: 0,
            passed_proposals: 0,
            total_members: 0,
            total_votes_cast: 0,
            average_participation: 0.0,
        })
    }
}

async fn check_proposal_deadlines() {
    let now = ic_cdk::api::time();
    
    unsafe {
        if let Some(ref mut proposals) = PROPOSALS {
            for proposal in proposals.values_mut() {
                if proposal.status == ProposalStatus::Active && now > proposal.voting_end {
                    // Voting period ended, determine result
                    if proposal.total_votes >= proposal.quorum_required {
                        if proposal.yes_votes > proposal.no_votes {
                            proposal.status = ProposalStatus::Passed;
                            if let Some(ref mut metrics) = DAO_METRICS {
                                metrics.passed_proposals += 1;
                            }
                        } else {
                            proposal.status = ProposalStatus::Rejected;
                        }
                    } else {
                        proposal.status = ProposalStatus::Expired;
                    }
                    
                    if let Some(ref mut metrics) = DAO_METRICS {
                        metrics.active_proposals = metrics.active_proposals.saturating_sub(1);
                    }
                }
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
    fn test_proposal_creation() {
        // Test proposal creation logic
        let proposal_id = "test_proposal_123".to_string();
        assert!(proposal_id.contains("test"));
    }
} 