// ライブラリクレートをバイナリクレートで扱う
extern crate minigrep;

// std::env::argsでコマンドライン引数を受け取れる
// 希望の関数が2モジュール以上ネストされている場合、関数ではなく親モジュールをスコープに導入するのが因習的
// use std::env::args は現在のモジュールに定義されている関数と容易に見間違えられるかもしれないから
use std::env;

// プロセスを扱う
use std::process;

// Config型を扱う
use minigrep::Config;

fn main() {
    // collect は型注釈が必要
    // let(変数宣言) 変数名 型注釈
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // 引数解析
    // unwrap_or_else panic!ではない何らか独自のエラー処理を定義できる
    let config = Config::new(&args).unwrap_or_else(|err| {
        // 引数解析時に問題
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // ライブラリクレート側のためクレート名をつけないと読み込めない
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

