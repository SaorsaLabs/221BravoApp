use core::num;

use ic_cdk::export::Principal;
use crate::account_identifier::{ AccountIdentifier, Subaccount };

// Check if admin/authorised
pub fn validate_caller(principalID: String, admins: Vec<String>) {
    let mut auth: bool = false;
    let st: String = String::from(
        "pdwl5-wayx6-vn7cb-cq7nq-7yojw-oczeb-dvfbv-gkayl-6pkgf-l6g3g-cae"
    ); // Saorsa Admin
    if principalID == st {
        auth = true;
    } else if admins.contains(&principalID) {
        auth = true;
    }
    match auth {
        true => (),
        _ => ic_cdk::trap("Caller Not Authorised"),
    }
}

// INCORRECT !
// pub fn number_to_array(input: u32) -> [u8; 32] {
//     let mut array: [u8; 32] = [0; 32];
//     let digits: Vec<u8> = input
//         .to_string()
//         .chars()
//         .map(|c| c.to_digit(10).unwrap() as u8)
//         .collect();

//     let num_digits = digits.len();
//     let start_index;
//     if num_digits > 32 {return array}
//     else{
//         start_index = 32-num_digits;
//     };

//     for (i, digit) in digits.iter().enumerate() {
//         array[i+start_index] = *digit;
//     }
//     array
// }

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
