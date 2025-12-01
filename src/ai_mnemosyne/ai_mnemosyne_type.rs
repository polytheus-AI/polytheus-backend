#[derive(Debug, Serialize, Deserialize)]
/// Root container for the AI Mnemosyne catalog.
//
/// Contains lists of known models, organizations, licences, benchmarks,
/// and index arrays used for faceted selection.
pub struct AIMnemosyne {
    /// All known AI models in the catalog.
    models: Vec<Model>,

    /// Registered organizations providing models.
    organizations: Vec<Organization>,

    /// License referenced by models.
    licences: Vec<Licence>,

    /// Benchmarks and leaderboards used to compare models.
    benchmarks: Vec<Benchmark>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum ImageParameterType {
    String,
    VecString,
}

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

    /// the type of the image parameter
    image_parameter_type: Option<ImageParameterType>,
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

#[derive(Debug, Serialize, Deserialize)]
/// Information about an organization providing models (not about organization that make benchmarks etc...).
pub struct Organization {
    /// Organization display name.
    name: String,

    /// Optional URL for the organization's site.
    #[serde(rename = "URL")]
    url: Option<String>,
}

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

#[derive(Debug, Serialize, Deserialize)]
/// Benchmark definition containing rankings and domain tags.
pub struct Benchmark {
    /// Human readable benchmark name.
    name: String,

    /// Optional long-form description of the benchmark.
    description: Option<String>,

    /// Ranking of model by name and their score (best -> worst).
    ranking: Vec<ModelBenchmarkScore>,

    /// Domains covered by this benchmark (e.g. ["nlp", "qa"]).
    domain: Vec<String>,

    /// Quality scores associated with the benchmark.
    quality: u8,

    /// URL to the leaderboard.
    leaderboard_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// the struct that define how a model are ranking and scores in a benchmark
pub struct ModelBenchmarkScore {
    /// model name
    model_name: String,

    /// thinking level (must be one of the authorized thinking levels for the model)
    thinking_level: Option<String>,

    /// score of the model in the benchmark
    score: f32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
/// Enumeration of known model providers.
pub enum Provider {
    Replicate,
    OpenRouter,
}
