use std::{
    io::{stdout, BufRead, BufReader, Write},
    usize::MAX,
};

use crate::{api_response::ChatCompletionChunk, chat_message};
use reqwest::blocking::Client;
use serde_json::json;

pub fn chatgpt(
    openai_token: &str,
    message_history: &chat_message::MessageHistory,
) -> Result<String, Box<dyn std::error::Error>> {
    // APIのエンドポイントURLを設定
    // https://platform.openai.com/docs/api-reference/chat/create
    let url = "https://api.openai.com/v1/chat/completions";

    // APIへのリクエストヘッダーを設定
    let headers = generate_headers(openai_token.to_string())?;

    // APIへのリクエストボディを設定
    // 「メッセージ履歴」は全て連結して送る必要がある
    let body = generate_body_from_history(message_history);

    // [TODO] リクエストの前に、トークン数を確認したい

    // APIへのリクエストを送信し、responseを受け取る。
    // 文字が順次送られてくるstream形式で受け取る。
    // https://platform.openai.com/docs/api-reference/chat/create#chat/create-stream
    let response = send_post_request(url, headers, body)?;

    // streamを逐次表示し、最後に全ての文字列を連結したものを joined_string として受け取る。
    let joined_string = print_chat_stream(response)?;

    // APIからのレスポンスの最後には一般に改行コードが含まれないため、ここで一度改行する
    println!("");

    Ok(joined_string)
}

// APIへ送信するbodyを作成する。
// メッセージ履歴は全て連結して送る必要がある。
fn generate_body_from_history(message_history: &chat_message::MessageHistory) -> serde_json::Value {
    let messages = message_history
        .messages
        .iter()
        .map(|m| json!({"role": m.role, "content": m.content}))
        .collect::<Vec<_>>();

    let body = json!({
        "top_p": 0.5,
        "stream": true, // streamとして受け取る設定
        "model": "gpt-4",
        "messages": messages,
    });
    body
}

// URLへPOSTリクエストを行う関数
fn send_post_request(
    url: &str,
    headers: reqwest::header::HeaderMap,
    body: serde_json::Value,
) -> Result<reqwest::blocking::Response, Box<dyn std::error::Error>> {
    let client = Client::new();

    // [TODO] if let Errにしてエラーレスポンスの内容を表示したい
    let response = client
        .post(url)
        .headers(headers)
        .json(&body)
        .send()?
        .error_for_status()?;
    Ok(response)
}

// APIを呼び出すのに必要なヘッダーを生成する
fn generate_headers(
    secret_token: String,
) -> Result<reqwest::header::HeaderMap, Box<dyn std::error::Error>> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Content-Type",
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&format!("Bearer {}", secret_token))?,
    );
    Ok(headers)
}

// print_chat_stream
//
// APIから下記のようなデータが連続して送られてくるので、`choices[0].delta.content`を取得して逐次Printする。
//
// 以下のようなデータが送られてくる:
// ```
// data: {"id":"chatcmpl-6qPcoOfhzpOdqX9iqRazRUQyjQ2fm","object":"chat.completion.chunk","created":1677949578,"model":"gpt-3.5-turbo-0301","choices":[{"delta":{"role":"assistant"},"index":0,"finish_reason":null}]}
//
// data: {"id":"chatcmpl-6qPcoOfhzpOdqX9iqRazRUQyjQ2fm","object":"chat.completion.chunk","created":1677949578,"model":"gpt-3.5-turbo-0301","choices":[{"delta":{"content":"され"},"index":0,"finish_reason":null}]}
//
// data: {"id":"chatcmpl-6qPcoOfhzpOdqX9iqRazRUQyjQ2fm","object":"chat.completion.chunk","created":1677949578,"model":"gpt-3.5-turbo-0301","choices":[{"delta":{"content":"ます"},"index":0,"finish_reason":null}]}
//
// data: {"id":"chatcmpl-6qPcoOfhzpOdqX9iqRazRUQyjQ2fm","object":"chat.completion.chunk","created":1677949578,"model":"gpt-3.5-turbo-0301","choices":[{"delta":{"content":"。"},"index":0,"finish_reason":null}]}
//
// data: {"id":"chatcmpl-6qPcoOfhzpOdqX9iqRazRUQyjQ2fm","object":"chat.completion.chunk","created":1677949578,"model":"gpt-3.5-turbo-0301","choices":[{"delta":{},"index":0,"finish_reason":"stop"}]}
//
// data: [DONE]
// ```
//
// `data: [DONE]` が送られてきたら読み込みを終了し、ループを抜ける。
//
// 送られてきた `choices[0].delta.content` は `joined_string`に連結し、最後に返す。
fn print_chat_stream(
    response: reqwest::blocking::Response,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut joined_string = String::new();

    // レスポンスを読み込むためのリーダーを作成する
    let reader = BufReader::new(response);

    // APIからの回答が横に長い場合は、読みづらいので改行する。
    // ただ、適切な改行を行うためにはTerminalの幅を取得する必要があるため、
    // 現状はMAXを仮で設定し、実質的に途中の強制改行が発生しないようにしている。
    let mut line_length = 0;
    let max_line_length = MAX;

    // レスポンスの各行を処理する
    for line in reader.lines() {
        let line = line?;

        // ループを抜けるための条件
        if line == "data: [DONE]" {
            break;
        }

        // "data: "で始まる各行を処理する
        if line.starts_with("data: ") {
            let data = line[6..].trim();
            let chunk: ChatCompletionChunk = serde_json::from_str(data)?;

            // 選択肢の各要素を処理する
            for choice in chunk.choices {
                if let Some(content) = choice.delta.content {
                    // 逐次Printする
                    print!("{}", content);

                    // 逐次連結する
                    joined_string.push_str(&content);

                    // 改行コードが含まれている場合は文字数をリセットする
                    if content.contains('\n') {
                        line_length = 0;
                    }

                    // 文字数をプラスする。
                    // UTF-8文字としてカウントしたいので、chars().count()を使う。
                    // https://doc.rust-lang.org/std/string/struct.String.html#utf-8
                    line_length += content.chars().count();

                    // 100文字に達した場合は改行する
                    if line_length >= max_line_length {
                        println!();
                        line_length = 0;
                    }

                    // 逐次Printしたものを即座に表示する
                    stdout().flush().unwrap();
                }
            }
        }
    }

    Ok(joined_string)
}
