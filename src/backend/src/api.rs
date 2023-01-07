use crate::CANISTER_DATA;

#[ic_cdk_macros::update]
#[candid::candid_method(update)]
fn increment_1() -> u64 {
    CANISTER_DATA.with(|data| {
        let mut data = data.borrow_mut();
        data.counter += 1;
        data.counter
    })
}

#[ic_cdk_macros::update]
fn increment_2() -> u64 {
    CANISTER_DATA.with(|data| {
        let mut data = data.borrow_mut();
        data.counter += 1;
        data.counter
    })
}
