#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, symbol_short, String, log};

#[contracttype]
#[derive(Clone)]
pub struct ServiceRecord {
    pub id: u64,
    pub customer_name: String,
    pub vehicle_details: String,
    pub service_type: String,
    pub timestamp: u64,
    pub completed: bool,
}

const SERVICE_COUNT: Symbol = symbol_short!("SVC_CNT");

#[contract]
pub struct CarDetailingService;

#[contractimpl]
impl CarDetailingService {
    pub fn book_service(env: Env, customer_name: String, vehicle_details: String, service_type: String) -> u64 {
        let mut svc_count: u64 = env.storage().instance().get(&SERVICE_COUNT).unwrap_or(0);
        svc_count += 1;
        let now = env.ledger().timestamp();

        let record = ServiceRecord {
            id: svc_count,
            customer_name,
            vehicle_details,
            service_type,
            timestamp: now,
            completed: false,
        };

        env.storage().instance().set(&svc_count, &record);
        env.storage().instance().set(&SERVICE_COUNT, &svc_count);
        log!(&env, "Service booked with ID: {}", svc_count);
        svc_count
    }

    pub fn mark_complete(env: Env, id: u64) {
        let mut record: ServiceRecord = env.storage().instance().get(&id).expect("Service ID not found");
        record.completed = true;
        env.storage().instance().set(&id, &record);
        log!(&env, "Service ID: {} marked as complete", id);
    }

    pub fn view_service(env: Env, id: u64) -> ServiceRecord {
        env.storage().instance().get(&id).expect("Service ID not found")
    }
}
