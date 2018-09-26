// std::env::argsでコマンドライン引数を受け取れる
// 希望の関数が2モジュール以上ネストされている場合、関数ではなく親モジュールをスコープに導入するのが因習的
// use std::env::args は現在のモジュールに定義されている関数と容易に見間違えられるかもしれないから
use std::env;

fn main() {
    // collect は型注釈が必要
    // let(変数宣言) 変数名 型注釈
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
}
