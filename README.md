# aichat-cli

ChatGPT/ClaudeのChat APIを呼び出します。

https://platform.openai.com/docs/api-reference/chat/create

```console
$ aichat-cli 

? 🤖 利用するモデルを選択してください (Ctrl+c to exit) › 
❯ claude-3-opus-20240229
  claude-3-sonnet-20240229
  claude-3-haiku-20240307
  gpt-4-turbo-2024-04-09
  gpt-4-turbo
  gpt-4-1106-vision-preview
  gpt-3.5-turbo-0125
  gpt-4-turbo-preview
  gpt-4-0125-preview
  gpt-3.5-turbo-1106
  gpt-4-1106-preview
  gpt-4-vision-preview
  gpt-3.5-turbo-instruct-0914
  gpt-3.5-turbo-instruct

👤 質問を入力してください。（入力完了時は改行してCtrl+D）>
あなたは誰ですか？

🤖 ChatGPTからの回答 >
私は人工知能のアシスタントです。私の目的は、さまざまな質問に答えたり、情報を提供したり、説明を行ったりすることで、あなたを支援することです。私はプログラムされたアルゴリズムとデータに基づいて動作し、学習能力を持っているため、時間とともに改善されます。私は感情や自己意識を持たないため、あなたが必要とする情報を提供することに集中しています。

👤 質問を入力してください。（入力完了時は改行してCtrl+D）>

```

対話型でAIと会話ができます。


## Installation

まずこのリポジトリをクローンし、プロジェクトルートに移動してください。`cargo install --path .` でインストールできます。

```console
$ cargo install --path .
```

## OpenAI API Keyのセット
環境変数 `OPENAI_API_KEY` と `ANTHROPIC_API_KEY` セットした上で、`chatgpt-cli` を実行してください。

```console
export OPENAI_API_KEY=sk-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
export ANTHROPIC_API_KEY=sk-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

