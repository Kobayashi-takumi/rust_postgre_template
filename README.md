# 概要

Rust + SQLx + PostgreSQL サンプル
ドメイン駆動 + unit of work + CQRS のパターンで実装

# 環境

1. make をインストール(既にインストール済みならスキップ)
2. make build
3. make up

# コマンド

| 内容                  | コマンド                             |
| --------------------- | ------------------------------------ |
| コンテナ ビルド       | make build                           |
| コンテナ 起動         | make up                              |
| コンテナ 破棄         | make down                            |
| ログ                  | make log                             |
| コマンド実行          | make sh ARGS="<実行したいコマンド>"  |
| bash                  | make init                            |
| DB クライアント       | make client                          |
| マイグレーション 追加 | make migrate-add ARGS="<ファイル名>" |
| マイグレーション 実行 | make migrate-run                     |
