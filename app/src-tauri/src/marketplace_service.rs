use crate::service::{AppError, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct ModStatus {
    id: String,
    state: String,
    message: String,
    can_install: bool,
    can_remove: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Snapshot {
    mods: Vec<ModStatus>,
    game_running: bool,
}

pub fn invoke(_: &Path, _: &Path, _: Option<(&str, bool)>) -> Result<Snapshot> {
    Err(AppError {
        code: "marketplace_mod",
        message: "Marketplace runtime modules are planned for a future macOS release.".into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marketplace_modules_are_unavailable() {
        assert!(invoke(Path::new("."), Path::new("."), None).is_err());
    }
}
