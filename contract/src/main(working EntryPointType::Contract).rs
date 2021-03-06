#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::{string::String, vec::Vec};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    ApiError, CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key, RuntimeArgs,
};

const KEY_NAME: &str = "my-key-name";
const RUNTIME_ARG_NAME: &str = "message";

/// An error enum which can be converted to a `u16` so it can be returned as an `ApiError::User`.
#[repr(u16)]
enum Error {
    KeyAlreadyExists = 0,
    KeyMismatch = 1,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        ApiError::User(error as u16)
    }
}

pub const DICTIONARY_REF: &str = "dictionary";
pub const DEFAULT_DICTIONARY_NAME: &str = "DefaultKey";
pub const DEFAULT_DICTIONARY_VALUE: &str = "Jiuhong";

#[no_mangle]
pub extern "C" fn test() {
    let dictionary_uref = storage::new_dictionary(DICTIONARY_REF).unwrap_or_revert();

    runtime::put_key("sun1", dictionary_uref.into());
    storage::dictionary_put(
        dictionary_uref,
        DEFAULT_DICTIONARY_NAME,
        DEFAULT_DICTIONARY_VALUE,
    );

    let maybe_value =
        storage::dictionary_get(dictionary_uref, DEFAULT_DICTIONARY_NAME).unwrap_or_revert();
    // Whether the value exists or not we're mostly interested in validation of access
    // rights
    let value: String = maybe_value.unwrap_or_default();
    runtime::put_key("value1", storage::new_uref(value).into());
}

#[no_mangle]
pub extern "C" fn call() {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "test",
        Vec::new(),
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    let (contract_hash, _) = storage::new_locked_contract(entry_points, None, None, None);
    runtime::put_key("dictionarycontract", contract_hash.into());
    runtime::call_contract(contract_hash, "test", RuntimeArgs::default())
}
