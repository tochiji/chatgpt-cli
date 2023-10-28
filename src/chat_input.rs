use rustyline::{error::ReadlineError, DefaultEditor};

// 標準入力から複数行の文字列を読み込む関数
pub fn stdin_to_string() -> Result<String, Box<dyn std::error::Error>> {
    let mut rl = DefaultEditor::new()?; // rustylineのインスタンスを作成

    let mut buffer = String::new(); // 読み込んだ文字列を格納するためのバッファを作成
    loop {
        let readline = rl.readline(""); // プロンプトを表示せずにユーザーからの入力を待つ
        match readline {
            Ok(line) => {
                buffer.push_str(&line); // 入力された行をバッファに追加
                buffer.push('\n'); // 改行をバッファに追加
            }
            Err(ReadlineError::Interrupted) => {
                std::process::exit(0); // Ctrl+Cで終了
            }
            Err(ReadlineError::Eof) => {
                break; // Ctrl+Dで入力を終える
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(buffer) // 読み込みに成功した場合は、バッファをOkで返す。
}
