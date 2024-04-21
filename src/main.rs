use aichat_cli::{
    chat_message::{self, Role},
    claude_client,
    model::{Campany, Model},
    openai_client,
};
use anyhow::Result;
use dotenv::dotenv;
use requestty::Question;
use std::env;

fn main() {
    // コマンドライン引数を取得
    let args: Vec<String> = env::args().collect();

    // -t オプションが指定されているかどうかを確認
    let is_translation_mode = args.contains(&"-t".to_string());

    match run(is_translation_mode) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    };
}

fn run(is_translation_mode: bool) -> Result<()> {
    // 必要な環境変数をここで確認
    dotenv().ok();
    let openai_token =
        env::var("OPENAI_API_KEY").expect("環境変数にOPENAI_API_KEYをセットしてください");
    let anthropic_token =
        env::var("ANTHROPIC_API_KEY").expect("環境変数にANTHROPIC_API_KEYをセットしてください");

    if is_translation_mode {
        println!("📖 翻訳モードで起動します");
    }

    let mut gpt_client = openai_client::ChatGPTClient::new(openai_token);
    let mut claude_client = claude_client::ClaudeClient::new(anthropic_token);

    // ユーザーにモデルを選択させる
    let selected_model = select_model_input(&claude_client, &gpt_client)?;

    // 初期メッセージを追加
    let mut messages = chat_message::MessageHistory::default();

    match selected_model.campany {
        Campany::Claude => {
            claude_client.set_model(selected_model.name.to_owned());

            if is_translation_mode {
                let initial_message = "system: このユーザーは翻訳を求めています。与えられた文章が日本語なら英訳を出力し、英語なら日本語訳を出力してください";
                messages.push(Role::User, initial_message);
                messages.push(
                    Role::Assistant,
                    "はい、承知しました。ここからは翻訳モードで応対します。",
                );
            }
            claude_client.run_claude(messages)?;
        }
        Campany::OpenAI => {
            gpt_client.set_model(selected_model.name.to_owned());

            if is_translation_mode {
                let initial_message = "このユーザーは翻訳を求めています。与えられた文章が日本語なら英訳を出力し、英語なら日本語訳を出力してください";
                messages.push(Role::System, initial_message);
            }

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
