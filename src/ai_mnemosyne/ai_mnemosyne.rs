impl AIMnemosyne {
    /// Create a comprehensive catalog pre-populated with real AI models,
    /// organizations, licenses, and benchmarks.
    pub fn fill() -> Self {
        // Organizations (IDs are 1-based indexes into this vector)
        let organizations = vec![
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
        ];

        // Licences (IDs are 1-based)
        let licences = vec![
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
        ];

        // Models (IDs are 1-based indexes into this vector).
        // For each model the URL is set to a "replicate"-style URL derived from owner/name.
        let models = vec![
            Model {
                name: "GPT 4o".to_string(),
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
                image_parameter_type: Some(ImageParameterType::VecString),
            },
            Model {
                name: "GPT 4o mini".to_string(),
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
                image_parameter_type: Some(ImageParameterType::VecString),
            },
            Model {
                name: "Claude 4 sonnet".to_string(),
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
                image_parameter_type: Some(ImageParameterType::String),
            },
            Model {
                name: "GPT 5 codex".to_string(),
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
                image_parameter_type: Some(ImageParameterType::String),
            },
            Model {
                name: "Grok 4".to_string(),
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
                image_parameter_type: Some(ImageParameterType::String),
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
                name: "Claude Sonnet 4.5".to_string(),
                url: Some("https://openrouter.ai/anthropic/claude-sonnet-4.5".to_string()),
                provider: Provider::OpenRouter,
                thinking_level_property: Some("extended_thinking".to_string()),
                thinking_levels_authorized: Some(vec!["false".to_string(), "true".to_string()]),
                characteristic: Some(Characteristic {
                    size: None,  // unknown
                    parameter_count: None,  // unknown
                    context_window: Some(1_000_000),  // likely same as Sonnet 4 family :contentReference[oaicite:0]{index=0}
                    max_output_length: Some(64_000),  // consistent with long-form generation regime :contentReference[oaicite:1]{index=1}
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
                image_parameter_type: Some(ImageParameterType::String),
            },
            Model {
                name: "Grok 4 Fast".to_string(),
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
                image_parameter_type: Some(ImageParameterType::String),
            },
            Model {
                name: "Gemini 3 Pro".to_string(),
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
                image_parameter_type: Some(ImageParameterType::String),
            },


        ];

        // Benchmarks – each ranking contains the top-10 model IDs (1-based indices into `models`).
        let benchmarks = vec![
            Benchmark {
                name: "LiveBench-Coding".to_string(),
                description: Some("A continuously-updated leaderboard that evaluates models on live/realistic tasks and integration performance. (live-bench coding evaluate model in coding tasks)".to_string()),
                ranking: vec![ModelBenchmarkScore{ model_name: "Claude 4 sonnet".to_string(), thinking_level: None, score: 80.74 }, ModelBenchmarkScore{ model_name: "Claude 4.5 sonnet".to_string(), thinking_level: Some("thinking".to_string()), score: 80.36 }, ModelBenchmarkScore{ model_name: "gpt-5".to_string(), thinking_level: None, score: 78.57 }, ModelBenchmarkScore{ model_name: "claude-4-sonnet".to_string(), thinking_level: Some("Thinking".to_string()), score: 77.48 }, ModelBenchmarkScore{ model_name: "gpt-5".to_string(), thinking_level: Some("High".to_string()), score: 77.10 }],
                domain: vec!["coding".to_string(), "programming".to_string()],
                quality: 7,
                leaderboard_url: "https://livebench.ai/#/".to_string(),
                
            },
            Benchmark {
                name: "LiveBench-Reasoning".to_string(),
                description: Some("Leaderboard focused on pure reasoning capability averaged across diverse reasoning tasks.".to_string()),
                ranking: vec![ModelBenchmarkScore { model_name: "gpt-5-codex".to_string(), thinking_level: None, score: 98.67 },ModelBenchmarkScore { model_name: "gpt-5".to_string(), thinking_level: Some("high".to_string()), score: 98.17  }, ModelBenchmarkScore { model_name: "grok-4".to_string(), thinking_level: None, score: 97.78 }, ModelBenchmarkScore { model_name: "gpt-5".to_string(), thinking_level: Some("medium".to_string()), score: 96.58 }, ModelBenchmarkScore { model_name: "Grok 4 Fast".to_string(), thinking_level: None, score: 95.44 }],
                domain: vec!["reasoning".to_string(), "nlp".to_string()],
                quality: 9,
                leaderboard_url: "https://livebench.ai/#/".to_string(),
            },
            Benchmark {
                name: "Humanity's Last Exam".to_string(),
                description: Some("Humanity’s Last Exam is a very difficult, expert-level benchmark for language models, composed of around 3,000 questions across many academic disciplines, designed to test reasoning and knowledge beyond what current AI models reliably handle.".to_string()),
                ranking: vec![ModelBenchmarkScore{ model_name: "Gemini 3 Pro".to_string(), thinking_level: None, score: 37.52 }, ModelBenchmarkScore{model_name: "GPT 5 Pro".to_string(), thinking_level: None, score: 31.64}, ModelBenchmarkScore{model_name: "GPT 5".to_string(), thinking_level: None, score: 25.32}, ModelBenchmarkScore{model_name: "Gemini 2.5 Pro".to_string(), thinking_level: None, score: 21.64}, ModelBenchmarkScore{model_name: "o3".to_string(), thinking_level: None, score: 20.32}],
                domain: vec!["reasoning".to_string(), "knowledge".to_string()],
                quality: 9,
                leaderboard_url: "https://scale.com/leaderboard/humanitys_last_exam".to_string(),
               
            }
            /*Benchmark {
                name: "aider polyglot".to_string(),
                description: Some("Multilingual evaluation focusing on instruction following and translation quality across many languages.".to_string()),
                ranking: vec!["bloomz-176b".to_string(), "claude-3-opus".to_string(), "gemini-pro".to_string(), "gpt-4o".to_string(), "llama-3-70b-chat".to_string(), "llama-2-70b".to_string(), "aleph-alpha-luminous".to_string(), "stability-a2".to_string(), "gpt-4o-mini".to_string(), "mpt-30b".to_string()],
                domain: vec!["multilingual".to_string(), "nlp".to_string()],
                leaderboard_url: "https://aider.chat/docs/leaderboards/".to_string(),
            },
            Benchmark {
                name: "SWE bash".to_string(),
                description: Some("Bash and shell scripting benchmark aimed at evaluating models on software-engineering terminal tasks.".to_string()),
                ranking: vec!["gpt-4o".to_string(), "gpt-4o-mini".to_string(), "claude-3-opus".to_string(), "llama-3-70b-chat".to_string(), "mpt-30b".to_string(), "falcon-180b".to_string(), "stability-a2".to_string(), "llama-2-70b".to_string(), "mistral-7b".to_string(), "vicuna-13b".to_string()],
                domain: vec!["code".to_string(), "shell".to_string(), "scripting".to_string()],
                leaderboard_url: "https://www.swebench.com/bash-only.html".to_string(),
            },
            Benchmark {
                name: "terminal-bench".to_string(),
                description: Some("Terminal-centric benchmark evaluating models' abilities to generate, debug and explain terminal commands.".to_string()),
                ranking: vec!["gpt-4o".to_string(), "gpt-4o-mini".to_string(), "gemini-pro".to_string(), "claude-3-opus".to_string(), "llama-3-70b-chat".to_string(), "mpt-30b".to_string(), "falcon-180b".to_string(), "stability-a2".to_string(), "mistral-7b".to_string(), "bloomz-176b".to_string()],
                domain: vec!["terminal".to_string(), "automation".to_string()],
                leaderboard_url: "https://www.tbench.ai/".to_string(),
            },
            Benchmark {
                name: "WebDev Arena".to_string(),
                description: Some("Web development specific leaderboard — HTML/CSS/JS generation, debugging and architecture suggestions.".to_string()),
                ranking: vec!["gemini-pro".to_string(), "gpt-4o".to_string(), "gpt-4o-mini".to_string(), "mpt-30b".to_string(), "claude-3-opus".to_string(), "llama-3-70b-chat".to_string(), "stability-a2".to_string(), "llama-2-70b".to_string(), "falcon-180b".to_string(), "bloomz-176b".to_string()],
                domain: vec!["webdev".to_string(), "code".to_string(), "frontend".to_string()],
                leaderboard_url: "https://web.lmarena.ai/leaderboard".to_string(),
            },
            Benchmark {
                name: "GSO".to_string(),
                description: Some("General scientific and optimisation benchmark (GSO) focusing on long-form reasoning and math).".to_string()),
                ranking: vec!["gpt-4o".to_string(), "gemini-pro".to_string(), "claude-3-opus".to_string(), "llama-3-70b-chat".to_string(), "stability-a2".to_string(), "llama-2-70b".to_string(), "mpt-30b".to_string(), "falcon-180b".to_string(), "gpt-4o-mini".to_string(), "bloomz-176b".to_string()],
                domain: vec!["reasoning".to_string(), "math".to_string(), "science".to_string()],
                leaderboard_url: "https://gso-bench.github.io/leaderboard.html".to_string(),
            },
            Benchmark {
                name: "Artificial Analysis Long Context Reasoning".to_string(),
                description: Some("Long-context reasoning and document-understanding benchmark from Artificial Analysis.".to_string()),
                ranking: vec!["gemini-pro".to_string(), "gpt-4o".to_string(), "claude-3-opus".to_string(), "llama-3-70b-chat".to_string(), "gpt-4o-mini".to_string(), "mpt-30b".to_string(), "llama-2-70b".to_string(), "stability-a2".to_string(), "mistral-7b".to_string(), "bloomz-176b".to_string()],
                domain: vec!["long-context".to_string(), "reasoning".to_string()],
                leaderboard_url: "https://artificialanalysis.ai/evaluations/artificial-analysis-long-context-reasoning".to_string(),
            },
            Benchmark {
                name: "τ-bench".to_string(),
                description: Some("τ-bench — a research-oriented benchmark emphasizing robustness and adversarial reasoning.".to_string()),
                ranking: vec!["gpt-4o".to_string(), "claude-3-opus".to_string(), "gemini-pro".to_string(), "llama-3-70b-chat".to_string(), "gpt-4o-mini".to_string(), "claude-2.1".to_string(), "mpt-30b".to_string(), "falcon-180b".to_string(), "llama-2-70b".to_string(), "bloomz-176b".to_string()],
                domain: vec!["robustness".to_string(), "adversarial".to_string(), "nlp".to_string()],
                leaderboard_url: "https://taubench.com/#leaderboard".to_string(),
            },
            Benchmark {
                name: "FrontierMath".to_string(),
                description: Some("FrontierMath — a leaderboard for cutting-edge mathematical problem solving and proof generation.".to_string()),
                ranking: vec!["gpt-4o".to_string(), "gemini-pro".to_string(), "claude-3-opus".to_string(), "llama-3-70b-chat".to_string(), "stability-a2".to_string(), "llama-2-70b".to_string(), "mpt-30b".to_string(), "falcon-180b".to_string(), "gpt-4o-mini".to_string(), "bloomz-176b".to_string()],
                domain: vec!["math".to_string(), "theorem-proving".to_string(), "reasoning".to_string()],
                leaderboard_url: "https://epoch.ai/frontiermath".to_string(),
            },*/
        ];

        AIMnemosyne {
            models,
            organizations,
            licences,
            benchmarks,
        }
    }
}
