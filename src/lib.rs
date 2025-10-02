//! Galpha Protocol Buffers
//!
//! This crate contains protobuf definitions for the Galpha services.
//! The Rust code is generated from .proto files during build time.

pub mod ledger {
    pub mod http {
        // Include the generated prost code
        include!(concat!(env!("OUT_DIR"), "/ledger.v1.rs"));

        // Include the generated pbjson serialization implementations
        include!(concat!(env!("OUT_DIR"), "/ledger.v1.serde.rs"));
    }
}
