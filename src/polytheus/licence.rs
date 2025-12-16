use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
/// Licence referenced by models.
pub struct Licence {
    /// Licence short name (e.g. "MIT").
    name: String,

    /// Optional URL for the licence text.
    #[serde(rename = "URL")]
    url: Option<String>,

    /// Openness flags describing what is shared.
    open_source: bool,

    /// Whether commercial use is permitted.
    commercial_use: bool,

    /// Whether the licence is considered free-software compatible.
    free_software: bool,
}

impl Licence {
    /// fill all the licences available with their different properties
    pub fn fill() -> Vec<Licence> {
        vec![
            Licence {
                name: "Proprietary".to_string(),
                url: None,
                open_source: false,
                commercial_use: true,
                free_software: false,
            },
            Licence {
                name: "Apache-2.0".to_string(),
                url: Some("https://www.apache.org/licenses/LICENSE-2.0".to_string()),
                open_source: true,
                commercial_use: true,
                free_software: true,
            },
            Licence {
                name: "MIT".to_string(),
                url: Some("https://opensource.org/licenses/MIT".to_string()),
                open_source: true,
                commercial_use: true,
                free_software: true,
            },
            Licence {
                name: "CC-BY-4.0".to_string(),
                url: Some("https://creativecommons.org/licenses/by/4.0/".to_string()),
                open_source: true,
                commercial_use: true,
                free_software: false,
            },
        ]
    }
}
