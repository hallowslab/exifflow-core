//! Shared, transport-agnostic upload plumbing for ExifFlow transfer front-ends.
//!
//! Extracted from `rftps` so the FTP server and the HTTP upload server (`recv`)
//! publish identical events and write the same durable replication records.

pub mod event;
pub mod record;
pub mod utils;

pub use event::{EventBus, FtpEvent};
pub use record::RecordStore;
pub use utils::{generate_random_string, resolve_local_ip, validate_certificates, verify_home};
