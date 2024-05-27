use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct ModelMessages {
    init_work: String,
    finding_version: String,
    make_link: String,
}

impl ModelMessages {
    pub fn init_work(&self) -> &str {
        &self.init_work
    }

    pub fn finding_version(&self) -> &str {
        &self.finding_version
    }

    pub fn make_link(&self) -> &str {
        &self.make_link
    }
}

impl Default for ModelMessages {
    fn default() -> Self {
        Self {
            init_work: "Generating link...".into(),
            finding_version: "Finding version...".into(),
            make_link: "Making link...".into(),
        }
    }
}
