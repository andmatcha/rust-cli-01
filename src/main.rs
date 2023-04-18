#![allow(unused)]

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    // コマンドラインの引数を構文解析
    // &args.patternのようにstructで定義した名前で引数の入力値にアクセスできる
    let args = Cli::parse();
    // コマンドラインの"path"引数からファイルを読み込む
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    // contentを1行ずつ読む
    for line in content.lines() {
        // 行にコマンドラインの"pattern"引数で指定した文字列が含まれていたら、コマンドラインに出力する
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
