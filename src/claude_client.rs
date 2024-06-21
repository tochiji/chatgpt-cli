use std::{
    io::{stdout, BufRead, BufReader, Write},
    usize::MAX,
};

use anyhow::{anyhow, Result};

use reqwest::blocking::Client;
use serde_json::json;

use crate::{
    chat_input,
    chat_message::{self, Role},
    claude_api_res::ClaudeEvent,
    model::Campany,
    model::Model,
};

pub struct ClaudeClient {
    claude_token: String,
    model: Option<String>,
    client: Client,
}

impl ClaudeClient {
    pub fn new(claude_token: String) -> Self {
        Self {
            claude_token,
            model: None,
            client: Client::new(),
        }
    }

    pub fn set_model(&mut self, model: String) {
        self.model = Some(model);
    }

    pub fn get_model_list(&self) -> Vec<Model> {
        vec![
            Model {
                name: "claude-3-5-sonnet-20240620".to_string(),
                campany: Campany::Claude,
            },
            Model {
                name: "claude-3-opus-20240229".to_string(),
                campany: Campany::Claude,
            },
            Model {
                name: "claude-3-sonnet-20240229".to_string(),
                campany: Campany::Claude,
            },
            Model {
                name: "claude-3-haiku-20240307".to_string(),
                campany: Campany::Claude,
            },
        ]
    }

    pub fn run_claude(&self, mut messages: chat_message::MessageHistory) -> Result<()> {
        // ユーザーからの質問を無限ループで受け付ける
        loop {
            // ユーザーからの入力を受け付ける
            println!("👤 質問を入力してください。（入力完了時は改行してCtrl+D）>");
            let message = chat_input::stdin_to_string()?;

            // 入力した質問を履歴に追加
            messages.push(Role::User, &message);

            println!("🤖 Claudeからの回答 >");

            // [TODO] エラー時、exitするのではなく、エラー内容を表示してループを継続したい
            let assistant_response = self.send_messages(&messages)?;
            messages.push(Role::Assistant, &assistant_response);

            // 次の質問との間に空行を入れる
            println!();
        }
    }

    // APIへPOSTリクエストを送信する
    // https://docs.anthropic.com/claude/reference/messages_post
    fn send_messages(&self, message_history: &chat_message::MessageHistory) -> Result<String> {
        let url = "https://api.anthropic.com/v1/messages";
        let headers = self.generate_headers()?;
        let body = self.generate_body_from_history(message_history);
        let response = self.send_post_request(url, headers, body)?;
        let joined_string = self.print_chat_stream(response)?;
        println!();
        Ok(joined_string)
    }

    fn generate_headers(&self) -> Result<reqwest::header::HeaderMap> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "anthropic-version",
            reqwest::header::HeaderValue::from_static("2023-06-01"),
        );
        headers.insert(
            "content-type",
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            "x-api-key",
            reqwest::header::HeaderValue::from_str(&self.claude_token.to_string())?,
        );
        Ok(headers)
    }

    // APIへ送信するbodyを作成する。
    // メッセージ履歴は全て連結して送る必要がある。
    fn generate_body_from_history(
        &self,
        message_history: &chat_message::MessageHistory,
    ) -> serde_json::Value {
        let messages = message_history
            .messages
            .iter()
            .map(|m| json!({"role": m.role.to_string(), "content": m.content}))
            .collect::<Vec<_>>();

        let model = self.model.as_ref().unwrap();

        json!({
            "stream": true,
            "max_tokens": 4096,
            "model": model,
            "messages": messages,
        })
    }

    fn send_post_request(
        &self,
        url: &str,
        headers: reqwest::header::HeaderMap,
        body: serde_json::Value,
    ) -> Result<reqwest::blocking::Response> {
        let res = self.client.post(url).headers(headers).json(&body).send()?;

        if res.status().is_success() {
            Ok(res)
        } else {
            Err(anyhow!("failed to send POST request: {}", res.text()?))
        }
    }

    // print_chat_stream
    //
    // APIから下記のようなデータが連続して送られてくるので、`choices[0].delta.content`を取得して逐次Printする。
    //
    // 以下のようなデータが送られてくる:
    // ```
    // event: content_block_start
    // data: {"type":"content_block_start","index":0,"content_block":{"type":"text","text":""}}
    //
    // event: ping
    // data: {"type": "ping"}
    //
    // event: content_block_delta
    // data: {"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":"Hello"}}
    //
    // event: content_block_delta
    // data: {"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":"!"}}
    //
    // event: content_block_delta
    // data: {"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":" How"}}
    //
    // event: content_block_delta
    // data: {"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":" can"}}
    //
    // event: content_block_delta
    // data: {"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":" I"}}
    //
    // event: content_block_delta
    // data: {"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":" assist"}}
    //
    // event: content_block_delta
    // data: {"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":" you"}}
    //
    // event: content_block_stop
    // data: {"type":"content_block_stop","index":0}
    //
    // event: message_delta
    // data: {"type":"message_delta","delta":{"stop_reason":"end_turn","stop_sequence":null},"usage":{"output_tokens":12}}
    //
    // event: message_stop
    // data: {"type":"message_stop"}
    // ```
    //
    // `{"type":"message_stop"}` が送られてきたら読み込みを終了し、ループを抜ける。
    //
    // 送られてきた `choices[0].delta.content` は `joined_string`に連結し、最後に返す。
    fn print_chat_stream(&self, response: reqwest::blocking::Response) -> Result<String> {
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

            // "data: "で始まる各行を処理する
            if let Some(data) = line.strip_prefix("data: ") {
                let event: ClaudeEvent = serde_json::from_str(data.trim())?;

                // 選択肢の各要素を処理する
                match event {
                    ClaudeEvent::ContentBlockDelta { delta, .. } => {
                        let text = delta.text;
                        // 逐次Printする
                        print!("{}", text);

                        // 逐次連結する
                        joined_string.push_str(&text);

                        // 改行コードが含まれている場合は一度文字数をリセットする
                        // 文中に改行コードが含まれている場合は少しズレるが、気にしない
                        if text.contains('\n') {
                            line_length = 0;
                        }

                        // 文字数をプラスする。
                        // UTF-8文字としてカウントしたいので、chars().count()を使う。
                        // https://doc.rust-lang.org/std/string/struct.String.html#utf-8
                        line_length += text.chars().count();

                        // 100文字に達した場合は改行する
                        if line_length >= max_line_length {
                            println!();
                            line_length = 0;
                        }

                        // 逐次Printしたものを即座に表示する
                        stdout().flush().unwrap();
                    }
                    ClaudeEvent::MessageStop => {
                        break;
                    }
                    _ => {}
                }
            }
        }

        Ok(joined_string)
    }
}
