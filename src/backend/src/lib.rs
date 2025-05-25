use candid::Principal;
use ic_cdk::export_candid;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;

use ic_llm::{ChatMessage, Model};

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static PRINCIPAL_COUNTERS: RefCell<StableBTreeMap<Principal, u64, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );
}

#[ic_cdk::update]
async fn prompt(prompt_str: String) -> String {
    ic_llm::prompt(Model::Llama3_1_8B, prompt_str).await
}

#[ic_cdk::update]
async fn chat(messages: Vec<ChatMessage>) -> String {
    let response = ic_llm::chat(Model::Llama3_1_8B)
        .with_messages(messages)
        .send()
        .await;

    // A response can contain tool calls, but we're not calling tools in this project,
    // so we can return the response message directly.
    response.message.content.unwrap_or_default()
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::update]
fn increment() -> u64 {
    let caller = ic_cdk::caller();
    PRINCIPAL_COUNTERS.with(|counters| {
        let mut counters = counters.borrow_mut();
        let current_count = counters.get(&caller).unwrap_or(0);
        let new_count = current_count + 1;
        counters.insert(caller, new_count);
        new_count
    })
}

#[ic_cdk::query]
fn get_count() -> u64 {
    let caller = ic_cdk::caller();
    PRINCIPAL_COUNTERS.with(|counters| counters.borrow().get(&caller).unwrap_or(0))
}

#[ic_cdk::update]
fn set_count(value: u64) -> u64 {
    let caller = ic_cdk::caller();
    PRINCIPAL_COUNTERS.with(|counters| {
        let mut counters = counters.borrow_mut();
        counters.insert(caller, value);
        value
    })
}

export_candid!();
