#![feature(try_from)]

//! holochain_net is a library that defines an abstract networking layer for
//! different network transports, providing a configurable interface
//! for swapping different backends connection modules at load time

extern crate base64;
#[macro_use]
extern crate failure;
extern crate holochain_core_types;
extern crate holochain_net_connection;
extern crate holochain_net_ipc;
#[macro_use]
extern crate lazy_static;
extern crate regex;
// macros used in tests
#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;
#[allow(unused_imports)]
#[macro_use]
extern crate serde_derive;
#[macro_use]
pub extern crate holochain_core_types_derive;

pub mod error;
pub mod ipc_net_worker;
pub mod mock_worker;
pub mod p2p_config;
pub mod p2p_network;
