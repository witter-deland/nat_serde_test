use candid::Principal;

use crate::errors::{CommonError, ServiceResult};
use crate::named_canister_ids::{is_named_canister_id, CanisterNames};
use crate::named_principals::{get_named_principals, is_named_principal, PRINCIPAL_NAME_ADMIN};
use crate::types::{AuthPrincipal, CanisterId};

pub fn must_be_system_owner(caller: &Principal) -> ServiceResult<()> {
    must_not_anonymous(caller)?;
    if !is_admin(caller) {
        return Err(CommonError::Unauthorized);
    }
    Ok(())
}

pub fn must_be_named_principal(caller: &Principal, name: &str) -> ServiceResult<()> {
    must_not_anonymous(caller)?;
    if !is_named_principal(name, caller) {
        return Err(CommonError::Unauthorized);
    }
    Ok(())
}

pub fn must_be_in_named_principal(caller: &Principal, names: &[&str]) -> ServiceResult<()> {
    must_not_anonymous(caller)?;
    for name in names {
        if is_named_principal(name, caller) {
            return Ok(());
        }
    }
    return Err(CommonError::Unauthorized);
}

pub fn must_be_named_canister(caller: Principal, name: CanisterNames) -> ServiceResult<()> {
    must_not_anonymous(&caller)?;
    if !is_named_canister_id(name, CanisterId(caller)) {
        return Err(CommonError::Unauthorized);
    }
    Ok(())
}

pub fn must_be_in_named_canister(caller: Principal, names: &[CanisterNames]) -> ServiceResult<()> {
    must_not_anonymous(&caller)?;
    for name in names {
        if is_named_canister_id(*name, CanisterId(caller)) {
            return Ok(());
        }
    }
    return Err(CommonError::Unauthorized);
}

pub fn must_not_anonymous(caller: &Principal) -> ServiceResult<AuthPrincipal> {
    if *caller == Principal::anonymous() {
        return Err(CommonError::Unauthorized);
    }
    Ok(AuthPrincipal(caller.clone()))
}

pub fn is_admin(user: &Principal) -> bool {
    is_named_principal(PRINCIPAL_NAME_ADMIN, user)
}

pub fn get_admin() -> Principal {
    get_named_principals(PRINCIPAL_NAME_ADMIN)
        .into_iter()
        .next()
        .unwrap()
}
