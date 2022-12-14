use candid::{CandidType, Deserialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

use crate::constants::*;
use crate::types::CanisterId;
use candid::Principal;
use ic_cdk::api;
use log::info;

thread_local! {
    pub static NAMED_CANISTER_IDS :RefCell<NamedCanisterIds> = RefCell::new(NamedCanisterIds::default());
    pub static DEV_NAMED_CANISTER_IDS :RefCell<HashMap<CanisterNames, Principal>> = RefCell::new(HashMap::default());
}

pub struct NamedCanisterIds {
    pub dynamic_canisters: HashMap<&'static str, Principal>,
    pub current_name: String,
}

impl Display for NamedCanisterIds {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (name, canister) in self.dynamic_canisters.iter() {
            writeln!(f, "{} = {}", name, canister)?;
        }
        Ok(())
    }
}

impl Default for NamedCanisterIds {
    fn default() -> Self {
        let map = HashMap::new();
        let result = NamedCanisterIds {
            dynamic_canisters: map,
            current_name: "".to_string(),
        };
        info!("named canister ids: {}", &result);
        result
    }
}

impl NamedCanisterIds {
    pub fn get_canister_id(&self, name: CanisterNames) -> CanisterId {
        match name {
            CanisterNames::MockSampleCanister => {
                CanisterId(*CANISTER_IDS_MOCK_SAMPLE_CANISTER.deref())
            }
            CanisterNames::ICLedger => CanisterId(*CANISTER_IDS_IC_LEDGER_CANISTER.deref()),
            CanisterNames::ICManagement => CanisterId(*CANISTER_IDS_IC_MANAGEMENT_CANISTER.deref()),
            CanisterNames::DFTCanister(canister_id) => canister_id,
        }
    }
}

pub fn get_named_canister_id(name: CanisterNames) -> CanisterId {
    NAMED_CANISTER_IDS.with(|n| {
        let n = n.borrow();
        n.get_canister_id(name)
    })
}

pub fn is_named_canister_id(name: CanisterNames, id: CanisterId) -> bool {
    NAMED_CANISTER_IDS.with(|n| {
        let n = n.borrow();
        n.get_canister_id(name) == id
    })
}

pub fn ensure_current_canister_id_match(name: CanisterNames) -> Result<(), String> {
    let current = CanisterId(api::id());
    let expected = get_named_canister_id(name);
    if current != expected {
        Err(format!(
            "Current canister id does not match expected canister id. Expected: {}, Current: {}",
            expected, current
        ))
    } else {
        info!(
            "Current canister id matches expected canister id, {}",
            current
        );
        Ok(())
    }
}

pub fn update_current_canister_name(name: &str) {
    NAMED_CANISTER_IDS.with(|n| {
        let mut x = n.borrow_mut();
        x.current_name = name.to_string();
    });
}

pub fn update_dev_named_canister_ids(ids: &HashMap<CanisterNames, Principal>) {
    DEV_NAMED_CANISTER_IDS.with(|n| {
        let mut x = n.borrow_mut();
        x.clear();
        x.extend(ids.iter());
    });
}

#[derive(Eq, Ord, PartialOrd, PartialEq, Hash, Debug, Copy, Clone, CandidType, Deserialize)]
#[repr(u8)]
pub enum CanisterNames {
    MockSampleCanister,
    DFTCanister(CanisterId),
    ICLedger,
    ICManagement,
}
