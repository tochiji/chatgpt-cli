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
        let mut filterd: Vec<&Model> = self.data.iter().filter(|m| m.id.contains("gpt-")).collect();
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
