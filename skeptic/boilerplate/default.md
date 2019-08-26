```rust,skt-example
#![allow(dead_code)]
#![allow(unused_imports)]

#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::{{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
}};
use hdk::holochain_core_types::{{
    entry::Entry,
    dna::entry_types::Sharing,
}};

use hdk::holochain_json_api::{{
    json::JsonString,
    error::JsonError
}};

use hdk::holochain_persistence_api::{{
    cas::content::Address
}};

use hdk_proc_macros::zome;
// see https://developer.holochain.org/api/0.0.18-alpha1/hdk/ for info on using the hdk library
fn main() {{
#[zome]
mod my_zome {{
{}
#[validate_agent]
pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {{
    Ok(())
    }}

}}
}}

```
