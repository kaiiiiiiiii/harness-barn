mod error;
mod github;
mod marketplace;
mod plugin;

pub use error::Error;
pub use github::{resolve_github_source, GitHubRef};
pub use marketplace::{parse_marketplace, Marketplace, PluginEntry};
pub use plugin::{parse_plugin_manifest, PluginManifest};

pub type Result<T> = std::result::Result<T, Error>;
