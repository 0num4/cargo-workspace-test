# cargo workspace の playground と learn-wgpu の試し打ち

https://claude.ai/chat/eb7d10b5-ed2d-42e4-a612-a78370fa004e

わかりました。Rust の Cargo ワークスペースを使って **learning-wgpu** と wasmsample2 という 2 つのプロジェクトを作成する手順を説明します。以下の手順に従ってプロジェクトを設定しましょう。

1. まず、新しいディレクトリを作成し、そこに移動します:

```bash
mkdir rust-wgpu-workspace
cd rust-wgpu-workspace
```

2. このディレクトリに `Cargo.toml` ファイルを作成し、ワークスペースを定義します:

```bash
touch Cargo.toml
```

3. `Cargo.toml` ファイルを編集し、以下の内容を追加します:

```toml
[workspace]
members = [
    "learning-wgpu",
    "wasmsample2"
]
```

4. 次に、各プロジェクトのディレクトリを作成します:

```bash
cargo new learning-wgpu
cargo new wasmsample2
```

5. 各プロジェクトの `Cargo.toml` ファイルを編集して、必要な依存関係を追加します。例えば、`learning-wgpu/Cargo.toml` に wgpu の依存関係を追加する場合:

```toml
[dependencies]
wgpu = "0.18"
```

6. 同様に、`wasmsample2/Cargo.toml` に必要な依存関係を追加します。

これで、基本的なワークスペースの構造が整いました。各プロジェクトのソースコードは、それぞれのディレクトリの `src` フォルダ内に配置されます。

プロジェクト全体をビルドする場合は、ルートディレクトリで以下のコマンドを実行します:

```bash
cargo build
```

特定のプロジェクトのみをビルドする場合は、以下のようにします:

```bash
cargo build -p learning-wgpu
```

または

```bash
cargo build -p wasmsample2
```

これで、Cargo ワークスペースを使用して **learning-wgpu** と wasmsample2 プロジェクトを作成できました。各プロジェクトの実装を進めることができます。

追加の情報や特定の実装に関する質問がありましたら、お知らせください。

## wgpu の勉強

https://qiita.com/ciscorn/items/c8c18fbcb60475cb4491

https://sotrh.github.io/learn-wgpu/
の写経

### wgpu について

https://crates.io/crates/wgpu
v22 系
