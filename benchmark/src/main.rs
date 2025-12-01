mod benchmark;

use backend::polytheus::Polytheus;
use benchmark::Benchmark;
use dotenvy::dotenv;

use crate::benchmark::BenchmarksRunner;

#[tokio::main]

async fn main() {
    dotenv().ok();

    let polytheus = Polytheus::new();

    let benchmarks_runner = BenchmarksRunner::new(&polytheus.ai_mnemosyne);

    // Example usage of run_benchmark
    // You might want to parse command line arguments here to make it more flexible
    let benchmark_name = "Humanity_Last_Exam".to_string(); // Or whatever default or arg
    let model_name = "GPT 5 codex".to_string(); // Example
    let judge_name = "GPT 4".to_string(); // Example
    let max_questions = 10;

    let benchmark = benchmarks_runner.retrieve_benchmark(benchmark_name.as_str());

    if let Some(benchmark) = benchmark {
        let score = benchmark
            .run_benchmark(&polytheus, model_name, judge_name, max_questions)
            .await;
        println!("Benchmark finished. Score: {}%", score);
    } else {
        println!("Benchmark not found");
    }
}
