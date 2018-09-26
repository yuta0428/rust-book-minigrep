// std::env::argsでコマンドライン引数を受け取れる
// 希望の関数が2モジュール以上ネストされている場合、関数ではなく親モジュールをスコープに導入するのが因習的
// use std::env::args は現在のモジュールに定義されている関数と容易に見間違えられるかもしれないから
use std::env;

// std::fs::File ファイルを扱うのに必要
// std::io::prelude::* ファイル入出力を含む入出力処理をする
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // collect は型注釈が必要
    // let(変数宣言) 変数名 型注釈
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // 参照を渡す
    let query = &args[1];
    let filename = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", filename);

    // ファイルへの可変なハンドルを得る
    // 失敗した場合はfile not foundを出力
    let mut f = File::open(filename).expect("file not found");
    // ファイル読み込み後に中身を保持するため、可変で空の String を生成
    let mut contents = String::new();
    // ファイルを読み込む
    // 失敗した場合はsomething went wrong reading the fileを出力
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    println!("With text:\n{}", contents);
}
