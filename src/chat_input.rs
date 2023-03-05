use std::io::Read;

// 標準入力から文字列を読み込む関数
// この関数が実行されると、Ctrl+Dが入力されるまでメインスレッドがストップする。
pub fn stdin_to_string() -> Result<String, Box<dyn std::error::Error>> {
    let stdin = std::io::stdin(); // 標準入力を取得
    let mut reader = stdin.lock(); // 標準入力をロックして、読み込み用のreaderを作成
    let mut buffer = String::new(); // 読み込んだ文字列を格納するためのバッファを作成
    reader.read_to_string(&mut buffer)?; // readerから文字列を読み込み、バッファに格納する。エラーが発生した場合は即座に関数から抜ける。
    Ok(buffer) // 読み込みに成功した場合は、バッファをOkで返す。
}
