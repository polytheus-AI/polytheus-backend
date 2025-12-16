use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
/// Representation of an AI model entry in the catalog.
pub struct Model {
    /// Human readable name of the model (e.g. "gpt-4").
    name: String,

    /// Canonical URL for the model's API.
    #[serde(rename = "URL")]
    url: Option<String>,

    /// Provider of the model (e.g. "Replicate").
    provider: Provider,

    /// Name of the propeties for manipulating the level of "thinking".
    thinking_level_property: Option<String>,

    /// Type of thinking levels supported by the model.
    thinking_levels_authorized: Option<Vec<String>>,

    /// Optional characteristics of the model.
    #[serde(rename = "characteristic")]
    characteristic: Option<Characteristic>,

    /// Pricing information (one of per run or per IO).
    price: Price,

    /// Optional organization name that owns/provides the model.
    organization: Option<String>,

    /// Licence name referencing an entry in `licences`.
    licence: String,

    /// Optional list of capabilities (e.g. ["chat", "completion"]).
    capability: Option<Vec<String>>,

    /// Optional list of supported input modalities (e.g. ["text","audio"]).
    input_modality: Option<Vec<String>>,

    /// Optional list of supported output modalities (e.g. ["text","audio"]).
    output_modality: Option<Vec<String>>,

    /// Optional textual description of the model.
    description: Option<String>,

    /// URL of the api of the model.
    apiurl: String,

    /// the parameters to make for allowing the model to take a image as input.
    image_parameters: Option<String>,

    /// Role that the model can accept
    roles_authorized: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
/// Enumeration of known model providers.
pub enum Provider {
    Replicate,
    OpenRouter,
}

#[derive(Debug, Serialize, Deserialize)]
/// Characteristics of a model.
///
/// Many fields are optional because not all sources provide the same details.
pub struct Characteristic {
    /// total size in bytes (use u64 for large models).
    size: Option<u64>,

    /// Number of parameters.
    parameter_count: Option<u64>,

    /// Context window in tokens.
    context_window: Option<u32>,

    /// Architecture name (e.g. "transformer-decoder").
    architecture: Option<String>,

    /// Optional maximum output length (tokens).
    max_output_length: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
/// Pricing representation for models.
///
/// Supports either a single per-run price or a token-based IO pricing schedule.
/// For token-based pricing, each side (input/output) is a list of tiers. Each tier
/// defines an optional inclusive upper bound (max_tokens). The first tier that
/// matches the token count is used. Use `None` for `max_tokens` to indicate
/// "infinity" (fallback tier).
pub enum Price {
    /// Price charged per run/execution.
    PerRun { run_price: f64 },

    /// Price charged per token with tiered schedules for input and output.
    PerIoWithTiers {
        /// Pricing tiers for input tokens (price per million tokens).
        input_tiers: Vec<PriceTier>,
        /// Pricing tiers for output tokens (price per million tokens).
        output_tiers: Vec<PriceTier>,
    },

    /// Price charged per token with flat rate (no tiers).
    PerIoFlat { input_price: f64, output_price: f64 },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// A single pricing tier for a token range.
///
/// - `max_tokens`: inclusive upper bound for this tier. `None` means no upper bound.
/// - `price_per_million`: price in USD per 1_000_000 tokens for this tier.
pub struct PriceTier {
    max_tokens: Option<u64>,
    price_per_million: f64,
}

impl Model {
    /// fill all the model available with their different properties
    pub fn fill() -> Vec<Model> {
        vec![
            Model {
                name: "gpt-4o".to_string(),
                url: Some("https://replicate.com/openai/gpt-4o".to_string()),
                provider: Provider::Replicate,
                thinking_level_property: None,
                thinking_levels_authorized: None,
                characteristic: Some(Characteristic { size: None, parameter_count: None, context_window: Some(128_000), architecture: None, max_output_length: Some(16_384) }),
                price: Price::PerIoFlat { input_price: 2.50, output_price: 10.00 },
                organization: Some("Open AI".to_string()),
                licence: "Proprietary".to_string(),
                capability: Some(vec!["generalist".to_string()]),
                input_modality: Some(vec!["text".to_string(), "image".to_string()]),
                output_modality: Some(vec!["text".to_string()]),
                description: Some("Versatile multi-modal assistant tailored for complex reasoning, long-context synthesis, and structured instruction execution.".to_string()),
                apiurl: "https://api.replicate.com/v1/models/openai/gpt-4o/predictions".to_string(),
                image_parameters: Some("image_input".to_string()),
                roles_authorized: Some(vec!["user".to_string(), "assistant".to_string(), "developer".to_string(), "system".to_string()]),
            },
            Model {
                name: "gpt-4o-mini".to_string(),
                url: Some("https://replicate.com/openai/gpt-4o-mini".to_string()),
                provider: Provider::Replicate,
                thinking_level_property: None,
                thinking_levels_authorized: None,
               characteristic: Some(Characteristic { size: None, parameter_count: None, context_window: Some(128_000), architecture: None, max_output_length: Some(16_384) }),
                price: Price::PerIoFlat { input_price: 2.50, output_price: 10.00 },
                organization: Some("Open AI".to_string()),
                licence: "Proprietary".to_string(),
                capability: Some(vec!["generalist".to_string()]),
                input_modality: Some(vec!["text".to_string(), "image".to_string()]),
                output_modality: Some(vec!["text".to_string()]),
                description: Some("Lightweight, latency-focused variant of GPT‑4o optimized for fast interactive edits, short-form coding tasks, and cost-sensitive deployments.".to_string()),
                apiurl: "https://api.replicate.com/v1/models/openai/gpt-4o-mini/predictions".to_string(),
                image_parameters: Some("image_input".to_string()),
                roles_authorized: Some(vec!["user".to_string(), "assistant".to_string(), "developer".to_string(), "system".to_string()]),
            },
            Model {
                name: "claude-4-sonnet".to_string(),
                url: Some("https://replicate.com/anthropic/claude-4-sonnet".to_string()),
                provider: Provider::Replicate,
                thinking_level_property: Some("extended_thinking".to_string()),
                thinking_levels_authorized: Some(vec!["false".to_string(), "true".to_string()]),
                characteristic: Some(Characteristic { size: None, parameter_count: None, context_window: Some(200_000), architecture: None, max_output_length: Some(64_000) }),
                price: Price::PerIoFlat { input_price: 3.0, output_price: 15.0 },
                organization: Some("Anthropic".to_string()),
                licence: "Proprietary".to_string(),
                capability: Some(vec!["generalist".to_string()]),
                input_modality: Some(vec!["text".to_string(), "image".to_string()]),
                output_modality: Some(vec!["text".to_string()]),
                description: Some("Anthropic's Sonnet: a careful conversationalist excelling at iterative refinement, summarization, and safety-conscious dialogue.".to_string()),
                apiurl: "https://api.replicate.com/v1/models/anthropic/claude-4-sonnet/predictions".to_string(),
                image_parameters: Some("image".to_string()),
                roles_authorized: Some(vec!["user".to_string(), "assistant".to_string()]),
            },
            Model {
                name: "gpt-5-codex".to_string(),
                url: Some("https://openrouter.ai/openai/gpt-5-codex".to_string()),
                provider: Provider::OpenRouter,
                thinking_level_property: Some("effort".to_string()),
                thinking_levels_authorized: Some(vec!["low".to_string(), "medium".to_string(), "high".to_string()]),
                characteristic: Some(Characteristic {
                    size: None,
                    parameter_count: None,
                    context_window: Some(400_000),
                    architecture: None,
                    max_output_length: Some(128_000),
                }),
                price: Price::PerIoFlat { input_price: 1.25, output_price: 10.0 },
                organization: Some("Open AI".to_string()),
                licence: "Proprietary".to_string(),
                capability: Some(vec!["generalist".to_string()]),
                input_modality: Some(vec!["text".to_string(), "image".to_string()]),
                output_modality: Some(vec!["text".to_string()]),
                description: Some("High-throughput GPT‑5 variant engineered for code generation, multi-step algorithm design, and complex reasoning pipelines.".to_string()),
                apiurl: "openai/gpt-5-codex".to_string(),
                image_parameters: None,
                roles_authorized: Some(vec!["user".to_string(), "system".to_string(), "assistant".to_string()]),
            },
            Model {
                name: "grok-4".to_string(),
                url: Some("https://openrouter.ai/x-ai/grok-4".to_string()),
                provider: Provider::OpenRouter,
                thinking_level_property: Some("effort".to_string()),
                thinking_levels_authorized: Some(vec!["low".to_string(), "medium".to_string(), "high".to_string()]),
                characteristic: Some(Characteristic {
                    size: None,
                    parameter_count: None,
                    context_window: Some(256_000),
                    architecture: None,
                    max_output_length: Some(256_000),
                }),
                price: Price::PerIoWithTiers { input_tiers: vec![PriceTier { max_tokens: Some(128_000), price_per_million: 3.0 }, PriceTier { max_tokens: Some(256_000), price_per_million: 6.0 }], output_tiers: vec![PriceTier { max_tokens: Some(128_000), price_per_million: 15.0 }, PriceTier { max_tokens: Some(256_000), price_per_million: 30.0 }] },
                organization: Some("xAI".to_string()),
                licence: "Proprietary".to_string(),
                capability: Some(vec!["generalist".to_string()]),
                input_modality: Some(vec!["text".to_string(), "image".to_string()]),
                output_modality: Some(vec!["text".to_string()]),
                description: Some("xAI's Grok 4: pragmatic reasoning engine optimized for developer workflows, factual recall, and real-world problem solving.".to_string()),
                apiurl: "x-ai/grok-4".to_string(),
                image_parameters: None,
                roles_authorized: Some(vec!["user".to_string(), "system".to_string(), "assistant".to_string()]),
            },
            /*Model {
                name: "GPT 5 pro".to_string(),
                url: Some("https://openrouter.ai/openai/gpt-5-pro".to_string()),
                provider: Provider::OpenRouter,
                thinking_level_property: Some("effort".to_string()),
                thinking_levels_authorized: Some(vec!["low".to_string(), "medium".to_string(), "high".to_string()]),
                characteristic: Some(Characteristic {
                    size: None,
                    parameter_count: None,
                    context_window: Some(200_000),
                    architecture: Some("transformer-decoder".to_string()),
                    max_output_length: Some(60000),
                }),
                price: Price::PerRun { run_price: 0.45 },
                organization: Some("openai".to_string()),
                licence: "Proprietary".to_string(),
                capability: Some(vec!["reasoning".to_string(), "multi-modal".to_string()]),
                modality: Some(vec!["text".to_string(), "image".to_string()]),
                description: Some("Pro-tier GPT-5 variant balancing throughput and high reasoning capacity.".to_string()),
                apiurl: "openai/gpt-5-pro".to_string()
            } ,*/
            Model {
                name: "claude-4.5-sonnet".to_string(),
                url: Some("https://openrouter.ai/anthropic/claude-sonnet-4.5".to_string()),
                provider: Provider::OpenRouter,
                thinking_level_property: Some("extended_thinking".to_string()),
                thinking_levels_authorized: Some(vec!["false".to_string(), "true".to_string()]),
                characteristic: Some(Characteristic {
                    size: None,  // unknown
                    parameter_count: None,  // unknown
                    context_window: Some(1_000_000),  // likely same as Sonnet 4 family :contentReference[oaicite:0]{index=0}
                    max_output_length: Some(64_000), // consistent with long-form generation regime :contentReference[oaicite:1]{index=1}
                    architecture: None,
                }),
                price: Price::PerIoFlat {
                    input_price: 3.00,   // $3 per million tokens → 0.003 per 1,000 tokens
                    output_price: 15.00,  // $15 per million tokens → 0.015 per 1,000 tokens
                },
                organization: Some("Anthropic".to_string()),
                licence: "Proprietary".to_string(),
                capability: Some(vec!["generalist".to_string()]),
                input_modality: Some(vec!["text".to_string(), "image".to_string()]),
                output_modality: Some(vec!["text".to_string()]),
                description: Some("Claude Sonnet 4.5 — long-context specialist focused on sustained reasoning, autonomous task orchestration, and alignment-aware responses.".to_string()),
                apiurl: "anthropic/claude-sonnet-4.5".to_string(),
                image_parameters: None,
                roles_authorized: Some(vec!["user".to_string(), "system".to_string(), "assistant".to_string()]),
            },
            Model {
                name: "grok-4-fast".to_string(),
                url: Some("https://openrouter.ai/x-ai/grok-4-fast".to_string()),
                provider: Provider::OpenRouter,
                thinking_level_property: Some("effort".to_string()),
                thinking_levels_authorized: Some(vec!["low".to_string(), "medium".to_string(), "high".to_string()]),
                characteristic: Some(Characteristic {
                    size: None,
                    parameter_count: None,
                    context_window: Some(2_000_000),
                    architecture: None,
                    max_output_length: Some(30_000),
                }),
                price: Price::PerIoWithTiers { input_tiers: vec![PriceTier { max_tokens: Some(128_000), price_per_million: 0.2 }, PriceTier { max_tokens: Some(256_000), price_per_million: 0.4 }], output_tiers: vec![PriceTier { max_tokens: Some(128_000), price_per_million: 0.5 }, PriceTier { max_tokens: Some(256_000), price_per_million: 1.0 }] },
                organization: Some("xAI".to_string()),
                licence: "Proprietary".to_string(),
                capability: Some(vec!["generalist".to_string()]),
                input_modality: Some(vec!["text".to_string(), "image".to_string()]),
                output_modality: Some(vec!["text".to_string()]),
                description: Some("Grok 4 Fast: ultra-low-latency flavor tuned for rapid interactive sessions, command-line workflows, and concise reasoning.".to_string()),
                apiurl: "x-ai/grok-4-fast".to_string(),
                image_parameters: None,
                roles_authorized: Some(vec!["user".to_string(), "system".to_string(), "assistant".to_string()]),
            },
            Model {
                name: "gemini-3-pro".to_string(),
                url: Some("https://openrouter.ai/google/gemini-3-pro-preview".to_string()),
                provider: Provider::OpenRouter,
                thinking_level_property: Some("effort".to_string()),
                thinking_levels_authorized: Some(vec!["low".to_string(),"high".to_string()]),
                characteristic: Some(Characteristic {
                    size: None,
                    parameter_count: None,
                    context_window: Some(1_048_576),
                    architecture: None,
                    max_output_length: Some(65_536),
                }),
                price: Price::PerIoWithTiers { input_tiers: vec![PriceTier { max_tokens: Some(200_000), price_per_million: 2.0 }, PriceTier { max_tokens: Some(1_048_576), price_per_million: 4.0 }], output_tiers: vec![PriceTier { max_tokens: Some(200_000), price_per_million: 12.0 }, PriceTier { max_tokens: Some(1_048_576), price_per_million: 18.0 }], },
                organization: Some("Google".to_string()),
                licence: "Proprietary".to_string(),
                capability: Some(vec!["generalist".to_string()]),
                input_modality: Some(vec!["text".to_string(), "image".to_string(), "audio".to_string(), "video".to_string(), "PDF".to_string()]),
                output_modality: Some(vec!["text".to_string()]),
                description: Some("Gemini 3 Pro: a generalist model that excels at a wide range of tasks, from coding to creative writing, and is optimized for speed and efficiency.".to_string()),
                apiurl: "google/gemini-3-pro-preview".to_string(),
                image_parameters: None,
                roles_authorized: Some(vec!["user".to_string(), "system".to_string(), "assistant".to_string()]),
            },


        ]
    }

    /// getter for the thinking level property of a model
    pub fn get_thinking_level_property(&self) -> Option<&str> {
        self.thinking_level_property.as_deref()
    }

    /// getter for the authorized thinking levels of a model
    pub fn get_thinking_levels_authorized(&self) -> Option<&Vec<String>> {
        self.thinking_levels_authorized.as_ref()
    }

    /// getter for the provider name of a model
    pub fn get_provider(&self) -> &Provider {
        &self.provider
    }

    /// getter for the apiurl of a model
    pub fn get_apiurl(&self) -> &str {
        &self.apiurl
    }

    /// getter for the image parameters of a model
    pub fn get_image_parameters(&self) -> Option<&str> {
        self.image_parameters.as_deref()
    }

    /// getter for the role authorized of a model
    pub fn get_roles_authorized(&self) -> Option<&Vec<String>> {
        self.roles_authorized.as_ref()
    }

    /// getter for the name of a model
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl Price {
    /// Return the input price per million tokens for a given token count, if applicable.
    ///
    /// For `PerIoWithTiers` this selects the first matching tier where `max_tokens` is
    /// `None` or `tokens <= max_tokens`. For `PerIoFlat` it returns the flat input price.
    /// For `PerRun` it returns `None`.
    pub fn input_price_per_million(&self, tokens: u64) -> Option<&f64> {
        match self {
            Price::PerIoWithTiers { input_tiers, .. } => Self::find_tier_price(input_tiers, tokens),
            Price::PerIoFlat { input_price, .. } => Some(input_price),
            Price::PerRun { .. } => None,
        }
    }

    /// Return the output price per million tokens for a given token count, if applicable.
    ///
    /// For `PerIoWithTiers` this selects the first matching tier where `max_tokens` is
    /// `None` or `tokens <= max_tokens`. For `PerIoFlat` it returns the flat output price.
    /// For `PerRun` it returns `None`.
    pub fn output_price_per_million(&self, tokens: u64) -> Option<&f64> {
        match self {
            Price::PerIoWithTiers { output_tiers, .. } => {
                Self::find_tier_price(output_tiers, tokens)
            }
            Price::PerIoFlat { output_price, .. } => Some(output_price),
            Price::PerRun { .. } => None,
        }
    }

    /// If this is a `PerRun` price, return the run price.
    ///
    /// Returns `Some(run_price)` for `PerRun`, otherwise `None`.
    pub fn run_price(&self) -> Option<&f64> {
        match self {
            Price::PerRun { run_price } => Some(run_price),
            _ => None,
        }
    }

    /// Find the price for `tokens` in the provided ordered `tiers`.
    ///
    /// The function returns the first tier where `max_tokens` is `None` or `tokens <= max_tokens`.
    fn find_tier_price(tiers: &[PriceTier], tokens: u64) -> Option<&f64> {
        for tier in tiers {
            match tier.max_tokens {
                Some(max) if tokens <= max => return Some(&tier.price_per_million),
                Some(_) => continue,
                None => return Some(&tier.price_per_million),
            }
        }
        None
    }
}
