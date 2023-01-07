use std::cell::RefCell;

use candid::export_service;
use data::CanisterData;

mod api;
mod data;
#[cfg(test)]
mod test;

thread_local! {
    pub static CANISTER_DATA: RefCell<CanisterData> = RefCell::new(CanisterData::default());
}

#[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}
