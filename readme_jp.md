# mtidy

mtidy (Media Tidier) は、SIGMA fpカメラで撮影されたCinemaDNG形式の動画ファイルを整理・管理するためのRust製コマンドラインツールです。SIGMA fpカメラの命名規則では、SSDをフォーマットするたびに同じフォルダ名（例: A001_001）が割り当てられるため、撮影日を特定することが難しく、同じ階層にフォルダを配置することができないという問題があります。mtidyは、動画ファイルをタイムスタンプに基づいて構造化されたフォルダ階層にコピーまたは移動することで、この問題を解決します。

## 特徴

- 指定された入力フォルダ内のCinemaDNG動画ファイルを再帰的に検索
- 動画ファイルをタイムスタンプ（YYYYMMDD_HHMMSS）に基づいてフォルダ構造に整理
- 動画ファイルとその親フォルダを出力フォルダにコピーまたは移動
- ファイルサイズとタイムスタンプを比較してファイルの競合を処理
- ファイルをコピーする代わりに移動するオプションを提供
- コピー、移動、スキップされたフォルダのログを出力

## 使用方法

```
mtidy <input_folder> <output_folder> [move]
```

- `<input_folder>`: 整理対象のCinemaDNG動画ファイルが含まれるフォルダのパス。
- `<output_folder>`: 整理された動画ファイルがコピーまたは移動されるフォルダのパス。
- `[move]`（オプション）: ファイルをコピーする代わりに移動する場合は "move" を指定。指定しない場合、デフォルトでファイルがコピーされます。

## 使用例

"input" フォルダから "output" フォルダにCinemaDNG動画ファイルをコピー:
```
mtidy /path/to/input /path/to/output
```

"input" フォルダから "output" フォルダにCinemaDNG動画ファイルを移動:
```
mtidy /path/to/input /path/to/output move
```

## インストール

1. システムにRustがインストールされていることを確認してください。インストールされていない場合は、[https://www.rust-lang.org/](https://www.rust-lang.org/)からインストールできます。

2. リポジトリをクローンします:
   ```
   git clone https://github.com/yourusername/mtidy.git
   ```

3. プロジェクトディレクトリに移動します:
   ```
   cd mtidy
   ```

4. プロジェクトをビルドします:
   ```
   cargo build --release
   ```

5. コンパイルされたバイナリは `target/release` ディレクトリにあります。システムのPATHにあるディレクトリに移動すると、アクセスが容易になります。

## 貢献

貢献は大歓迎です！問題を見つけた場合や改善のための提案がある場合は、GitHubリポジトリでissueを開くかpull requestを送信してください。

## ライセンス

このプロジェクトは[MITライセンス](LICENSE)の下でライセンスされています。