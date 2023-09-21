use core::num;

use candid::Principal;

use crate::account_identifier::{ AccountIdentifier, Subaccount };

// Check if admin/authorised
pub fn validate_caller(principalID: String, admins: Vec<String>) {
    let mut auth: bool = false;
    if admins.contains(&principalID) {
        auth = true;
    }
    match auth {
        true => (),
        _ => ic_cdk::trap("Caller Not Authorised"),
    }
}


pub fn get_subaccount_from_principal(principal_id: String, subaccount: u8) -> String {
    let pncpl = Principal::from_text(principal_id).expect("Could not decode the principal.");
    let mut sub = [0; 32];
    sub[31] = subaccount;
    let sub_ac: Subaccount = Subaccount(sub);
    let sub_account = AccountIdentifier::new(pncpl, Some(sub_ac));
    return AccountIdentifier::to_hex(&sub_account);
}

pub fn get_multiple_subaccounts_from_principal(
    principal_id: String,
    start: u8,
    end: u8
) -> Vec<String> {
    let mut output: Vec<String> = vec![];
    let mut pncpl;
    let mut sub = [0; 32];
    let mut sub_ac: Subaccount;
    let mut sub_account;
    for x in start..=end {
        pncpl = Principal::from_text(principal_id.clone()).expect(
            "Could not decode the principal."
        );
        sub[31] = x;
        sub_ac = Subaccount(sub);
        sub_account = AccountIdentifier::new(pncpl, Some(sub_ac));
        output.push(AccountIdentifier::to_hex(&sub_account));
    }
    return output;
}

// LOG

// CYCLES/ STORAGE ETC
