# chatgpt-cli

ChatGPTのChat APIを呼び出します。

https://platform.openai.com/docs/api-reference/chat/create

```console
$ chatgpt-cli 

? 🤖 ChatGPTのモデルを選択してください (Ctrl+c to exit) › 
❯ gpt-3.5-turbo-1106
  gpt-4-1106-preview
  gpt-4-vision-preview
  gpt-3.5-turbo-instruct-0914
  gpt-3.5-turbo-instruct
  gpt-4
  gpt-4-0314
  gpt-4-0613
  gpt-3.5-turbo-0613
  gpt-3.5-turbo-16k-0613
  gpt-3.5-turbo-16k
  gpt-3.5-turbo-0301
  gpt-3.5-turbo

👤 質問を入力してください。（入力完了時は改行してCtrl+D）>
あなたは誰ですか？

🤖 ChatGPTからの回答 >
私は人工知能のアシスタントです。私の目的は、さまざまな質問に答えたり、情報を提供したり、説明を行ったりすることで、あなたを支援することです。私はプログラムされたアルゴリズムとデータに基づいて動作し、学習能力を持っているため、時間とともに改善されます。私は感情や自己意識を持たないため、あなたが必要とする情報を提供することに集中しています。

👤 質問を入力してください。（入力完了時は改行してCtrl+D）>

```

対話型でChatGPTと会話ができます。


## Installation

まずこのリポジトリをクローンし、プロジェクトルートに移動してください。`cargo install --path .` でインストールできます。

```console
$ cargo install --path .
```

## OpenAI API Keyのセット
環境変数 `OPENAI_API_KEY` にOpenAIのAPI Keyをセットした上で、`chatgpt-cli` を実行してください。

```console
export OPENAI_API_KEY=sk-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

