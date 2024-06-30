# Docker

## イメージのビルド

`docker-compose build`

## コンテナに入る

`docker-compose run --rm rust-app /bin/bash`

# Cargo

## ビルド

`cargo build`
`cargo build --release`

## ビルド＆実行

`cargo run`
`cargo run --release`

## テスト

`cargo test`
`cargo test --release`

## コンパイルチェック（リンクしないので高速）

`cargo check`

## ビルドアーティファクトの削除

`cargo clean`

## ドキュメント生成

`cargo doc`
`cargo doc --open`

## フォーマッティング

`cargo fmt`

## リント（潜在的なバグやスタイル違反の指摘）

`cargo clippy`

## 依存関係を最新版に更新する

`cargo update`

# リリース

```
git tag -a vx.x.x -m 'hoge'
git push origin vx.x.x
```
