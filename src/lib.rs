#![doc = include_str!("../README.md")]



#[cfg(any(feature = "zle", feature = "builtins", feature = "modules"))]
#[macro_use]
mod hooks_import;

mod extensions;
mod base;
pub use base::*;

/// The zsh version this crate's bindings were generated/built against.
pub const ZSH_BUILD_VERSION: &str = env!("ZSH_BUILD_VERSION");


#[cfg(feature = "builtins")]
pub mod builtins;

#[cfg(feature = "modules")]
pub mod modules;

#[cfg(feature = "zle")]
pub mod zle;
