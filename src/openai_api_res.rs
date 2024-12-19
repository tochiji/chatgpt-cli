use serde::Deserialize;

///
/// ChatGPTのmodel一覧を取得するAPIのレスポンス
///
/// ```json
/// // sample response
/// {
///   "object": "list",
///   "data": [
///     {
///       "id": "text-search-babbage-doc-001",
///       "object": "model",
///       "created": 1651172509,
///       "owned_by": "openai-dev"
///     },
///     {
///       "id": "gpt-3.5-turbo-16k-0613",
///       "object": "model",
///       "created": 1685474247,
///       "owned_by": "openai"
///     },
///    ...
///  ]
/// }
/// ```
///
#[derive(Debug, Deserialize)]
pub struct Models {
    pub object: String,
    pub data: Vec<Model>,
}

impl Models {
    pub fn get_gpts(&self) -> Vec<String> {
        // gpt-から始まるか、oから始まるか
        let mut filterd: Vec<&Model> = self
            .data
            .iter()
            .filter(|m| m.id.contains("gpt-") || m.id.contains("o"))
            .collect();
        filterd.sort_by_key(|m| m.created);
        filterd.reverse();
        filterd.iter().map(|m| m.id.clone()).collect()
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Model {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub owned_by: String,
}

// ストリーミングでは、下記のようなデータが返ってくる
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
pub struct ChatCompletionStreamChunk {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<StreamChoice>,
}

#[derive(Debug, Deserialize)]
pub struct StreamChoice {
    pub delta: Delta,
    pub index: u64,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Delta {
    pub content: Option<String>,
    pub role: Option<String>,
}

// 非ストリーミングでは、下記のようなデータが返ってくる
//
// ```json
// {
//   "id": "chatcmpl-6qPcoOfhzpOdqX9iqRazRUQyjQ2fm",
//   "object": "chat.completion",
//   "created": 1677949578,
//   "model": "gpt-3.5-turbo-0301",
//   "choices": [{"message":{"role":"assistant","content":"されます"},"finish_reason":"stop","index":0}]
// }
// ```
#[derive(Debug, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub index: u64,
    pub message: Message,
    pub finish_reason: String,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}
