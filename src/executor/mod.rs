/// This mod defines the structures related to `executor`.
///

mod executor;
mod queue;

use super::task::*;
pub use executor::*;

/// pub use priority::PRIO_LEVEL;
pub use priority::PRIO_LEVEL;

///
mod priority {
    #[cfg(feature = "prio-level-4")]
    ///
    pub const PRIO_LEVEL: usize = 4;
    #[cfg(feature = "prio-level-8")]
    ///
    pub const PRIO_LEVEL: usize = 8;
    #[cfg(feature = "prio-level-16")]
    ///
    pub const PRIO_LEVEL: usize = 16;
}
