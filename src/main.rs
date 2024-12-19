use aichat_cli::{
    chat_message::{self},
    claude_client,
    model::{Campany, Model},
    openai_client,
};
use anyhow::Result;
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

fn run() -> Result<()> {
    // 必要な環境変数をここで確認
    dotenv().ok();
    let openai_token =
        env::var("OPENAI_API_KEY").expect("環境変数にOPENAI_API_KEYをセットしてください");
    let anthropic_token =
        env::var("ANTHROPIC_API_KEY").expect("環境変数にANTHROPIC_API_KEYをセットしてください");

    let mut gpt_client = openai_client::ChatGPTClient::new(openai_token);
    let mut claude_client = claude_client::ClaudeClient::new(anthropic_token);

    // ユーザーにモデルを選択させる
    let selected_model = select_model_input(&claude_client, &gpt_client)?;

    // 初期メッセージを追加
    let messages = chat_message::MessageHistory::default();

    match selected_model.campany {
        Campany::Claude => {
            claude_client.set_model(selected_model.name.to_owned());
            claude_client.run_claude(messages)?;
        }
        Campany::OpenAI => {
            gpt_client.set_model(selected_model);
            gpt_client.run_chatgpt(messages)?;
        }
    }

    Ok(())
}

/// ユーザーにモデルを選択させる
fn select_model_input(
    claude_client: &claude_client::ClaudeClient,
    gpt_client: &openai_client::ChatGPTClient,
) -> Result<Model, anyhow::Error> {
    let claude_models = claude_client.get_model_list();
    let openai_models = gpt_client.fetch_models()?;

    let models: Vec<Model> = claude_models
        .iter()
        .chain(openai_models.iter())
        .cloned()
        .collect();
    let select = Question::select("theme")
        .should_loop(false)
        .message("🤖 利用するモデルを選択してください (Ctrl+c to exit)")
        .choices(models.clone())
        .default(0)
        .build();
    let answer = requestty::prompt_one(select)?;
    let model_index = &answer.as_list_item().unwrap().index;
    let model = models[*model_index].clone();
    Ok(model.to_owned())
}
