// 環境変数を扱う
use std::env;

// std::fs::File ファイルを扱うのに必要
// std::io::prelude::* ファイル入出力を含む入出力処理をする
use std::fs::File;
use std::io::prelude::*;

// エラー型を扱うError トレイト
use std::error::Error;

// 設定値
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // 成功時には Config インスタンスを含み、エラー時には問題に言及する Result 値を返す
    // &'static str エラー文
    // 'static ライフタイム注釈(staticは特別 config的な扱い)
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // clone は参照を保持するよりも時間とメモリを消費するが、参照のライフタイムを管理する必要がなくなる
        let query = args[1].clone();
        let filename = args[2].clone();

        // 環境変数の確認
        // is_err() 値がセットされていればfalseを返す
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// Box<T> トレイトを返り値で返したいとき使う ポインタで扱う Tで扱う型を指定
// Error 様々なError型を包括するError型
pub fn run(config: Config) -> Result<(), Box<Error>> {
    // ファイルへの可変なハンドルを得る
    // ? 演算子は呼び出し元が処理できるように、現在の関数からエラー値を返す
    let mut f = File::open(config.filename)?;
    // ファイル読み込み後に中身を保持するため、可変で空の String を生成
    let mut contents = String::new();
    // ファイルを読み込む
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    // Ok(()) という記法の、() を使うのは、run を副作用のためだけに呼び出していると示唆する慣習的な方法
    Ok(())
}

// クエリ文字列の検索を行う 大文字小文字を区別する
// 2引数でライフタイムが異なるのでライブタイム注釈が必要　その時のライフタイムはを参照する文字列スライスの方
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // 検索結果を保持するベクタ
    let mut results = Vec::new();

    // 各行ごとに探索処理 lines()
    for line in contents.lines() {
        // 文字列中探索 contains()
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

// クエリ文字列の検索を行う 大文字小文字を区別しない
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // クエリ、探索文字列を小文字にすることで、 大文字小文字を区別しない
    // to_lowercase() Stringを返す
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        // contains() は文字列スライス取るため、参照を渡す
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
