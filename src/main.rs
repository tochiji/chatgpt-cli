use chatgpt_cli::{call_api, chat_input, chat_message};
use dotenv::dotenv;
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
    let openai_token = env::var("OPENAI_TOKEN").expect("OPENAI_TOKEN must be set");

    // APIに送るメッセージ履歴
    // ChatGPTへの初期プロンプト、ユーザーからの質問、ChatGPTからの回答が格納される。
    // この履歴を元に、ChatGPTは次の回答を生成する。
    let mut message_history: chat_message::MessageHistory = chat_message::MessageHistory::new();

    // ChatGPTの初期設定を追加
    let system_content = "あなたは親切なアシスタントです。あなたは非常に聡明で、抽象的な説明と具体的な例示が得意です。";
    message_history.push("system", system_content);

    // ユーザーからの質問を無限ループで受け付ける
    loop {
        // ユーザーからの入力を受け付ける
        println!("👤 質問を入力してください。（入力完了時は改行してCtrl+D）>");
        let message = chat_input::stdin_to_string()?;

        // 入力した質問を履歴に追加
        message_history.push("user", &message);
        println!("");

        // APIを呼び出して回答を取得し、履歴に追加
        println!("🤖 ChatGPTからの回答 >");

        // [TODO] エラー時、exitするのではなく、エラー内容を表示してループを継続したい
        let assistant_response = call_api::chatgpt(&openai_token, &message_history)?;
        message_history.push("assistant", &assistant_response);

        // 次の質問との間に空行を入れる
        println!("");
    }
}
