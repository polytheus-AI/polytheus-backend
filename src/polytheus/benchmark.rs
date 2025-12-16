use serde::{Deserialize, Serialize};

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

impl Benchmark {
    /// fill all the benchmarks available with their different properties
    pub fn fill() -> Vec<Benchmark> {
        vec![
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
        ]
        }

       
        /// get the name of the benchmark
        pub fn get_name(&self) -> &str {
            &self.name
        }

    }

