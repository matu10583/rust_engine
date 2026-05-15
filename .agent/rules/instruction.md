---
trigger: always_on
---

# rust_engine Codex Instructions

このリポジトリは、学習優先の Rust 製 2D エンジン基盤です。
Codex は実装を急がず、ユーザーが次から自分で設計・実装できるように、判断理由を明確にしながら支援してください。

## 目的

- 最優先は学習。ユーザーを単独で設計・実装できる状態に近づける。
- 2D 先行で、将来的に 3D や MMORPG 制作に拡張できるエンジン基盤を目指す。
- プロダクション品質より、理解しやすい設計・小さな実装・検証可能性を優先する。
- 明確な依頼がない限り、巨大な自動生成や大規模スキャフォールドは避ける。

## 開発の進め方

- 回答・説明・レビューは日本語で行う。
- 小さい修正は Codex が実装してよい。実装後は差分の意図と検証結果を短く説明する。
- 大きな機能追加では、先に設計案・公開インターフェイス・変更範囲・テスト方針をユーザーと合意する。
- 公開 API、trait、主要 struct、モジュール境界を変える場合は、原則として実装前に相談する。
- インターフェイス設計は丁寧に進める。ユーザーが手動で書く意向がある場合は、Codex はレビュー・補助・テスト追加に回る。
- 既存の構成に沿った最小変更を優先し、不要な抽象化や先回りした汎用化を避ける。
- 仕様が曖昧なときは、選択肢とトレードオフを示してから小さく進める。

## 検証

標準の検証は host の Rust toolchain を使う。

```sh
make check
make test
make clippy
make fmt-check
```

- Codex からは Docker socket が使えない環境があるため、通常は `docker-*` ターゲットではなく host cargo 経由の `make` ターゲットを使う。
- Docker / Dev Container が必要な検証は、人間の端末で実行するか、明示的に相談する。
- 変更のリスクに応じてテストを追加する。小さい修正では対象テスト、共有基盤の変更では `make test` まで確認する。
- `make clippy` は品質ゲートとして扱う。既存の広範な指摘が出た場合は、目的の変更と分離して扱う。

## GitHub / PR 運用

- 差分確認とレビューは GitHub PR を主な UI として使う。
- Codex はローカル実装、テスト、コミット、push、PR 作成、PR コメント確認、レビュー対応を担当できる。
- PR レビューコメントを受けたら、`gh api` などでコメント本文・対象ファイル・対象行を確認し、該当コードを読んで修正する。
- GitHub API や `gh` はネットワーク権限が必要な場合がある。必要なときだけ実行し、結果をユーザーに要約する。
- PR への追加修正は小さなコミットに分ける。レビュー対応の理由をコミットメッセージや説明に残す。

## アーキテクチャ方針

- ECS: `hecs` は `core::ecs` の中に隠蔽する。
  - 公開 API に `hecs` の型を漏らさない。
  - 公開する安定名は `World`, `Entity`, `Component`, `query_ref`, `query_mut` を中心にする。
- DI: `DiContainer` は `TypeId -> Box<dyn Any + Send + Sync>` の型マップとして扱う。
  - `Time`, 入力, Render 文脈, Events キューなどの共有リソースを格納する。
- Events<T>: ダブルバッファのイベントキュー。
  - `send`, `drain`, `update` の意味を崩さない。
  - 原則として `LateUpdate` で `update()` し、次フレームで読めるようにする。
- Time / TimeState: delta と elapsed を管理し、各フレーム `TimeState::tick` で更新する。
- Schedule / Stage / System / Plugin / App:
  - `System` は `fn(&mut DiContainer, &mut ecs::World)` の関数ポインタ前提。
  - クロージャ対応や状態付き system は、必要になってから設計する。
  - Stage は `Startup`, `ProcessInput`, `Update`, `FixedUpdate`, `PreRender`, `Render`, `LateUpdate` を使う。
  - Schedule はステージごとに優先度バケットを持つ。
  - Plugin は `fn build(&self, app: &mut App)` で機能を登録する。
- Rendering: まずは `NullRenderer` と 2D コマンド収集を小さく育てる。実 renderer は設計を相談してから進める。

## コーディングスタイル

- Rust 2021+ の慣用表現を使う。
- ライブラリ内は `crate::`、テスト・examples からは `rust_engine::` を使う。
- `lib.rs` で外向き API を整理し、公開範囲を必要最小限にする。
- 関数は短く単一責務にする。
- POD 的な型には必要に応じて `Debug`, `Clone`, `Copy`, `Default`, `PartialEq` などを付与する。
- `new()` を提供する型は、自然に空/初期状態を作れるなら `Default` も実装する。
- テストは外部のローカル設定や未追跡 asset に依存させない。必要な config や fixture はテスト内で作る。
- コメントは、処理の説明ではなく設計意図・注意点・借用スコープなどを補うときにだけ書く。

## 依存追加

- ユーザー要求がない限り、async、スレッド、重量依存、巨大なフレームワークは導入しない。
- 新しい crate を追加する場合は、用途、代替案、学習コスト、将来の置き換えやすさを説明してから進める。

## レビュー観点

- バグ、回帰リスク、公開 API への影響、テスト不足を優先して見る。
- 「動く」だけでなく、次にユーザーが読んで直せる構造かを確認する。
- 設計が重くなっている場合は、より小さい段階的な案を提示する。
