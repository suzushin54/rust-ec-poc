## memo

- なぜ main 関数に非同期ランタイムの Tokio のマクロ `#[tokio::main]` があるのか
  - 非同期プログラムのエントリポイントを制御し、ランタイムの初期化と管理を集約するため
  - 標準ライブラリに非同期ランタイムを含まないため、 main を非同期関数とするためには、手動で初期化して実行する必要がある


- `failed to build proc-macrorust-analyzermacro-error` が表示されるようになった
  - バージョン、依存関係、crateの問題などで `rust-analyzer` が出してくる。 `cargo build` するとエラーが出るかも

e.g. 

```shell
λ cargo build
error: rustc 1.80.0 is not supported by the following package:
  aws-sdk-s3@1.66.0 requires rustc 1.81.0
Either upgrade rustc or select compatible dependency versions with
`cargo update <name>@<current-ver> --precise <compatible-ver>`
where `<compatible-ver>` is the latest version supporting rustc 1.80.0
```

