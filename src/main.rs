// ライブラリクレート(lib.rs)をバイナリクレート(main.rs)に持っていく
extern crate minigrep;

// args関数を使用できる
use std::env;
// process::exitを使用できる。code: 1
use std::process;

use minigrep::Config;

// ロジックをsrc/lib.rsに抜き出し、引数集めとエラー処理をsrc/main.rsに残した
// テスト駆動開発(TDD)過程
// もう必要ないので、プログラムの振る舞いを確認していたprintln!文をsrc/lib.rsとsrc/main.rsから削除
fn main() {
    let args: Vec<String> = env::args().collect();
    
    // unwrap_or_elseでは、Okの場合その中身を返し、Errの場合その中身をクロージャで扱える？？
    // opとか&strは保管された
    let config = Config::new(&args).unwrap_or_else(|err| {
        // 引数解析時に問題
        // eprintlnにすることで標準出力から標準エラー出力になり、> output.txtしても画面に表示されるようになる
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);
    // エラー型については、トレイトオブジェクトのBox<dyn Error>を使用
    // 関数がErrorトレイトを実装する型を返すことを意味しますが、 戻り値の型を具体的に指定しなくても良い
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}