//! Static build info for Oeksound Soothe 2 Live.

pub const BUILD_VERSION: &str  = "v3.6.2";
pub const BUILD_CHANNEL: &str  = "beta";
pub const BUILD_COMMIT: &str   = "aaf67a124d9e";
pub const BUILD_TIMESTAMP: u64 = 1704362890;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Oeksound Soothe 2 Live", BUILD_VERSION, BUILD_CHANNEL)
}
