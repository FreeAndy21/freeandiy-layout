//! Project links (Help menu, About, start screen, CLI, MCP).

use serde_json::{Value, json};

/// The app's slug on getartcraft.com and GitHub.
pub const APP: &str = "designcraft";
/// The FreeANDIY website.
pub const WEBSITE: &str = "https://freeandiy.freeandy.agency";
/// DesignCraft's page on the FreeANDIY website.
pub const APP_PAGE: &str = "https://freeandiy.freeandy.agency/layout";
/// DesignCraft's source repository.
pub const GITHUB: &str = "https://github.com/FreeAndy21/freeandiy-layout";
/// Where to report bugs and request features.
pub const ISSUES: &str = "https://github.com/FreeAndy21/freeandiy-layout/issues";

/// All links as JSON (the `app.links` command).
pub fn all() -> Value {
    json!({
        "website": WEBSITE,
        "appPage": APP_PAGE,
        "github": GITHUB,
        "issues": ISSUES,
    })
}

/// A link by key (`website`, `appPage`, `github`, `issues`).
pub fn get(key: &str) -> Option<&'static str> {
    Some(match key {
        "website" => WEBSITE,
        "appPage" => APP_PAGE,
        "github" => GITHUB,
        "issues" => ISSUES,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_follow_the_app_slug() {
        assert!(APP_PAGE.starts_with(WEBSITE));
        assert!(GITHUB.starts_with("https://github.com/"));
        assert!(ISSUES.starts_with(GITHUB));
        assert!(get("discord").is_none());
        for k in ["website", "appPage", "github", "issues"] {
            assert!(get(k).is_some_and(|u| u.starts_with("https://")), "{k}");
            assert_eq!(all()[k], get(k).unwrap());
        }
    }
}
