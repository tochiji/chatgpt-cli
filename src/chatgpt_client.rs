use std::{
    io::{stdout, BufRead, BufReader, Write},
    usize::MAX,
};

use crate::{
    api_response::{ChatCompletionChunk, Models},
    chat_message,
};
use requestty::Question;
use reqwest::blocking::Client;
use serde_json::json;

pub struct ChatGPTClient {
    openai_token: String,
    model: Option<String>,
    client: Client,
}

impl ChatGPTClient {
    pub fn new(openai_token: String) -> Self {
        Self {
            openai_token,
            model: None,
            client: Client::new(),
        }
    }

    pub fn select_model(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let url = "https://api.openai.com/v1/models";
        let headers = self.generate_headers()?;
        let response = self.get_request(url, headers)?;

        let models: Models = response.json()?;
        let gpts = models.get_gpts();

        let select = Question::select("theme")
            .should_loop(false)
            .message("🤖 ChatGPTのモデルを選択してください (Ctrl+c to exit)")
            .choices(gpts)
            .default(0)
            .build();

        let answer = requestty::prompt_one(select)?;
        let model = &answer.as_list_item().unwrap().text;
        self.model = Some(model.to_owned());

        Ok(())
    }

    pub fn send_messages(
        &self,
        message_history: &chat_message::MessageHistory,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = "https://api.openai.com/v1/chat/completions";
        let headers = self.generate_headers()?;
        let body = self.generate_body_from_history(message_history);
        let response = self.send_post_request(url, headers, body)?;
        let joined_string = self.print_chat_stream(response)?;
        println!();
        Ok(joined_string)
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
            "top_p": 0.5,
            "stream": true,
            "model": model,
            "messages": messages,
        })
    }

    fn get_request(
        &self,
        url: &str,
        headers: reqwest::header::HeaderMap,
    ) -> Result<reqwest::blocking::Response, Box<dyn std::error::Error>> {
        let res = self.client.get(url).headers(headers).send()?;

        Ok(res)
    }

    fn send_post_request(
        &self,
        url: &str,
        headers: reqwest::header::HeaderMap,
        body: serde_json::Value,
    ) -> Result<reqwest::blocking::Response, Box<dyn std::error::Error>> {
        let res = self
            .client
            .post(url)
            .headers(headers)
            .json(&body)
            .send()?
            .error_for_status()?;

        Ok(res)
    }

    // APIを呼び出すのに必要なヘッダーを生成する
    fn generate_headers(&self) -> Result<reqwest::header::HeaderMap, Box<dyn std::error::Error>> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            "Authorization",
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", self.openai_token))?,
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
        &self,
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
            if let Some(data) = line.strip_prefix("data: ") {
                let chunk: ChatCompletionChunk = serde_json::from_str(data.trim())?;

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
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn select_model() {
        let openai_token = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
        let mut client = ChatGPTClient::new(openai_token);

        let result = client.select_model();
        assert!(result.is_ok());
    }
}
