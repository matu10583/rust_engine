# Development

このリポジトリは host の Rust toolchain で開発できます。
Docker Compose / Dev Container も必要に応じて使えます。

## 初回セットアップ

```sh
make fetch
```

## よく使う検証

```sh
make check
make test
make clippy
make fmt-check
```

コード整形は次で実行します。

```sh
make fmt
```

## Dev Container

VS Code や Dev Containers CLI を使う場合は `.devcontainer/devcontainer.json` を開いてください。
作成後に `cargo fetch` が実行され、`rust-analyzer` と `CodeLLDB` を使える設定です。

## Docker Compose

Docker で検証したい場合は `docker-*` ターゲットを使います。

```sh
docker compose build
make docker-test
make docker-clippy
```

コンテナ内の作業ディレクトリは `/workspace` です。

```sh
make docker-shell
```

## Codex

Codex の sandbox から host の `cargo` が見えている場合は、通常の `make check` / `make test` / `make clippy` を使ってください。
Docker socket が sandbox から使えない環境では `docker-*` ターゲットは人間の端末から実行します。
