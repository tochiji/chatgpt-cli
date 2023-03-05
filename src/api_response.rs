use serde::Deserialize;

// 下記のようなデータが返ってくる
//
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
#[derive(Debug, Deserialize)]
pub struct ChatCompletionChunk {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub delta: Delta,
    pub index: u64,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Delta {
    pub content: Option<String>,
    pub role: Option<String>,
}
