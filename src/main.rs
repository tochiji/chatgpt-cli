use chatgpt_cli::{claude_client, openai_client};
use dotenv::dotenv;
use requestty::Question;
use std::env;

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    };
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    // 必要な環境変数をここで確認
    dotenv().ok();
    let openai_token = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let anthropic_token = env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY must be set");

    let mut gpt_client = openai_client::ChatGPTClient::new(openai_token);

    // claudeかchatgptかをmodel一覧から選択する
    let claude_models = vec![
        "claude-3-opus-20240229",
        "claude-3-sonnet-20240229",
        "claude-3-haiku-20240307",
    ];
    let openai_models = gpt_client.fetch_models()?;

    let models: Vec<String> = claude_models
        .into_iter()
        .map(|m| m.to_string())
        .chain(openai_models)
        .collect();

    let select = Question::select("theme")
        .should_loop(false)
        .message("🤖 利用するモデルを選択してください (Ctrl+c to exit)")
        .choices(models)
        .default(0)
        .build();

    let answer = requestty::prompt_one(select)?;
    let model = &answer.as_list_item().unwrap().text;

    match model.as_str() {
        "claude-3-opus-20240229" | "claude-3-sonnet-20240229" | "claude-3-haiku-20240307" => {
            let mut claude_client = claude_client::ClaudeClient::new(anthropic_token);
            claude_client.set_model(model.to_owned());
            claude_client.run_claude()?;
        }
        _ => {
            gpt_client.set_model(model.to_owned());
            gpt_client.run_chatgpt()?;
        }
    }

    Ok(())
}
