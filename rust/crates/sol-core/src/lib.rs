use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct LoadedFixture {
    pub document: Value,
}

impl LoadedFixture {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, FixtureLoadError> {
        let path = path.as_ref();
        let text = fs::read_to_string(path).map_err(FixtureLoadError::Io)?;
        let document = serde_json::from_str(&text).map_err(FixtureLoadError::Json)?;
        Ok(Self { document })
    }
}

#[derive(Debug)]
pub enum FixtureLoadError {
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl std::fmt::Display for FixtureLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(f, "failed to read fixture: {error}"),
            Self::Json(error) => write!(f, "failed to parse fixture JSON: {error}"),
        }
    }
}

impl std::error::Error for FixtureLoadError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .canonicalize()
            .expect("repository root must be resolvable")
    }

    #[test]
    fn loads_accepted_thermal_reference_fixture() {
        let path = repo_root().join("examples/thermal-reference-model-v0.1.json");
        let fixture = LoadedFixture::load(path).expect("thermal reference fixture must load");
        assert!(fixture.document.is_object());
    }

    #[test]
    fn loads_accepted_plasma_qrc_reference_fixture() {
        let path = repo_root().join("examples/plasma-qrc-reference-model-v0.1.json");
        let fixture = LoadedFixture::load(path).expect("plasma/QRC reference fixture must load");
        assert!(fixture.document.is_object());
    }
}
