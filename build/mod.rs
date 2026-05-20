
#[cfg(feature = "test-stubs")]
mod stubs;
mod traits;

pub use crate::build::traits::{MaybeSplitToStubs, WriteToFile};