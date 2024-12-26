/// Local development service.
pub mod service;

/// Development chain specs.
pub mod chain_spec;

pub use chain_spec::*;
pub use service::new_partial;
