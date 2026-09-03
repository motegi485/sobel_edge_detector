mod edge_detection;

use std::env;
use std::error::Error;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::Path;
use std::process::ExitCode;

/// 出力ディレクトリを省略した場合の既定値
const DEFAULT_OUTPUT_DIR: &str = "processed_images";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.iter().any(|arg| arg == "-h" || arg == "--help") {
        print_usage();
        return ExitCode::SUCCESS;
    }

    if args.is_empty() {
        eprintln!("エラー: 入力画像パスが指定されていません");
        print_usage();
        return ExitCode::FAILURE;
    }

    if args.len() > 2 {
        eprintln!("エラー: 引数が多すぎます（指定できるのは最大2つです）");
        print_usage();
        return ExitCode::FAILURE;
    }

    let input_path = args[0].as_str();
    let output_dir = Path::new(args.get(1).map_or(DEFAULT_OUTPUT_DIR, String::as_str));

    match run(input_path, output_dir) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("エラー: {err}");
            ExitCode::FAILURE
        }
    }
}

/// グレースケール化とエッジ検出を実行し、結果を出力ディレクトリに保存する
fn run(input_path: &str, output_dir: &Path) -> Result<(), Box<dyn Error>> {
    let input = Path::new(input_path);
    if !input.is_file() {
        return Err(format!("入力画像が見つかりません: {input_path}").into());
    }

    // 出力ファイル名の接頭辞として、入力ファイルの拡張子を除いた名前を使う
    let stem = input
        .file_stem()
        .ok_or_else(|| format!("入力ファイル名を取得できません: {input_path}"))?;

    fs::create_dir_all(output_dir).map_err(|err| {
        format!(
            "出力ディレクトリの作成に失敗しました: {}: {err}",
            output_dir.display()
        )
    })?;

    let grayscale_img = edge_detection::convert_to_grayscale(input_path)
        .map_err(|err| format!("画像の読み込みに失敗しました: {input_path}: {err}"))?;
    let grayscale_path = output_dir.join(output_file_name(stem, "grayscale"));
    grayscale_img.save(&grayscale_path).map_err(|err| {
        format!(
            "画像の保存に失敗しました: {}: {err}",
            grayscale_path.display()
        )
    })?;
    println!("保存しました: {}", grayscale_path.display());

    let edge_img = edge_detection::apply_sobel(grayscale_img);
    let edge_path = output_dir.join(output_file_name(stem, "edge"));
    edge_img
        .save(&edge_path)
        .map_err(|err| format!("画像の保存に失敗しました: {}: {err}", edge_path.display()))?;
    println!("保存しました: {}", edge_path.display());

    Ok(())
}

/// `<入力ファイル名>_<suffix>.png` 形式の出力ファイル名を組み立てる
fn output_file_name(stem: &OsStr, suffix: &str) -> OsString {
    let mut name = stem.to_os_string();
    name.push("_");
    name.push(suffix);
    name.push(".png");
    name
}

/// 使い方を標準出力に表示する
fn print_usage() {
    println!(
        "\
使い方:
    sobel_edge_detector <入力画像パス> [出力ディレクトリ]

引数:
    <入力画像パス>      エッジ検出を行う画像のパス
    [出力ディレクトリ]  結果の保存先（省略時: {DEFAULT_OUTPUT_DIR}）

オプション:
    -h, --help          このヘルプを表示する

出力ファイル:
    <出力ディレクトリ>/<入力ファイル名>_grayscale.png  グレースケール化した中間結果
    <出力ディレクトリ>/<入力ファイル名>_edge.png       エッジ検出の最終結果"
    );
}
