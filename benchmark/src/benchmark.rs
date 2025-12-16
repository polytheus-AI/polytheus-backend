use backend::ai_mnemosyne;
use backend::polytheus::Polytheus;
use polars::prelude::*;

pub struct BenchmarksRunner<'a> {
    benchmarks: Vec<Benchmark<'a>>,
}

impl<'a> BenchmarksRunner<'a> {
    pub fn new(ai_mnemosyne: &'a ai_mnemosyne::AIMnemosyne) -> Self {
        let benchmarks = vec![
            Benchmark {
                benchmark: ai_mnemosyne.get_benchmark_by_name("LiveBench-Coding").unwrap(),
                dataset_name: None,
                system_prompt_model: None,
                system_prompt_judge: None,
            },
            Benchmark {
                benchmark: ai_mnemosyne.get_benchmark_by_name("LiveBench-Reasoning").unwrap(),
                dataset_name: None,
                system_prompt_model: None,
                system_prompt_judge: None,
            },
            Benchmark {
                benchmark: ai_mnemosyne.get_benchmark_by_name("Humanity's Last Exam").unwrap(),
                dataset_name: Some("Humanity_Last_Exam".to_string()),
                system_prompt_model: Some("You are about to take an extremely difficult test to measure your performance in various fields such as mathematics, physics, chemistry, etc. This test includes text and images. These tests have been created by renowned scientists in the fields covered by the questions. I must emphasize that this test is extremely difficult. It is designed to test AI on questions at the human expert level to see if you can compete with these experts. So take your time before answering, think carefully, check your answer before submitting it to make sure it is correct, and don't hesitate to break the problem down into smaller parts if that helps you. Your answer should be concise, you don't need to argue your point. Another AI will check your answer and assign it a score.".to_string()),
                system_prompt_judge: Some("You are an AI evaluator. You will receive the answer to a question asked by the AI, the question itself, and the correct answer to that question. Your mission is to evaluate the quality of the AI's answer. The AI you will be evaluating is undergoing a very complicated test, and the question you will be evaluating is also very complicated. After carefully considering the question, the correct answer to that question, and the AI's answer, you are supposed to give the AI a score out of 100. Here are some scoring criteria: -0%: The AI answered the question completely incorrectly. -25% The AI answered the question incorrectly, but it managed to make a point that is still relevant to the question. -50% The AI answered the question half-correctly; it cannot be considered correct, but there is some truth in its answer. -75% The AI answered the question as mostly true, but its answer is still ambiguous. -100% The AI answered the question perfectly.".to_string()),
            },
        ];
        Self { benchmarks }
    }

    /// retrieve a benchmark by name
    pub fn retrieve_benchmark(&self, name: &str) -> Option<&Benchmark> {
        self.benchmarks.iter().find(|b| b.get_name() == name)
    }
}

pub struct Benchmark<'a> {
    benchmark: &'a ai_mnemosyne::Benchmark,

    /// Name of the dataset
    dataset_name: Option<String>,

    /// the system prompt that the model that will make the test will use.
    system_prompt_model: Option<String>,

    /// the system prompt that the model that will judge the model that will made the test will use.
    system_prompt_judge: Option<String>,
}

impl<'a> Benchmark<'a> {
    /// retrive all the benchmark questions and answers
    pub fn retrieve_benchmark(&self) -> Result<DataFrame, String> {
        let file_path = format!(
            "data/{}.parquet",
            self.dataset_name.as_ref().ok_or("No dataset name")?
        );
        let file = std::fs::File::open(&file_path)
            .map_err(|e| format!("Failed to open file {}: {}", file_path, e))?;
        ParquetReader::new(file)
            .finish()
            .map_err(|e| format!("Failed to read parquet: {}", e))
    }

    /// run the benchmark
    /// #Params
    /// * model: the model to use
    /// * model_judge: the model to use as judge
    /// * max_questions: the maximum number of questions to use
    ///
    /// #Returns
    /// * u8: the score of the benchmark in pourcentage
    pub async fn run_benchmark(
        &self,
        polytheus: &Polytheus,
        model: String,
        model_judge: String,
        max_questions: u16,
    ) -> u8 {
        let df = match self.retrieve_benchmark() {
            Ok(df) => df,
            Err(e) => {
                eprintln!("Error retrieving benchmark: {}", e);
                return 0;
            }
        };

        println!("benchmark retrieved");
        println!("df: {}", df);

        let questions_count = std::cmp::min(df.height() as u16, max_questions);
        let mut total_score = 0.0;

        for i in 0..questions_count {
            let question = match self.retrieve_question(&df, i) {
                Some(q) => q,
                None => continue,
            };
            let correct_answer = match self.retrieve_answer(&df, i) {
                Some(a) => a,
                None => continue,
            };

            // Run the model to get the answer

            let message = backend::polytheus::Message {
                thinking_level: None,
                input_text: question.clone(),
                input_image: None,
                input_audio: None,
                input_audio_format: None,
                input_video: None,
            };
            let messages = vec![message];

            // Run the model to get the answer
            let model_response = match polytheus.run(&model, messages, None).await {
                Ok(resp) => resp,
                Err(e) => {
                    eprintln!("Error running model {}: {}", model, e);
                    continue;
                }
            };

            // Construct the judge prompt
            let judge_system_prompt = self
                .system_prompt_judge
                .as_deref()
                .unwrap_or("You are an AI evaluator.");
            let judge_input = format!(
                "{}\n\nQuestion: {}\nCorrect Answer: {}\nAI Answer: {}\n\nEvaluate the AI answer and give a score from 0 to 100.",
                judge_system_prompt, question, correct_answer, model_response
            );

            println!("Judge input: {}", judge_input);

            // Run the judge model

            let message = backend::polytheus::Message {
                thinking_level: None,
                input_text: judge_input,
                input_image: None,
                input_audio: None,
                input_audio_format: None,
                input_video: None,
            };
            let messages = vec![message];

            // Run the judge model
            let judge_response = match polytheus.run_llm(&model_judge, messages, None).await {
                Ok(resp) => resp,
                Err(e) => {
                    eprintln!("Error running judge model {}: {}", model_judge, e);
                    continue;
                }
            };

            // Parse the score from the judge's response
            // This is a naive parsing; in production, you'd want more robust extraction
            let score = self.extract_score(&judge_response);
            total_score += score;
        }

        if questions_count > 0 {
            (total_score / questions_count as f32) as u8
        } else {
            0
        }
    }

    /// retrieve a question of the benchmark
    pub fn retrieve_question(&self, df: &DataFrame, num_question: u16) -> Option<String> {
        let idx = num_question as usize;
        if idx >= df.height() {
            return None;
        }
        // Assuming column name is "question"
        df.column("question")
            .ok()
            .and_then(|col| col.get(idx).ok())
            .map(|any_val| any_val.to_string())
    }

    /// retrive a answer of a question of the benchmark
    pub fn retrieve_answer(&self, df: &DataFrame, num_answer: u16) -> Option<String> {
        let idx = num_answer as usize;
        if idx >= df.height() {
            return None;
        }
        // Assuming column name is "answer"
        df.column("answer")
            .ok()
            .and_then(|col| col.get(idx).ok())
            .map(|any_val| any_val.to_string())
    }

    fn extract_score(&self, response: &str) -> f32 {
        // Look for a number between 0 and 100 in the response
        // This is a placeholder logic.
        // A simple regex or finding the last number might work better.
        // For now, let's try to find a number.
        let re = regex::Regex::new(r"\b(100|[1-9]?[0-9])\b").unwrap();
        if let Some(caps) = re.captures(response) {
            if let Some(m) = caps.get(1) {
                if let Ok(s) = m.as_str().parse::<f32>() {
                    return s;
                }
            }
        }
        0.0
    }

    /// get the name of the benchmark
    pub fn get_name(&self) -> &str {
        &self.benchmark.get_name()
    }
}
