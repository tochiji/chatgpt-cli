use chatgpt_cli::{
    chat_input,
    chat_message::{self, Role},
    chatgpt_client,
};
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
    let openai_token = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

    let mut client = chatgpt_client::ChatGPTClient::new(openai_token);
    client.select_model()?;
    let mut messages = chat_message::MessageHistory::default();

    // ChatGPTの初期設定を追加
    let system_content = "あなたは親切なアシスタントです。あなたは非常に聡明で、抽象的な説明と具体的な例示が得意です。";
    messages.push(Role::System, system_content);

    // ユーザーからの質問を無限ループで受け付ける
    loop {
        // ユーザーからの入力を受け付ける
        println!("👤 質問を入力してください。（入力完了時は改行してCtrl+D）>");
        let message = chat_input::stdin_to_string()?;

        // 入力した質問を履歴に追加
        messages.push(Role::User, &message);

        println!("🤖 ChatGPTからの回答 >");

        // [TODO] エラー時、exitするのではなく、エラー内容を表示してループを継続したい
        let assistant_response = client.send_messages(&messages)?;
        messages.push(Role::Assistant, &assistant_response);

        // 次の質問との間に空行を入れる
        println!();
    }
}
