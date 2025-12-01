use crate::polytheus::Polytheus;
use polars::prelude::*;
use serde::{Deserialize, Serialize};

// Include type definitions
include!("ai_mnemosyne_type.rs");

// Include constructor implementations
include!("ai_mnemosyne.rs");

#[derive(Deserialize, Debug)]
pub struct PredictionUrls {
    pub stream: String,
    // Other fields like get, cancel, etc. might also be present
}

#[derive(Deserialize, Debug)]
pub struct PredictionResponse {
    pub urls: PredictionUrls,
    // Other fields like id, model, created_at, etc.
}

impl AIMnemosyne {
    /// getter for the model by its name
    pub fn get_model_by_name(&self, model_name: &str) -> Option<&Model> {
        self.models.iter().find(|model| model.name == model_name)
    }

    /// getter for the benchmark by its name
    pub fn get_benchmark_by_name(&self, benchmark_name: &str) -> Option<&Benchmark> {
        self.benchmarks
            .iter()
            .find(|benchmark| benchmark.name == benchmark_name)
    }
}

impl Model {
    /// getter for the thinking level property of a model by its name
    pub fn get_thinking_level_property(&self) -> Option<&str> {
        self.thinking_level_property.as_deref()
    }

    /// getter for the authorized thinking levels of a model by its name
    pub fn get_thinking_levels_authorized(&self) -> Option<&Vec<String>> {
        self.thinking_levels_authorized.as_ref()
    }

    /// getter for the provider name of a model by its name
    pub fn get_provider(&self) -> &Provider {
        &self.provider
    }

    /// getter for the apiurl of a model by its name
    pub fn get_apiurl(&self) -> &str {
        &self.apiurl
    }

    /// getter for the image parameters of a model by its name
    pub fn get_image_parameters(&self) -> Option<&str> {
        self.image_parameters.as_deref()
    }

    /// getter for the image parameter type of a model by its name
    pub fn get_image_parameter_type(&self) -> Option<&ImageParameterType> {
        self.image_parameter_type.as_ref()
    }
}
impl Price {
    /// Return the input price per million tokens for a given token count, if applicable.
    ///
    /// For `PerIoWithTiers` this selects the first matching tier where `max_tokens` is
    /// `None` or `tokens <= max_tokens`. For `PerIoFlat` it returns the flat input price.
    /// For `PerRun` it returns `None`.
    pub fn input_price_per_million(&self, tokens: u64) -> Option<f64> {
        match self {
            Price::PerIoWithTiers { input_tiers, .. } => Self::find_tier_price(input_tiers, tokens),
            Price::PerIoFlat { input_price, .. } => Some(*input_price),
            Price::PerRun { .. } => None,
        }
    }

    /// Return the output price per million tokens for a given token count, if applicable.
    ///
    /// For `PerIoWithTiers` this selects the first matching tier where `max_tokens` is
    /// `None` or `tokens <= max_tokens`. For `PerIoFlat` it returns the flat output price.
    /// For `PerRun` it returns `None`.
    pub fn output_price_per_million(&self, tokens: u64) -> Option<f64> {
        match self {
            Price::PerIoWithTiers { output_tiers, .. } => {
                Self::find_tier_price(output_tiers, tokens)
            }
            Price::PerIoFlat { output_price, .. } => Some(*output_price),
            Price::PerRun { .. } => None,
        }
    }

    /// If this is a `PerRun` price, return the run price.
    ///
    /// Returns `Some(run_price)` for `PerRun`, otherwise `None`.
    pub fn run_price(&self) -> Option<f64> {
        match self {
            Price::PerRun { run_price } => Some(*run_price),
            _ => None,
        }
    }

    /// Find the price for `tokens` in the provided ordered `tiers`.
    ///
    /// The function returns the first tier where `max_tokens` is `None` or `tokens <= max_tokens`.
    fn find_tier_price(tiers: &[PriceTier], tokens: u64) -> Option<f64> {
        for tier in tiers {
            match tier.max_tokens {
                Some(max) if tokens <= max => return Some(tier.price_per_million),
                Some(_) => continue,
                None => return Some(tier.price_per_million),
            }
        }
        None
    }
}

impl Benchmark {
    /// get the name of the benchmark
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    /// Test tier selection for PerIoWithTiers pricing (matches the example table).
    #[test]
    fn test_perio_tier_selection() {
        let start = Instant::now();

        let price = Price::PerIoWithTiers {
            input_tiers: vec![
                PriceTier {
                    max_tokens: Some(128_000),
                    price_per_million: 3.0,
                },
                PriceTier {
                    max_tokens: None,
                    price_per_million: 6.0,
                },
            ],
            output_tiers: vec![
                PriceTier {
                    max_tokens: Some(128_000),
                    price_per_million: 15.0,
                },
                PriceTier {
                    max_tokens: None,
                    price_per_million: 30.0,
                },
            ],
        };

        assert_eq!(price.input_price_per_million(100_000), Some(3.0));
        assert_eq!(price.input_price_per_million(128_000), Some(3.0));
        assert_eq!(price.input_price_per_million(200_000), Some(6.0));

        assert_eq!(price.output_price_per_million(10_000), Some(15.0));
        assert_eq!(price.output_price_per_million(128_001), Some(30.0));

        let duration = Instant::now() - start;
        eprintln!("test_perio_tier_selection took: {:?}", duration);
    }

    /// Test PerRun and PerIoFlat behavior alongside helpers.
    #[test]
    fn test_perrun_and_helpers() {
        let start = Instant::now();

        let run = Price::PerRun { run_price: 0.50 };
        assert_eq!(run.run_price(), Some(0.50));
        assert_eq!(run.input_price_per_million(10_000), None);
        assert_eq!(run.output_price_per_million(10_000), None);

        let flat = Price::PerIoFlat {
            input_price: 5.0,
            output_price: 20.0,
        };
        assert_eq!(flat.input_price_per_million(1), Some(5.0));
        assert_eq!(flat.output_price_per_million(1), Some(20.0));

        let duration = Instant::now() - start;
        eprintln!("test_perrun_and_helpers took: {:?}", duration);
    }
}
