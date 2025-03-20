use super::{RepoStatus, Url};
use anyhow::Result;

pub trait Github {
    fn load_token(f: impl FnOnce(&str) -> Result<()>) -> Result<bool>;
    #[allow(dead_code)]
    fn save_token() -> Result<()>;
    fn archival_status(url: Url) -> Result<RepoStatus<()>>;
}

// smoelius: If `__real_github` is enabled, we assume that `--all-features` was passed and therefore
// disable `__mock_github`.

#[cfg(all(feature = "__mock_github", not(feature = "__real_github")))]
mod mock;
#[cfg(all(feature = "__mock_github", not(feature = "__real_github")))]
pub use mock::Impl;

#[cfg(any(not(feature = "__mock_github"), feature = "__real_github"))]
pub mod real;
#[cfg(any(not(feature = "__mock_github"), feature = "__real_github"))]
pub use real::Impl;
#[cfg(any(not(feature = "__mock_github"), feature = "__real_github"))]
pub use real::util;
