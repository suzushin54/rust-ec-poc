# リポジトリ作成から初期構築までの作業ログ

1. プロジェクトの初期化。Cargoを使って新規プロジェクトの作成

```shell
λ cargo init --bin
    Creating binary (application) package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

2. 必要な依存関係を `Cargo.toml` に追加

```Cargo.toml
[dependencies]
axum = "0.7.9"
tokio = { version = "1", features = ["full"] }
hyper = "1.5.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"
aws-sdk-s3 = "1.65.0"
```

3. 追加したクレートのバージョンが古ければ更新する

```shell
λ cargo update
```

4. 簡易サーバを立ち上げるだけのサンプルコードを追加

5. クレートのページを参考に、フルの機能を利用するための追加

```shell
cargo add tokio --features macros,rt-multi-thread
```

6. ビルド、起動

```shell
λ cargo build && cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/rust-ec-poc`
```

```shell
λ curl -v 127.0.0.1:8080
*   Trying 127.0.0.1:8080...
* Connected to 127.0.0.1 (127.0.0.1) port 8080
> GET / HTTP/1.1
> Host: 127.0.0.1:8080
> User-Agent: curl/8.7.1
> Accept: */*
>
* Request completely sent off
< HTTP/1.1 200 OK
< content-type: text/plain; charset=utf-8
< content-length: 13
< date: Sun, 15 Dec 2024 09:52:21 GMT
<
* Connection #0 to host 127.0.0.1 left intact
Hello, World!⏎       
```





