use candid::{candid_method, CandidType, Deserialize, Nat};
use ic_cdk_macros::*;
use log::{debug, info};
use num_bigint::BigUint;
use serde::de;

use common::permissions::{is_admin, must_be_system_owner};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TestRequest {
    #[serde(deserialize_with = "deserialize_nat")]
    pub num_req: Nat,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TestResponse {
    #[serde(deserialize_with = "deserialize_nat")]
    pub num_res: Nat,
}

#[query(name = "test")]
#[candid_method(query, rename = "test")]
fn test(req: TestRequest) -> TestResponse {
    TestResponse {
        num_res: req.num_req,
    }
}

candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query, rename = "__get_candid_interface_tmp_hack")]
fn __export_did_tmp_() -> String {
    __export_service()
}

fn deserialize_nat<'de, D>(deserializer: D) -> Result<Nat, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: BigUint = de::Deserialize::deserialize(deserializer)?;
    Ok(Nat::from(s))
}
