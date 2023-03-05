# chatgpt-cli

ChatGPTのChat APIを呼び出します。

https://platform.openai.com/docs/api-reference/chat/create

```console
$ chatgpt-cli 

👤 質問を入力してください。（入力完了時は改行してCtrl+D）>
あなたは誰ですか？
^D

🤖 ChatGPTからの回答 >
私は人工知能のアシスタントです。どのようにお手伝いできますか？

👤 質問を入力してください。（入力完了時は改行してCtrl+D）>

```

対話型でChatGPTと会話ができます。


## Installation

まずこのリポジトリをクローンし、プロジェクトルートに移動してください。`cargo install --path path/to/chatgpt-cli` でインストールできます。

```console
$ cargo install --path path/to/chatgpt-cli
```

## OpenAI API Keyのセット
環境変数 `OPENAI_API_KEY` にOpenAIのAPI Keyをセットしてください。

```console
export OPENAI_API_KEY=sk-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

