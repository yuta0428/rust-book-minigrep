// std::env::argsでコマンドライン引数を受け取れる
// 希望の関数が2モジュール以上ネストされている場合、関数ではなく親モジュールをスコープに導入するのが因習的
// use std::env::args は現在のモジュールに定義されている関数と容易に見間違えられるかもしれないから
use std::env;

// std::fs::File ファイルを扱うのに必要
// std::io::prelude::* ファイル入出力を含む入出力処理をする
use std::fs::File;
use std::io::prelude::*;

// プロセスを扱う
use std::process;

// エラー型を扱うError トレイト
use std::error::Error;

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

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

// Box<T> トレイトを返り値で返したいとき使う ポインタで扱う Tで扱う型を指定
// Error 様々なError型を包括するError型
fn run(config: Config) -> Result<(), Box<Error>> {
    // ファイルへの可変なハンドルを得る
    // ? 演算子は呼び出し元が処理できるように、現在の関数からエラー値を返す
    let mut f = File::open(config.filename)?;
    // ファイル読み込み後に中身を保持するため、可変で空の String を生成
    let mut contents = String::new();
    // ファイルを読み込む
    f.read_to_string(&mut contents)?;
    println!("With text:\n{}", contents);

    // Ok(()) という記法の、() を使うのは、run を副作用のためだけに呼び出していると示唆する慣習的な方法
    Ok(())
}

// 設定値
struct Config {
    query: String,
    filename: String,
}

impl Config {
    // 成功時には Config インスタンスを含み、エラー時には問題に言及する Result 値を返す
    // &'static str エラー文
    // 'static ライフタイム注釈(staticは特別 config的な扱い)
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // clone は参照を保持するよりも時間とメモリを消費するが、参照のライフタイムを管理する必要がなくなる
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}
