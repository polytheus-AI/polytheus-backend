use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
/// Information about an organization providing models (not about organization that make benchmarks etc...).
pub struct Organization {
    /// Organization display name.
    name: String,

    /// Optional URL for the organization's site.
    #[serde(rename = "URL")]
    url: Option<String>,
}

impl Organization {
    /// fill all the organizations available with their different properties
    pub fn fill() -> Vec<Organization> {
        vec![
            Organization {
                name: "Open AI".to_string(),
                url: Some("https://openai.com".to_string()),
            },
            Organization {
                name: "Anthropic".to_string(),
                url: Some("https://www.anthropic.com".to_string()),
            },
            Organization {
                name: "Google DeepMind".to_string(),
                url: Some("https://deepmind.google".to_string()),
            },
            Organization {
                name: "Meta".to_string(),
                url: Some("https://ai.meta.com".to_string()),
            },
            Organization {
                name: "xAI".to_string(),
                url: Some("https://x.ai".to_string()),
            },
        ]
    }
}
