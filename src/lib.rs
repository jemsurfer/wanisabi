///The crate's error type. Aggregates errors from:
/// - Wanikani
/// - Reqwest
/// - Serde
pub use error::Error;
pub use wanisabi_model as model;

pub mod client;
pub mod error;
pub mod response;
extern crate serde_qs as qs;
#[macro_use]
pub mod wrapper;
