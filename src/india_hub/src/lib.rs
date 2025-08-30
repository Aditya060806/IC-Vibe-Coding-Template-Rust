use candid::{CandidType, Deserialize};
use ic_cdk::{api::call::call, export::candid, init, post_upgrade, pre_upgrade, query, update};
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};
use std::collections::BTreeMap;
use uuid::Uuid;

// India Hub Integration Constants
const AADHAAR_API_ENDPOINT: &str = "https://api.uidai.gov.in";
const GST_API_ENDPOINT: &str = "https://api.gst.gov.in";
const DIGITAL_LOCKER_ENDPOINT: &str = "https://api.digitallocker.gov.in";
const WCHL25_HACKATHON_ID: &str = "WCHL25_CIVICLEDGER_INDIA_HUB";

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct IndiaHubRegistration {
    pub policy_id: String,
    pub registration_id: String,
    pub hub_verification_status: bool,
    pub compliance_score: f64,
    pub regional_impact_score: f64,
    pub timestamp: u64,
    // Enhanced India Hub Features
    pub aadhaar_integration: Option<AadhaarVerification>,
    pub gst_verification: Option<GSTVerification>,
    pub pan_card_validation: Option<PANValidation>,
    pub regional_compliance: Vec<RegionalCompliance>,
    pub digital_locker_integration: Option<DigitalLockerEntry>,
    pub biometric_verification: Option<BiometricVerification>,
    pub e_kyc_status: Option<EKYCStatus>,
    pub compliance_audit: ComplianceAudit,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct AadhaarVerification {
    pub aadhaar_number: String,
    pub verification_status: bool,
    pub biometric_match: bool,
    pub otp_verified: bool,
    pub verification_timestamp: u64,
    pub verification_score: f64,
    pub demographic_data: DemographicData,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct GSTVerification {
    pub gst_number: String,
    pub business_name: String,
    pub registration_status: String,
    pub compliance_status: String,
    pub last_filing_date: u64,
    pub verification_score: f64,
    pub tax_compliance: TaxCompliance,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct PANValidation {
    pub pan_number: String,
    pub holder_name: String,
    pub validation_status: bool,
    pub verification_timestamp: u64,
    pub verification_score: f64,
    pub kyc_status: String,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct RegionalCompliance {
    pub state: String,
    pub district: String,
    pub compliance_rules: Vec<String>,
    pub compliance_status: bool,
    pub compliance_score: f64,
    pub regional_authority: String,
    pub approval_date: u64,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct DigitalLockerEntry {
    pub locker_id: String,
    pub document_type: String,
    pub document_hash: String,
    pub upload_timestamp: u64,
    pub verification_status: bool,
    pub access_permissions: Vec<String>,
    pub document_metadata: DocumentMetadata,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct BiometricVerification {
    pub biometric_type: String,
    pub verification_status: bool,
    pub match_score: f64,
    pub verification_timestamp: u64,
    pub device_id: String,
    pub location: String,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct EKYCStatus {
    pub kyc_status: String,
    pub verification_level: String,
    pub last_updated: u64,
    pub verification_score: f64,
    pub compliance_requirements: Vec<String>,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct ComplianceAudit {
    pub audit_id: String,
    pub audit_date: u64,
    pub compliance_score: f64,
    pub audit_findings: Vec<AuditFinding>,
    pub recommendations: Vec<String>,
    pub next_audit_date: u64,
    pub auditor: String,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct DemographicData {
    pub name: String,
    pub date_of_birth: String,
    pub gender: String,
    pub address: String,
    pub photo_hash: String,
    pub verification_status: bool,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct TaxCompliance {
    pub filing_frequency: String,
    pub last_filing_period: String,
    pub tax_liability: f64,
    pub compliance_score: f64,
    pub pending_returns: i32,
    pub penalty_amount: f64,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct DocumentMetadata {
    pub file_name: String,
    pub file_size: u64,
    pub mime_type: String,
    pub upload_source: String,
    pub verification_hash: String,
    pub expiry_date: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct AuditFinding {
    pub finding_id: String,
    pub severity: String,
    pub description: String,
    pub recommendation: String,
    pub status: String,
    pub due_date: u64,
}

// Stable storage
static mut REGISTRATIONS: Option<BTreeMap<String, IndiaHubRegistration>> = None;
static mut COMPLIANCE_RULES: Option<BTreeMap<String, Vec<String>>> = None;
static mut VERIFICATION_LOGS: Option<BTreeMap<String, Vec<VerificationLog>>> = None;

#[derive(CandidType, Deserialize, Clone, SerdeSerialize, SerdeDeserialize)]
pub struct VerificationLog {
    pub log_id: String,
    pub policy_id: String,
    pub verification_type: String,
    pub status: bool,
    pub timestamp: u64,
    pub details: String,
    pub blockchain_hash: Option<String>,
}

#[init]
fn init() {
    unsafe {
        REGISTRATIONS = Some(BTreeMap::new());
        COMPLIANCE_RULES = Some(BTreeMap::new());
        VERIFICATION_LOGS = Some(BTreeMap::new());
        
        // Initialize compliance rules for different states
        if let Some(ref mut rules) = COMPLIANCE_RULES {
            rules.insert("Maharashtra".to_string(), vec![
                "Maharashtra Public Trusts Act".to_string(),
                "Bombay Public Trusts Rules".to_string(),
                "Maharashtra Transparency Act".to_string(),
            ]);
            rules.insert("Delhi".to_string(), vec![
                "Delhi Societies Registration Act".to_string(),
                "Delhi Transparency Act".to_string(),
            ]);
            rules.insert("Karnataka".to_string(), vec![
                "Karnataka Societies Registration Act".to_string(),
                "Karnataka Transparency Act".to_string(),
            ]);
        }
    }
    
    ic_cdk::println!("🚀 WCHL25: India Hub initialized successfully");
}

#[pre_upgrade]
fn pre_upgrade() {
    let registrations = unsafe { REGISTRATIONS.take().unwrap() };
    let compliance_rules = unsafe { COMPLIANCE_RULES.take().unwrap() };
    let verification_logs = unsafe { VERIFICATION_LOGS.take().unwrap() };
    
    ic_cdk::storage::stable_save((registrations, compliance_rules, verification_logs)).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    let (registrations, compliance_rules, verification_logs): (
        BTreeMap<String, IndiaHubRegistration>,
        BTreeMap<String, Vec<String>>,
        BTreeMap<String, Vec<VerificationLog>>,
    ) = ic_cdk::storage::stable_restore().unwrap();
    
    unsafe {
        REGISTRATIONS = Some(registrations);
        COMPLIANCE_RULES = Some(compliance_rules);
        VERIFICATION_LOGS = Some(verification_logs);
    }
}

#[update]
async fn register_with_india_hub(
    policy_id: String,
    district: String,
    fund_allocation: u64,
) -> Result<IndiaHubRegistration, String> {
    let registration_id = format!("INDIA_HUB_{}", Uuid::new_v4().to_string());
    let now = ic_cdk::api::time();
    
    // Simulate Aadhaar verification
    let aadhaar_verification = verify_aadhaar(&policy_id).await;
    
    // Simulate GST verification
    let gst_verification = verify_gst(&policy_id).await;
    
    // Simulate PAN validation
    let pan_validation = validate_pan(&policy_id).await;
    
    // Check regional compliance
    let regional_compliance = check_regional_compliance(&district).await;
    
    // Create digital locker entry
    let digital_locker = create_digital_locker_entry(&policy_id, &registration_id).await;
    
    // Perform biometric verification
    let biometric_verification = perform_biometric_verification(&policy_id).await;
    
    // Complete e-KYC
    let e_kyc_status = complete_ekyc(&policy_id).await;
    
    // Conduct compliance audit
    let compliance_audit = conduct_compliance_audit(&policy_id, &district).await;
    
    let registration = IndiaHubRegistration {
        policy_id: policy_id.clone(),
        registration_id: registration_id.clone(),
        hub_verification_status: true,
        compliance_score: calculate_compliance_score(&regional_compliance),
        regional_impact_score: calculate_regional_impact_score(&district, fund_allocation),
        timestamp: now,
        aadhaar_integration: aadhaar_verification,
        gst_verification,
        pan_card_validation: pan_validation,
        regional_compliance,
        digital_locker_integration: digital_locker,
        biometric_verification,
        e_kyc_status,
        compliance_audit,
    };
    
    unsafe {
        if let Some(ref mut registrations) = REGISTRATIONS {
            registrations.insert(policy_id.clone(), registration.clone());
        }
        
        // Log verification
        if let Some(ref mut logs) = VERIFICATION_LOGS {
            let log_entry = VerificationLog {
                log_id: format!("LOG_{}", Uuid::new_v4().to_string()),
                policy_id: policy_id.clone(),
                verification_type: "India Hub Registration".to_string(),
                status: true,
                timestamp: now,
                details: "Policy registered with India Hub successfully".to_string(),
                blockchain_hash: Some(generate_blockchain_hash(&policy_id)),
            };
            
            if let Some(logs_for_policy) = logs.get_mut(&policy_id) {
                logs_for_policy.push(log_entry);
            } else {
                logs.insert(policy_id, vec![log_entry]);
            }
        }
    }
    
    ic_cdk::println!("✅ WCHL25: Policy {} registered with India Hub", policy_id);
    
    Ok(registration)
}

#[query]
fn get_registrations() -> Vec<IndiaHubRegistration> {
    unsafe {
        if let Some(ref registrations) = REGISTRATIONS {
            registrations.values().cloned().collect()
        } else {
            Vec::new()
        }
    }
}

#[query]
fn get_registration(policy_id: String) -> Result<IndiaHubRegistration, String> {
    unsafe {
        if let Some(ref registrations) = REGISTRATIONS {
            registrations.get(&policy_id).cloned().ok_or("Registration not found".to_string())
        } else {
            Err("Registrations not initialized".to_string())
        }
    }
}

#[query]
fn get_verification_logs(policy_id: String) -> Vec<VerificationLog> {
    unsafe {
        if let Some(ref logs) = VERIFICATION_LOGS {
            logs.get(&policy_id).cloned().unwrap_or_default()
        } else {
            Vec::new()
        }
    }
}

#[update]
async fn update_compliance_score(
    policy_id: String,
    new_score: f64,
) -> Result<(), String> {
    unsafe {
        if let Some(ref mut registrations) = REGISTRATIONS {
            if let Some(registration) = registrations.get_mut(&policy_id) {
                registration.compliance_score = new_score;
                registration.timestamp = ic_cdk::api::time();
                
                // Update compliance audit
                registration.compliance_audit.compliance_score = new_score;
                registration.compliance_audit.audit_date = ic_cdk::api::time();
                
                return Ok(());
            }
        }
    }
    Err("Registration not found".to_string())
}

// Helper functions
async fn verify_aadhaar(policy_id: &str) -> Option<AadhaarVerification> {
    // Simulate Aadhaar verification
    Some(AadhaarVerification {
        aadhaar_number: format!("{}", (100000000000..999999999999).collect::<Vec<u64>>()[0]),
        verification_status: true,
        biometric_match: true,
        otp_verified: true,
        verification_timestamp: ic_cdk::api::time(),
        verification_score: 0.98,
        demographic_data: DemographicData {
            name: "Citizen Name".to_string(),
            date_of_birth: "1990-01-01".to_string(),
            gender: "Male".to_string(),
            address: "Mumbai, Maharashtra".to_string(),
            photo_hash: format!("PHOTO_{}", policy_id),
            verification_status: true,
        },
    })
}

async fn verify_gst(policy_id: &str) -> Option<GSTVerification> {
    // Simulate GST verification
    Some(GSTVerification {
        gst_number: format!("27AABCA1234A1Z5"),
        business_name: "CivicLedger Solutions".to_string(),
        registration_status: "Active".to_string(),
        compliance_status: "Compliant".to_string(),
        last_filing_date: ic_cdk::api::time(),
        verification_score: 0.95,
        tax_compliance: TaxCompliance {
            filing_frequency: "Monthly".to_string(),
            last_filing_period: "2024-01".to_string(),
            tax_liability: 50000.0,
            compliance_score: 0.95,
            pending_returns: 0,
            penalty_amount: 0.0,
        },
    })
}

async fn validate_pan(policy_id: &str) -> Option<PANValidation> {
    // Simulate PAN validation
    Some(PANValidation {
        pan_number: format!("ABCDE1234F"),
        holder_name: "CivicLedger Solutions".to_string(),
        validation_status: true,
        verification_timestamp: ic_cdk::api::time(),
        verification_score: 0.99,
        kyc_status: "Verified".to_string(),
    })
}

async fn check_regional_compliance(district: &str) -> Vec<RegionalCompliance> {
    // Check compliance for the district
    let state = if district.contains("Mumbai") || district.contains("Pune") {
        "Maharashtra"
    } else if district.contains("Delhi") {
        "Delhi"
    } else if district.contains("Bangalore") {
        "Karnataka"
    } else {
        "Maharashtra"
    };
    
    unsafe {
        if let Some(ref rules) = COMPLIANCE_RULES {
            if let Some(compliance_rules) = rules.get(state) {
                vec![RegionalCompliance {
                    state: state.to_string(),
                    district: district.to_string(),
                    compliance_rules: compliance_rules.clone(),
                    compliance_status: true,
                    compliance_score: 0.92,
                    regional_authority: format!("{} Regional Authority", state),
                    approval_date: ic_cdk::api::time(),
                }]
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }
}

async fn create_digital_locker_entry(policy_id: &str, registration_id: &str) -> Option<DigitalLockerEntry> {
    Some(DigitalLockerEntry {
        locker_id: format!("DL_{}", registration_id),
        document_type: "Policy Registration".to_string(),
        document_hash: generate_blockchain_hash(policy_id),
        upload_timestamp: ic_cdk::api::time(),
        verification_status: true,
        access_permissions: vec!["Government".to_string(), "Citizen".to_string()],
        document_metadata: DocumentMetadata {
            file_name: format!("policy_{}.pdf", policy_id),
            file_size: 1024000,
            mime_type: "application/pdf".to_string(),
            upload_source: "CivicLedger".to_string(),
            verification_hash: generate_blockchain_hash(policy_id),
            expiry_date: Some(ic_cdk::api::time() + 365 * 24 * 60 * 60 * 1_000_000_000),
        },
    })
}

async fn perform_biometric_verification(policy_id: &str) -> Option<BiometricVerification> {
    Some(BiometricVerification {
        biometric_type: "Fingerprint".to_string(),
        verification_status: true,
        match_score: 0.97,
        verification_timestamp: ic_cdk::api::time(),
        device_id: "BIOMETRIC_DEVICE_001".to_string(),
        location: "Mumbai, Maharashtra".to_string(),
    })
}

async fn complete_ekyc(policy_id: &str) -> Option<EKYCStatus> {
    Some(EKYCStatus {
        kyc_status: "Completed".to_string(),
        verification_level: "Level 2".to_string(),
        last_updated: ic_cdk::api::time(),
        verification_score: 0.96,
        compliance_requirements: vec![
            "Aadhaar Verification".to_string(),
            "PAN Validation".to_string(),
            "Biometric Verification".to_string(),
            "Address Verification".to_string(),
        ],
    })
}

async fn conduct_compliance_audit(policy_id: &str, district: &str) -> ComplianceAudit {
    ComplianceAudit {
        audit_id: format!("AUDIT_{}", Uuid::new_v4().to_string()),
        audit_date: ic_cdk::api::time(),
        compliance_score: 0.94,
        audit_findings: vec![
            AuditFinding {
                finding_id: "FINDING_001".to_string(),
                severity: "Low".to_string(),
                description: "Minor documentation improvement needed".to_string(),
                recommendation: "Update policy documentation".to_string(),
                status: "Open".to_string(),
                due_date: ic_cdk::api::time() + 30 * 24 * 60 * 60 * 1_000_000_000,
            },
        ],
        recommendations: vec![
            "Enhance transparency reporting".to_string(),
            "Improve citizen engagement".to_string(),
            "Strengthen audit trail".to_string(),
        ],
        next_audit_date: ic_cdk::api::time() + 90 * 24 * 60 * 60 * 1_000_000_000,
        auditor: "WCHL25 Audit Team".to_string(),
    }
}

fn calculate_compliance_score(regional_compliance: &[RegionalCompliance]) -> f64 {
    if regional_compliance.is_empty() {
        return 0.0;
    }
    
    let total_score: f64 = regional_compliance.iter().map(|rc| rc.compliance_score).sum();
    total_score / regional_compliance.len() as f64
}

fn calculate_regional_impact_score(district: &str, fund_allocation: u64) -> f64 {
    // Calculate impact based on district and fund allocation
    let base_score = 0.8;
    let fund_multiplier = (fund_allocation as f64 / 1_000_000_000.0).min(1.0);
    let district_multiplier = if district.contains("Mumbai") { 1.2 } else { 1.0 };
    
    (base_score * fund_multiplier * district_multiplier).min(1.0)
}

fn generate_blockchain_hash(data: &str) -> String {
    format!("0x{}{}", data, ic_cdk::api::time()).chars().take(64).collect()
}

// Candid interface
candid::export_service!();

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compliance_score_calculation() {
        let compliance = vec![
            RegionalCompliance {
                state: "Maharashtra".to_string(),
                district: "Mumbai".to_string(),
                compliance_rules: vec!["Rule 1".to_string()],
                compliance_status: true,
                compliance_score: 0.9,
                regional_authority: "Authority".to_string(),
                approval_date: 0,
            },
            RegionalCompliance {
                state: "Maharashtra".to_string(),
                district: "Mumbai".to_string(),
                compliance_rules: vec!["Rule 2".to_string()],
                compliance_status: true,
                compliance_score: 0.8,
                regional_authority: "Authority".to_string(),
                approval_date: 0,
            },
        ];
        
        let score = calculate_compliance_score(&compliance);
        assert_eq!(score, 0.85);
    }
    
    #[test]
    fn test_regional_impact_score() {
        let score = calculate_regional_impact_score("Mumbai", 1_000_000_000);
        assert!(score > 0.8);
    }
}
